/// The shacl-api implementation of a SHACL validation engine.
/// See: https://github.com/sdsc-ordes/shacl-api
use reqwest::blocking::{multipart, Client};
use sophia::{inmem::graph::LightGraph, turtle::parser::turtle};
use sophia_api::prelude::*;
use std::error::Error;

use crate::core::{ShaclEngine, ValidationReport};
use catplus_common::rdf::rdf_serializers::serialize_graph_to_turtle;

pub struct ShaclApiEndpoint {
    url: String,
}

impl ShaclApiEndpoint {
    pub fn new(url: String) -> Self {
        ShaclApiEndpoint { url }
    }
}

impl ShaclEngine for ShaclApiEndpoint {
    fn is_available(&self) -> bool {
        let url = format!("{}/", self.url);
        let client = Client::new();
        let response = client.get(url).send();

        return response.is_ok();
    }

    fn validate(
        &self,
        data: &LightGraph,
        shapes: Option<&LightGraph>,
    ) -> Result<ValidationReport, Box<dyn Error>> {
        let url = format!("{}/validate", self.url);
        // Request report in turtle format
        let accept_header = "text/turtle";

        // Serialize data graph and add to multipart form
        let data_bytes = serialize_graph_to_turtle(&data).unwrap().into_bytes();
        let data_part =
            multipart::Part::bytes(data_bytes).file_name("data.ttl").mime_str("text/turtle")?;

        let mut form = multipart::Form::new().part("data", data_part);

        // If shapes are provided, serialize them and add to form
        if let Some(shapes) = shapes {
            let shapes_bytes = serialize_graph_to_turtle(shapes).unwrap().into_bytes();
            let shapes_part = multipart::Part::bytes(shapes_bytes)
                .file_name("shapes.ttl")
                .mime_str("text/turtle")?;

            form = form.part("shapes", shapes_part);
        };

        let client = Client::new();
        let response = client.post(url).header("Accept", accept_header).multipart(form).send()?;

        let report_graph = turtle::parse_str(&response.text()?).collect_triples()?;

        Ok(ValidationReport::from_graph(report_graph))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use testcontainers::{
        core::{logs, wait, IntoContainerPort, WaitFor},
        runners::SyncRunner,
        GenericImage, ImageExt,
    };

    #[test]
    fn test_shacl_api_endpoint() {
        // Spin up validation service
        let _server = GenericImage::new("ghcr.io/sdsc-ordes/shacl-api", "develop")
            .with_wait_for(
                WaitFor::Log(
                    wait::LogWaitStrategy::stderr(
                        "INFO:     Application startup complete."
                    )
                )
            )
            .with_mapped_port(15400, 15400.tcp())
            .with_env_var("UVICORN_PORT", "15400")
            .with_env_var(
                "SHAPES_URL", 
                "https://github.com/sdsc-ordes/catplus-ontology/releases/download/v0.1.0/catplus_ontology.ttl"
            )
            .with_log_consumer(move |frame: &logs::LogFrame| {
                println!("{}", String::from_utf8_lossy(frame.bytes()));
            })
            .start()
            .unwrap();

        let validator = ShaclApiEndpoint::new("http://localhost:15400".to_string());
        assert!(validator.is_available(), "SHACL API endpoint is not available");

        let data = LightGraph::new();
        let result = validator.validate(&data, None);
        assert!(result.is_ok(), "validation did not complete");

        let report = result.unwrap();
        assert!(!report.conforms, "empty data does not pass validation")
    }
}
