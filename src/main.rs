use clap::Parser;

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
        #[arg(short, long, default_value_t = String::from("."), help = "output path directory")]
        out_path: String,
        #[arg(short, long, help = "entry file")]
        file: String,
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

fn main() {
    let args = Limitsc::parse();
    println!("{:#?}", args);
}
