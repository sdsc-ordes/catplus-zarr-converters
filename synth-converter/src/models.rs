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

pub struct AttachTo<'a, 'b> {
    iri: SimpleTerm<'a>,
    pred: SimpleTerm<'b>,
}

pub trait GraphSerializer {
    fn serialize(&self, archive: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()>;

    fn serialize_link(
        &self,
        archive: &mut LightGraph,
        iri: Option<SimpleTerm>,
        attach_link: Option<AttachTo>,
    ) -> anyhow::Result<()> {
        let new_iri = iri.unwrap_or_else(|| generate_bnode_term());

        if let Some(a) = attach_link {
            _ = archive.insert(&a.iri, &a.pred, new_iri.clone());
        }

        self.serialize(archive, new_iri)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Observation {
    pub value: f64,
    pub unit: String,
    pub error_margin: Option<ErrorMargin>,
}

impl<'a> GraphSerializer for SimpleTerm<'a> {
    fn serialize(&self, _archive: &mut LightGraph, _iri: SimpleTerm) -> anyhow::Result<()> {
        Ok(())
    }

    fn serialize_link(
        &self,
        archive: &mut LightGraph,
        _iri: Option<SimpleTerm>,
        attach_to: Option<AttachTo>,
    ) -> anyhow::Result<()> {
        assert!(attach_to.is_some());
        assert!(!self.is_triple());

        if let Some(a) = attach_to {
            _ = archive.insert(&a.iri, &a.pred, self);
        }

        Ok(())
    }
}

impl GraphSerializer for Observation {
    fn serialize(&self, archive: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (prop, value) in [
            (rdf::type_, cat::Observation.as_simple()),
            (qudt::unit, self.unit.as_simple()),
            (qudt::value, self.value.as_simple()),
        ] {
            archive.insert(&iri, prop, value)?;
        }

        for (prop, value) in [(cat::errorMargin, &self.error_margin)] {
            if let Some(val) = value {
                val.serialize_link(
                    archive,
                    None,
                    Some(AttachTo {
                        iri: iri.clone(),
                        pred: prop.as_simple(),
                    }),
                )?;
            }
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ErrorMargin {
    pub value: f64,
    pub unit: String,
}

impl GraphSerializer for ErrorMargin {
    fn serialize(&self, archive: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (prop, value) in [
            (qudt::unit, self.unit.as_simple()),
            (qudt::value, self.value.as_simple()),
        ] {
            _ = archive.insert(&iri, prop, value);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let mut graph = LightGraph::new();
        observation.serialize_link(&mut graph, None, None)?;

        Ok(())
    }
}
