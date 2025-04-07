use crate::graph::namespaces::{
    allocom, allodc, allohdf, allohdfcube, alloproc, alloprop, alloqual, allores, allorole, cat, obo, purl,
    qb, qudt, qudtext, schema, unit,
};
use sophia_api::{prefix::Prefix, prelude::Iri};

use lazy_static::lazy_static;
use sophia::api::ns::Namespace;

lazy_static! {
    pub static ref rdf: Namespace<&'static str> =
        Namespace::new("http://www.w3.org/1999/02/22-rdf-syntax-ns#").unwrap();
    pub static ref xsd: Namespace<&'static str> =
        Namespace::new("http://www.w3.org/2001/XMLSchema#").unwrap();
}

macro_rules! ns_entries_direct {  // For rdf and xsd
    ($msg:expr, $($ns:ident),*) => {
        vec![
            $(
                (stringify!($ns), $ns.get("").expect(&$msg)),
            )*
        ]
    };
}

macro_rules! ns_entries_module { // For the other modules
    ($msg:expr, $($module:ident),*) => {
        vec![
            $(
                (stringify!($module), $module::ns.get("").expect(&$msg)),
            )*
        ]
    };
}

pub fn generate_prefix_map() -> Vec<(Prefix<Box<str>>, Iri<Box<str>>)> {
    let msg = "Namespace URI should always be valid";
    ns_entries_direct!(msg, rdf, xsd) // Correct call for rdf and xsd
        .into_iter()
        .chain(
            ns_entries_module!(
                // Correct call for the other modules
                msg,
                cat,
                schema,
                unit,
                allores,
                allorole,
                alloproc,
                alloprop,
                allocom,
                allohdf,
                allohdfcube,
                qb,
                qudt,
                qudtext,
                alloqual,
                allodc,
                purl,
                obo
            )
            .into_iter(),
        )
        .map(|(prefix, iri)| {
            (
                Prefix::new(prefix.to_string().into_boxed_str()).expect("Invalid prefix"),
                Iri::new(iri.to_string().into_boxed_str()).expect("Invalid IRI"),
            )
        })
        .collect()
}
