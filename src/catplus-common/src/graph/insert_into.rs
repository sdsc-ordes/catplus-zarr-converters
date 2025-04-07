use crate::graph::{
    graph_builder::{GraphBuilder, OutputNodeStrategy},
    utils::{generate_bnode_term, generate_iri_term},
};
use sophia_api::{
    graph::MutableGraph,
    term::{SimpleTerm, Term},
};

/// Used in [InsertIntoGraph::attach_and_insert].
#[derive(Clone)]
pub struct Link<'a, 'b, 'c> {
    pub source_iri: SimpleTerm<'a>,
    pub pred: SimpleTerm<'b>,
    pub target_iri: Option<SimpleTerm<'c>>,
}

/// InsertIntoGraph provides a trait to implement the conversion into a graph
/// by different types.
pub trait InsertIntoGraph {
    /// Inserts `&self` into `graph` with subject IRI `iri`
    fn insert_into(&self, builder: &mut GraphBuilder, iri: SimpleTerm) -> anyhow::Result<()>;

    /// Inserts `&self` into `graph` with subject IRI `iri` (default to a blank node)
    /// and "attach" self to an existing node with an additional triple.
    fn attach_into(&self, builder: &mut GraphBuilder, attach: Link) -> anyhow::Result<()> {
        let strategy = &builder.node_strategy;
        let target_node = match attach.target_iri {
            Some(target) => target, // Use provided target IRI directly
            None => {
                // No target IRI provided, decide default based on strategy
                match strategy {
                    OutputNodeStrategy::Iri => self.get_uri(), // Default to IRI generation/retrieval
                    OutputNodeStrategy::BNode => generate_bnode_term(), // Default to BNode generation
                }
            }
        };
        _ = builder.graph.insert(&attach.source_iri, &attach.pred, &target_node);

        self.insert_into(builder, target_node)
    }

    fn get_uri(&self) -> SimpleTerm<'static> {
        generate_iri_term()
    }

    fn get_bnode(&self) -> SimpleTerm<'static> {
        generate_bnode_term()
    }
}

/// Default implementation for [Option<T>].
impl<T> InsertIntoGraph for Option<T>
where
    T: InsertIntoGraph,
{
    fn insert_into(&self, archive: &mut GraphBuilder, iri: SimpleTerm) -> anyhow::Result<()> {
        if let Some(v) = self {
            v.insert_into(archive, iri)?
        }
        Ok(())
    }

    fn attach_into(&self, builder: &mut GraphBuilder, attach: Link) -> anyhow::Result<()> {
        if let Some(v) = self {
            v.attach_into(builder, attach)?
        }
        Ok(())
    }
}

impl<T> InsertIntoGraph for Vec<T>
where
    T: InsertIntoGraph,
{
    fn insert_into(&self, builder: &mut GraphBuilder, iri: SimpleTerm) -> anyhow::Result<()> {
        for item in self {
            item.insert_into(builder, iri.clone())?;
        }
        Ok(())
    }

    fn attach_into(&self, builder: &mut GraphBuilder, attach: Link) -> anyhow::Result<()> {
        for item in self {
            item.attach_into(builder, attach.clone())?;
        }
        Ok(())
    }
}

/// Default implementation for [SimpleTerm].
impl<'a> InsertIntoGraph for SimpleTerm<'a> {
    fn insert_into(&self, _builder: &mut GraphBuilder, _iri: SimpleTerm) -> anyhow::Result<()> {
        unimplemented!("cannot insert {:?} into graph, use `attach_and_insert`", &self)
    }

    fn attach_into(&self, builder: &mut GraphBuilder, attach: Link) -> anyhow::Result<()> {
        assert!(!self.is_triple());

        _ = builder.graph.insert(&attach.source_iri, &attach.pred, self);

        Ok(())
    }
}
