/// Interface for validating an RDF graph.

use sophia::inmem::graph::LightGraph;

pub trait GraphValidator {
    fn validate(&self, data: &LightGraph, shapes: &LightGraph) -> Result<(), GraphValidationError>;

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
    fn validate(&self, data: &LightGraph, shapes: &LightGraph) -> Result<(), GraphValidationError> {
        // serialize graphs to ttl
        // base64 encode ttl
        // send POST request /validate
        // body should be {datafile: <data>, shapesfile: <data>}
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use testcontainers::{
        core::IntoContainerPort,
        runners::SyncRunner,
        GenericImage,
        ImageExt,
    };

    #[test]
    fn test_shacl_api_endpoint() {
        let validator = ShaclApiEndpoint::new("http://example.com".to_string());
        let data = LightGraph::new();
        let shapes = LightGraph::new();
        let result = validator.validate(&data, &shapes);
        assert!(result.is_ok());
    }
}
