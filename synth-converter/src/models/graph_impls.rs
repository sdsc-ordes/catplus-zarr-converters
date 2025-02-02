use crate::graph::{
    insert_into::{AttachTo, InsertIntoGraph},
    namespaces::{cat, qudt},
};
use anyhow;
use sophia::{api::ns::rdf, inmem::graph::LightGraph};
use sophia_api::term::{SimpleTerm, Term};

use super::{ErrorMargin, Observation};

/// Implementation for concrete [Observation].
impl InsertIntoGraph for Observation {
    fn insert(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (prop, value) in [
            (
                rdf::type_,
                &cat::Observation.as_simple() as &dyn InsertIntoGraph,
            ),
            (qudt::unit, &self.unit.as_simple() as &dyn InsertIntoGraph),
            (qudt::value, &self.value.as_simple() as &dyn InsertIntoGraph),
            (cat::errorMargin, &self.error_margin as &dyn InsertIntoGraph),
        ] {
            value.attach_and_insert(
                graph,
                None,
                Some(AttachTo {
                    iri: iri.clone(),
                    pred: prop.as_simple(),
                }),
            )?;
        }

        Ok(())
    }
}

/// Implementation for concrete [Observation].
impl InsertIntoGraph for ErrorMargin {
    fn insert(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (prop, value) in [
            (
                rdf::type_,
                &cat::errorMargin.as_simple() as &dyn InsertIntoGraph,
            ),
            (qudt::unit, &self.unit.as_simple() as &dyn InsertIntoGraph),
            (qudt::value, &self.value.as_simple() as &dyn InsertIntoGraph),
        ] {
            value.attach_and_insert(
                graph,
                None,
                Some(AttachTo {
                    iri: iri.clone(),
                    pred: prop.as_simple(),
                }),
            )?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use sophia::iri::IriRef;
    use sophia_api::term::Term;

    use crate::{
        graph::{graph_builder::GraphBuilder, insert_into::InsertIntoGraph},
        models::{ErrorMargin, Observation},
    };

    #[test]
    fn test_observation_to_triples() -> anyhow::Result<()> {
        let observation = Observation {
            value: 42.0,
            unit: "http://qudt.org/vocab/unit#DegreeCelsius".to_string(),
            error_margin: Some(ErrorMargin {
                value: 0.5,
                unit: "http://qudt.org/vocab/unit#DegreeCelsius".to_string(),
            }),
        };

        let mut b = GraphBuilder::new();
        let i = IriRef::new_unchecked("http://test.com/my-obersvation");
        observation.attach_and_insert(&mut b.graph, Some(i.as_simple()), None)?;
        println!("Graph\n{}", b.serialize_to_turtle().unwrap());

        Ok(())
    }
}
