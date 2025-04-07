use crate::{
    graph::{
        graph_builder::GraphBuilder,
        insert_into::{InsertIntoGraph, Link},
        namespaces::{allocom, allohdf, allores, cat, obo, purl, schema},
    },
    models::core::Chemical,
};

use anyhow;
use serde::{Deserialize, Serialize};
use sophia::api::ns::rdf;
use sophia_api::term::{SimpleTerm, Term};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Campaign {
    pub campaign_name: String,
    pub description: String,
    #[serde(rename = "objective")]
    pub generic_objective: String,
    pub campaign_class: String,
    #[serde(rename = "type")]
    pub campaign_type: String,
    pub reference: String,
    pub has_objective: Option<Objective>,
    pub has_batch: HciBatch,
    pub has_chemical: Option<Vec<Chemical>>,
}

impl InsertIntoGraph for Campaign {
    fn insert_into(&self, builder: &mut GraphBuilder, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &cat::Campaign.as_simple() as &dyn InsertIntoGraph),
            (schema::name, &self.campaign_name.as_simple()),
            (schema::description, &self.description.as_simple()),
            (cat::genericObjective, &self.generic_objective.as_simple()),
            (cat::campaignClass, &self.campaign_class.as_simple()),
            (cat::campaignType, &self.campaign_type.as_simple()),
            (allores::AFR_0002764, &self.reference.as_simple()),
            (cat::hasObjective, &self.has_objective),
            (cat::hasBatch, &self.has_batch),
            (cat::hasChemical, &self.has_chemical),
        ] {
            value.attach_into(
                builder,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Objective {
    pub criteria: String,
    pub condition: String,
    pub description: String,
    pub objective_name: String,
}

impl InsertIntoGraph for Objective {
    fn insert_into(&self, builder: &mut GraphBuilder, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &obo::IAO_0000005.as_simple()),
            (schema::name, &self.objective_name.as_simple()),
            (schema::description, &self.description.as_simple()),
            (cat::criteria, &self.criteria.as_simple()),
            (allocom::AFC_0000090, &self.condition.as_simple()),
        ] {
            value.attach_into(
                builder,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct CampaignWrapper {
    #[serde(rename = "hasCampaign")]
    pub has_campaign: Campaign,
}
impl InsertIntoGraph for CampaignWrapper {
    fn insert_into(&self, builder: &mut GraphBuilder, iri: SimpleTerm) -> anyhow::Result<()> {
        self.has_campaign.insert_into(builder, iri)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "Batch")]
pub struct HciBatch {
    #[serde(rename = "batchID")]
    pub batch_id: String,
    pub batch_name: Option<String>,
    pub reaction_type: Option<String>,
    pub reaction_name: Option<String>,
    pub optimization_type: Option<String>,
    pub link: Option<String>,
}

impl InsertIntoGraph for HciBatch {
    fn insert_into(&self, builder: &mut GraphBuilder, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &cat::Batch.as_simple() as &dyn InsertIntoGraph),
            (purl::identifier, &self.batch_id.as_simple()),
            (schema::name, &self.batch_name.as_ref().clone().map(|s| s.as_simple())),
            (allohdf::HardLink, &self.link.as_ref().clone().map(|s| s.as_simple())),
            (cat::reactionType, &self.reaction_type.as_ref().clone().map(|s| s.as_simple())),
            (cat::reactionName, &self.reaction_name.as_ref().clone().map(|s| s.as_simple())),
            (
                cat::optimizationType,
                &self.optimization_type.as_ref().clone().map(|s| s.as_simple()),
            ),
        ] {
            value.attach_into(
                builder,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }

        Ok(())
    }
}
