use crate::graph::{
    namespaces::{cat, qudt},
    utils::generate_bnode_term,
};
use anyhow;
use serde::{Deserialize, Serialize};
use sophia::{
    api::{graph::MutableGraph, ns::rdf},
    inmem::graph::LightGraph,
};
use sophia_api::{
    ns::NsTerm,
    term::{FromTerm, SimpleTerm, Term},
};

// NOTE:
// - Try to make MWE working next time, or at least comment the stuff out which does not work,
//   none compiling things in Rust are quite hard to debug.
// - If dealing with a new type which is more difficult to understand as the `SimpleTerm` which
//   is a owned or borrowed `str` underneath, its best to go slow and first learn how
//   this type behaves, not just throwing `clone`, more references, dyn Traits and more
//   conversion loops at the problem, its only getting more complicate and probably gets you
//   nowhere.
//   I first tried to understand that `'static` is needed and `SimpleTerm::from_term` to
//   convert to an owned `SimpleTerm` then I tried to have the basic function signature
//   with minimal lifetime generics. Do not implement further till you have the interface how you
//   want, Rust will otherwise stop you when you wrote like 4k lines of code and then the shit to
//   untangle gets hard.
// - Naming in code is really important, to not make it more complicate than it should be:
//   Generally naming everything `subject`, `object`, `predicate` does not help.
//   We know that we are dealing with these types, they are not really concrete. Rather
//   use `props`, `values` where it makes sense.
//   Be consistent: `iri` not `url` etc. no mix, then `data_properties` is to long, keep variable
//   to a reasonable length: in small function (which always should be the case)
//   `props` is better.
// - Also `to_triples` might have wrong signature `subject_iri: IriRef` probably.
// - Also using `SimpleTerm` is ok to use, I suspect it to be at certain points `IRI`s,
//   either assert that fact with the TypeSystem or check at runtime.
// - The `ToGraph` is a nice approach, but should be probably named differently and I might have
//   slightly chosen another approach but its a good start:
//   `GraphSerializer` and the function `fn serialize(&self, graph: &mut LightGraph)` which
//   directly spits the triples into the graph. And then a serialization loop which is fully
//   recursive.

pub fn link_graph<G>(
    source_iri: SimpleTerm,
    predicate: SimpleTerm,
    graph: &G,
) -> Vec<[SimpleTerm<'static>; 3]>
where
    G: ToGraph,
{
    let node_iri = graph.get_uri();

    // Create the link.
    let mut triples = vec![[source_iri, predicate, node_iri.clone()].map(SimpleTerm::from_term)];

    // Serialize the graph.
    let mut t = graph.to_triples(node_iri);
    triples.append(&mut t);

    triples
}

/// Convert a struct into an RDF graph.
pub trait ToGraph {
    fn to_triples(&self, subject_iri: SimpleTerm) -> Vec<[SimpleTerm<'static>; 3]>;

    fn to_graph(&self, graph_iri: SimpleTerm) -> anyhow::Result<LightGraph> {
        let mut graph = LightGraph::new();
        let triples = self.to_triples(graph_iri);
        for triple in triples {
            graph.insert(&triple[0], &triple[1], &triple[2])?;
        }
        return Ok(graph);
    }

    fn get_uri(&self) -> SimpleTerm<'static> {
        generate_bnode_term()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Observation {
    pub value: f64,
    pub unit: String,
    pub error_margin: Option<ErrorMargin>,
}

impl ToGraph for Observation {
    fn to_triples(&self, observe_iri: SimpleTerm) -> Vec<[SimpleTerm<'static>; 3]> where {
        let props: [(NsTerm, SimpleTerm<'_>); 3] = [
            (rdf::type_, cat::Observation.as_simple()),
            (qudt::unit, self.unit.as_simple()),
            (qudt::value, self.value.as_simple()),
        ];

        let mut triples: Vec<[SimpleTerm<'static>; 3]> = props
            .into_iter()
            .map(|(prop, value)| {
                [observe_iri.clone(), prop.as_simple(), value].map(SimpleTerm::from_term)
            })
            .collect();

        let props = [(cat::errorMargin, &self.error_margin)];
        for (prop, value) in props {
            if let Some(val) = value {
                let mut g = link_graph(observe_iri.clone(), prop.as_simple(), val);
                triples.append(&mut g);
            }
        }

        triples
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ErrorMargin {
    pub value: f64,
    pub unit: String,
}

impl ToGraph for ErrorMargin {
    fn to_triples(&self, subject: SimpleTerm) -> Vec<[SimpleTerm<'static>; 3]> {
        let props = [
            (qudt::unit, self.unit.as_simple()),
            (qudt::value, self.value.as_simple()),
        ];

        let triples: Vec<[SimpleTerm<'static>; 3]> = props
            .into_iter()
            .map(|(predicate, object)| {
                [subject.clone(), predicate.as_simple(), object].map(SimpleTerm::from_term)
            })
            .collect();

        triples
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::graph::namespaces::{cat, qudt};
    use sophia::api::ns::rdf;

    #[test]
    fn test_observation_to_triples() {
        let observation = Observation {
            value: 42.0,
            unit: "http://qudt.org/vocab/unit#DegreeCelsius".to_string(),
            error_margin: Some(ErrorMargin {
                value: 0.5,
                unit: "http://qudt.org/vocab/unit#DegreeCelsius".to_string(),
            }),
        };
        let obs_subject = observation.get_uri();
        let triples = observation.to_triples(obs_subject);
        assert_eq!(triples.len(), 6);
    }
}
