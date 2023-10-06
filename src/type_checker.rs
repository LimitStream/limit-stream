//! Type checker
//!
//! steps:
//! 1. check `end` exists?
//! 2. check `ring` exists?
//! 3. check if could generate dual type
//!
//!

use std::collections::HashMap;

use petgraph::{algo::is_cyclic_directed, Graph};

use crate::ast::{Def, GetFields, GetName, Type, TypeOrName};

pub enum Error<'a> {
    NameIsNotFound(&'a str),
}

pub fn ring_checker<'a>(irs: &'a [Def]) -> Result<bool, Error<'a>> {
    let g = ir2graph(irs)?;
    Ok(is_cyclic_directed(&g))
}

pub fn ir2graph<'a>(irs: &'a [Def]) -> Result<Graph<Type<'a>, ()>, Error<'a>> {
    let table: HashMap<_, _> = irs
        .iter()
        .map(|def| (def.get_name(), def.clone().into()))
        .collect();
    let r: Result<_, Error> = irs
        .iter()
        .map(|def| {
            def.get_fields()
                .into_iter()
                .map(|type_or_name| -> Result<(Type<'a>, Type<'a>), Error> {
                    Ok((
                        def.clone().into(),
                        unify_type_or_name(&type_or_name, &table)?,
                    ))
                })
                .collect::<Result<Vec<_>, Error>>()
        })
        .collect::<Result<Vec<Vec<_>>, Error>>();
    let r = r?.into_iter().flatten();

    let mut graph: Graph<Type, ()> = Graph::new();
    for (node0, node1) in r {
        let node0 = graph.add_node(node0);
        let node1 = graph.add_node(node1);
        graph.add_edge(node0, node1, ());
    }
    Ok(graph)
}

fn unify_type_or_name<'a>(
    i: &TypeOrName<'a>,
    table: &HashMap<&str, Type<'a>>,
) -> Result<Type<'a>, Error<'a>> {
    match i {
        TypeOrName::Name(n) => table.get(n).cloned().ok_or(Error::NameIsNotFound(n)),
        TypeOrName::Type(_) => unimplemented!(),
    }
}
