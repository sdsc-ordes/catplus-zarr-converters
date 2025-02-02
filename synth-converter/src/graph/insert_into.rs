use anyhow::anyhow;
use sophia::inmem::graph::LightGraph;
use sophia_api::{
    graph::MutableGraph,
    term::{SimpleTerm, Term},
};

use super::utils::generate_bnode_term;

/// Used in [InsertIntoGraph::attach_and_insert].
pub struct AttachTo<'a, 'b> {
    pub iri: SimpleTerm<'a>,
    pub pred: SimpleTerm<'b>,
}

/// InsertIntoGraph provides a trait to implement the conversion into a graph
/// by different types.
pub trait InsertIntoGraph {
    /// Insert inserts `&self` into `graph` with instance name `iri`
    fn insert(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()>;

    /// Insert inserts `&self` into `graph` with instance name `iri` (default to a blank node)
    /// and will attach to `attach_link` if its not empty.
    fn attach_and_insert(
        &self,
        graph: &mut LightGraph,
        iri: Option<SimpleTerm>,
        attach_link: Option<AttachTo>,
    ) -> anyhow::Result<()> {
        let new_iri = iri.unwrap_or_else(|| generate_bnode_term());

        if let Some(a) = attach_link {
            _ = graph.insert(&a.iri, &a.pred, new_iri.clone());
        }

        self.insert(graph, new_iri)
    }
}

/// Blanket implementation for [Option<T>].
impl<T> InsertIntoGraph for Option<T>
where
    T: InsertIntoGraph,
{
    fn insert(&self, archive: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        if let Some(v) = self {
            v.insert(archive, iri)?
        }
        Ok(())
    }

    fn attach_and_insert(
        &self,
        archive: &mut LightGraph,
        iri: Option<SimpleTerm>,
        attach_link: Option<AttachTo>,
    ) -> anyhow::Result<()> {
        if let Some(v) = self {
            v.attach_and_insert(archive, iri, attach_link)?
        }
        Ok(())
    }
}

/// Default stupid implementation for [SimpleTerm].
impl<'a> InsertIntoGraph for SimpleTerm<'a> {
    fn insert(&self, _archive: &mut LightGraph, _iri: SimpleTerm) -> anyhow::Result<()> {
        panic!(
            "cannot insert {:?} into graph, use `attach_and_insert`",
            &self
        )
    }

    fn attach_and_insert(
        &self,
        graph: &mut LightGraph,
        _iri: Option<SimpleTerm>,
        attach_to: Option<AttachTo>,
    ) -> anyhow::Result<()> {
        if attach_to.is_none() {
            return Err(anyhow!(
                "cannot insert {:?} into graph without 'attach_to'",
                &self
            ));
        }
        assert!(!self.is_triple());

        if let Some(a) = attach_to {
            _ = graph.insert(&a.iri, &a.pred, self);
        }

        Ok(())
    }
}
