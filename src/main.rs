use std::{
    cell::{Cell, RefCell},
    ffi::OsString,
    fs::{metadata, read_dir, File},
    io::{Read, Write},
    path::Path,
    rc::Rc,
};

use clap::Parser;
use limit_stream::codegen::{format_idl, formatter::Formatter, idl2rust, rust::Rust};

#[derive(Parser, Debug)]
#[command(author, version, about = "
limitc is a compiler for limit_stream IDL files.
"
    , long_about = Some(
"
limitc is a compiler for limit_stream IDL files.
It can generate Rust, Go, Typescript and more source code,
also it is type driving by using session type for the classes defined in FILE.
"
    ), rename_all = "kebab_case")]
enum Limitsc {
    #[command(about = "generate code from IDL file")]
    CodeGen {
        #[arg(short, long)]
        lang: String,
        #[arg(short, long, default_value_t = String::from("client"), help = "client | server | all")]
        gen_mode: String,
        #[arg(short, long, default_value_t = String::from("."), help = "IDL path directory")]
        idl_path: String,
        #[arg(short, long, default_value_t = String::from(""), help = "output path directory")]
        out_path: String,
        // #[arg(short, long, help = "entry file")]
        // file: String,
    },
    #[command(about = "check IDL file session type")]
    TypeCheck {
        #[arg(short, long, default_value_t = String::from("."), help = "IDL path directory")]
        path: String,
        #[arg(short, long)]
        file: String,
    },
    #[command(about = "format IDL file")]
    Format {
        #[arg(short, long, default_value_t = 4, help = "indent size")]
        indent: usize,
        #[arg(short, long, default_value_t = String::from("."), help = "IDL path directory")]
        path: String,
    },
}

pub fn rust_codegen_file(mut rs: Rust, idl_path: &Path, out_path: &Path) -> std::io::Result<()> {
    let mut src = String::new();
    {
        let mut f = File::open(idl_path)?;
        f.read_to_string(&mut src)?;
    }
    let code = idl2rust(&src, &mut rs);
    let mut f = File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(out_path)?;

    let _ = f.write(code.as_bytes())?;
    Ok(())
}

fn format_file(mut fmt: Formatter, path: &Path) -> std::io::Result<()> {
    let mut src = String::new();
    {
        let mut f = File::open(path.clone())?;
        f.read_to_string(&mut src)?;
    }
    let formated_src = format_idl(&src, &mut fmt);
    let mut f = File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;
    let _ = f.write(formated_src.as_bytes())?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args = Limitsc::parse();
    match args {
        Limitsc::Format { indent, path } => {
            let fmt = Formatter {
                tab_size: indent,
                indent: 0,
            };
            let pathinfo = metadata(path.clone())?;
            if pathinfo.file_type().is_dir() {
                let dir = read_dir(path)?;
                for i in dir.flatten() {
                    if i.file_type()?.is_file()
                        && i.path().extension().expect("invalid extension name")
                            == Into::<OsString>::into("lstr".to_string())
                    {
                        format_file(fmt.clone(), i.path().as_path())?;
                    }
                }
            } else {
                format_file(fmt, Path::new(&path))?;
            }
        }
        Limitsc::CodeGen {
            lang,
            gen_mode,
            idl_path,
            out_path,
        } => {
            match lang.as_str() {
                "rust" => {
                    let rust = Rust {
                        tab_size: 2,
                        indent: 0,
                        enum_id: Rc::new(Cell::new(0)),
                        codegen_regester: Rc::new(RefCell::new(vec![])),
                    };
                    let pathinfo = metadata(idl_path.clone())?;
                    if pathinfo.file_type().is_dir() {
                        let dir = read_dir(idl_path.clone())?;
                        for i in dir.flatten() {
                            if i.file_type()?.is_file()
                                && i.path().extension().expect("invalid extension name")
                                    == Into::<OsString>::into("lstr".to_string())
                            {
                                let input_path = Path::new(&idl_path);
                                let out_path = if out_path.is_empty() {
                                    input_path
                                } else {
                                    Path::new(&out_path)
                                };
                                let out_path = out_path.with_file_name(i.file_name());
                                // format("{}/{}")
                                rust_codegen_file(
                                    rust.clone(),
                                    i.path().as_path(),
                                    out_path.as_path(),
                                )?;
                            }
                        }
                    } else {
                        let idl_path = Path::new(&idl_path);
                        let gen_out_path = idl_path.with_extension("rs");
                        let out_path = if out_path.is_empty() {
                            gen_out_path.as_path()
                        } else {
                            Path::new(&out_path)
                        };
                        dbg!(idl_path);
                        dbg!(out_path);
                        rust_codegen_file(rust.clone(), idl_path, out_path)?;
                    }
                }
                _ => unimplemented!("unimplemented codegen target"),
            }
        }
        Limitsc::TypeCheck { path, file } => todo!(),
    }
    Ok(())
}
