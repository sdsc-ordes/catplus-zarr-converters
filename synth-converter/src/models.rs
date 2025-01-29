
// The structure follows the input data as descibed in the
// https://github.com/sdsc-ordes/cat-plus-ontology see here for the expected Synth input data:
// https://github.com/sdsc-ordes/cat-plus-ontology/tree/96091fd2e75e03de8a4c4d66ad502b2db27998bd/json-file/1-Synth
use std::fmt;
use crate::graph::{
    namespaces::{alloproc, alloqual, allores, cat, obo, purl, qudt, schema},
    utils::generate_bnode_term,
};
use anyhow;
use serde::{Deserialize, Serialize};
use sophia::{
    api::{
        graph::MutableGraph,
        ns::rdf,
    },
    inmem::graph::LightGraph,
};
use sophia_api::{
    ns::NsTerm,
    term::{SimpleTerm, Term},
};

fn to_graph_box<T: ToGraph + 'static>(item: T) -> Box<dyn ToGraph> {
    Box::new(item)
}

pub fn link_node<'a, 'b, 'c, 'd, N>(source_uri: SimpleTerm<'a>, predicate: SimpleTerm<'b>, node: &'d N) -> Vec<[SimpleTerm<'c>; 3]>
where
    N: ToGraph + ?Sized,
    'a: 'c,
    'b: 'c,
    'd: 'c
{
    let node_uri = node.get_uri();
    let mut triples = vec![[source_uri.clone(), predicate.clone(), node_uri.clone()]];
    triples.append(&mut node.to_triples(node_uri.clone()));

    triples
}

/// Convert a struct into an RDF graph.
pub trait ToGraph {
    /// Represent the struct as a collection of triples.
    ///
    /// # Arguments
    /// - `subject`: The URI to use for the struct being converted.
    //
    /// # Returns
    /// A collection of triples.
    fn to_triples<'a, 'b, 'c>(&'c self, subject: SimpleTerm<'a>) -> Vec<[SimpleTerm<'b>; 3]>
    where
        'c: 'b,
        'a: 'b;

    /// Convert the struct to a graph.
    ///
    /// # Arguments
    /// - `subject`: The URI to use for the struct being converted.
    ///
    /// # Returns
    /// The graph representation of the struct.
    fn to_graph(&self, subject: SimpleTerm) -> anyhow::Result<LightGraph> {
        let mut graph = LightGraph::new();
        let triples = self.to_triples(subject);
        for triple in triples {
            graph.insert(&triple[0], &triple[1], &triple[2])?;
        }
        return Ok(graph);
    }

    /// Get the URI for the struct.
    ///
    /// The default implementation generates a random blank node URI.
    ///
    /// # Returns
    /// The URI for the struct.
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
    fn to_triples<'a, 'b, 'c>(&'c self, subject: SimpleTerm<'a>) -> Vec<[SimpleTerm<'b>; 3]>
    where
        'c: 'b,
        'a: 'b,
    {
        let data_properties = [
            (&rdf::type_, &cat::Observation.as_simple()),
            (&qudt::unit, &self.unit.as_simple()),
            (&qudt::value, &self.value.as_simple()),
        ];

        let object_properties = [(cat::errorMargin, &self.error_margin)];

        let mut triples: Vec<[SimpleTerm; 3]> = data_properties
            .into_iter()
            .map(|(predicate, object)| [subject.clone(), predicate.as_simple(), object.as_simple()])
            .collect();

        for (pred, object) in object_properties {
            if let Some(obj) = object {
                triples.append(&mut link_node(subject.clone(), pred.as_simple(), *obj));
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
    fn to_triples<'a, 'b, 'c>(&'c self, subject: SimpleTerm<'a>) -> Vec<[SimpleTerm<'b>; 3]>
    where
        'c: 'b,
        'a: 'b,
    {
        let data_properties = [
            (&qudt::unit, self.unit.as_simple()),
            (&qudt::value, self.value.as_simple()),
        ];

        let triples = data_properties
            .into_iter()
            .map(|(predicate, object)| [subject.clone(), predicate.as_simple(), object])
            .collect();

        triples
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::graph::namespaces::{
        qudt,
        cat,
    };
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
        let triples = observation.to_triples(obs_subject.clone());
        assert_eq!(triples.len(), 5);
        format!(
            r#"
            <{:?}> a cat:Observation;
            qudt:unit <http://qudt.org/vocab/unit#DegreeCelsius>;
            qudt:value 42.0;
            cat:errorMargin [
                qudt:unit <http://qudt.org/vocab/unit#DegreeCelsius>;
                qudt:value 0.5
            ] .
            "#,
            obs_subject,
        );
        let error_margin_subject = generate_bnode_term();
        let expected = [
            [obs_subject.clone(), rdf::type_.as_simple(), cat::Observation.as_simple()],
            [obs_subject.clone(), qudt::unit.as_simple(), "http://qudt.org/vocab/unit#DegreeCelsius".as_simple()],
            [obs_subject.clone(), qudt::value.as_simple(), 42.0.as_simple()],
            [obs_subject.clone(), cat::errorMargin.as_simple(), error_margin_subject.clone()],
            [error_margin_subject.clone(), rdf::type_.as_simple(), cat::Observation.as_simple()],
            [error_margin_subject.clone(), qudt::unit.as_simple(), "http://qudt.org/vocab/unit#DegreeCelsius".as_simple()],
            [error_margin_subject.clone(), qudt::value.as_simple(), 0.5.as_simple()],
        ];
        for (i, triple) in triples.iter().enumerate() {
            assert_eq!(triple, &expected[i]);
        }
    }
}
