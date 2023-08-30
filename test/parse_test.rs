use limit_stream::ast::{
    Annotation, Constant, Def, EnumDef, Session, SessionDef, SessionType, SimpleType, StructDef,
    Type, TypeOrName,
};
use limit_stream::parser::{_type, def, enum_def, enum_item, session_def, struct_def, struct_item};

macro_rules! gen_test {
    ($parse: expr, $testname: ident, $src: expr, $result: expr) => {
        #[test]
        fn $testname() {
            assert_eq!($parse($src).unwrap().1, $result);
        }
    };
}

gen_test!(
    _type,
    type_test,
    r"recv string -> 
send int -> 
send int -> 
end
",
    Type::Session(SessionType(vec![
        Session::Recv(TypeOrName::Type(Box::new(Type::SimpleType(
            SimpleType::String
        )))),
        Session::Send(TypeOrName::Type(Box::new(Type::SimpleType(
            SimpleType::Int
        )))),
        Session::Send(TypeOrName::Type(Box::new(Type::SimpleType(
            SimpleType::Int
        )))),
        Session::Endpoint,
    ]))
);

gen_test!(
    session_def,
    session_def_test,
    "channel a = recv 1 -> recv 2 -> send 3 -> end",
    SessionDef {
        name: "a",
        session: SessionType(vec![
            Session::Recv(TypeOrName::Type(Box::new(Type::Constant(Constant::Uint(
                1
            ))))),
            Session::Recv(TypeOrName::Type(Box::new(Type::Constant(Constant::Uint(
                2
            ))))),
            Session::Send(TypeOrName::Type(Box::new(Type::Constant(Constant::Uint(
                3
            ))))),
            Session::Endpoint,
        ])
    }
);

gen_test!(
    struct_item,
    struct_item_test,
    "user: User = 0",
    ("user", (TypeOrName::Name("User"), Some(0)))
);

gen_test!(
    struct_def,
    struct_def_test,
    "
struct User {
  name: string = 0,
  age: uint = 1,
  desc: string = 2
}
",
    StructDef {
        name: "User",
        annotation: Annotation {},
        records: vec![
            (
                "name",
                (
                    TypeOrName::Type(Box::new(Type::SimpleType(SimpleType::String))),
                    Some(0)
                )
            ),
            (
                "age",
                (
                    TypeOrName::Type(Box::new(Type::SimpleType(SimpleType::Uint))),
                    Some(1)
                )
            ),
            (
                "desc",
                (
                    TypeOrName::Type(Box::new(Type::SimpleType(SimpleType::String))),
                    Some(2)
                )
            ),
        ]
    }
);

gen_test!(
    enum_item,
    enum_item_test,
    "Admin(user) = 0
",
    ("Admin", (TypeOrName::Name("user"), Some(0)))
);

gen_test!(
    enum_def,
    enum_def_test,
    "
enum usertype {
  Admin(user) = 0,
  Normal(user) = 1,
  Visitor(visitor) = 2
}
",
    EnumDef {
        name: "usertype",
        items: vec![
            ("Admin", (TypeOrName::Name("user"), Some(0))),
            ("Normal", (TypeOrName::Name("user"), Some(1))),
            ("Visitor", (TypeOrName::Name("visitor"), Some(2))),
        ]
    }
);
