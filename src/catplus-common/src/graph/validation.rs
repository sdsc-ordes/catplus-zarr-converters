/// Interface for validating an RDF graph.

use sophia::inmem::graph::LightGraph;
use sophia::turtle::parser::turtle;
use reqwest::blocking::{Client, multipart};
use sophia_api::source::TripleSource;

use crate::rdf::rdf_serializers::serialize_graph_to_turtle;

pub trait GraphValidator {
    fn validate(&self, data: &LightGraph, shapes: Option<&LightGraph>) -> Result<LightGraph, GraphValidationError>;

}

pub struct GraphValidationError {
    pub message: String,
}

pub struct ShaclApiEndpoint {
    url: String,
}

impl ShaclApiEndpoint {
    pub fn new(url: String) -> Self {
        ShaclApiEndpoint { url }
    }
}

impl GraphValidator for ShaclApiEndpoint {
    fn validate(&self, data: &LightGraph, shapes: Option<&LightGraph>) -> Result<LightGraph, GraphValidationError> {
        // serialize graphs to ttl
        let url = format!("{}/validate", self.url);
        let accept_header = "text/turtle";

        // Serialize data graph and add to multipart form
        let data_bytes = serialize_graph_to_turtle(&data).unwrap().into_bytes();
        let data_part = multipart::Part::bytes(data_bytes)
            .file_name("data.ttl")
            .mime_str("text/turtle")
            .unwrap();

        let mut form = multipart::Form::new()
            .part("data", data_part);

        // If shapes are provided, serialize them and add to form
        if let Some(shapes) = shapes {
            let shapes_bytes = serialize_graph_to_turtle(shapes).unwrap().into_bytes();
            let shapes_part = multipart::Part::bytes(shapes_bytes)
                .file_name("shapes.ttl")
                .mime_str("text/turtle")
                .unwrap();
                        
            form = form.part("shapes", shapes_part);
        };

        let client = Client::new();
        let response = client
            .post(url)
            .header("Accept", accept_header)
            .multipart(form)
            .send()
            .unwrap();

        let report_text = response.text().unwrap();
        println!("report: {:?}", report_text);

        let report = turtle::parse_str(&report_text)
            .collect_triples()
            .unwrap();

        Ok(report)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{thread, time};
    use testcontainers::{
        core::{IntoContainerPort, logs, WaitFor, wait},
        runners::SyncRunner,
        GenericImage,
        ImageExt,
    };

    #[test]
    fn test_shacl_api_endpoint() {
        // Spin up validation service
        let _server = GenericImage::new("ghcr.io/sdsc-ordes/shacl-api", "refactor-endpoints")
            .with_wait_for(
                WaitFor::Log(
                    wait::LogWaitStrategy::stderr(
                        "INFO:     Application startup complete."
                    )
                )
            )
            .with_mapped_port(8001, 15400.tcp())
            .with_env_var(
                "SHAPES_URL", 
                "https://github.com/sdsc-ordes/catplus-ontology/releases/download/v0.1.0/catplus_ontology.ttl"
            )
            .start()
            .unwrap();

        // wait for server to start up by polling the endpoint
        let url = "http://localhost:8001";

               
        let validator = ShaclApiEndpoint::new(url.to_string());
        let data = LightGraph::new();
        let result = validator.validate(&data, None);
        //let result = validator.validate(&data, Some(&shapes));
        assert!(result.is_ok());
    }
}
