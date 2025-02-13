use sophia_isomorphism::isomorphic_graphs;
use hci_converter::convert::json_to_rdf;
use catplus_common::rdf::rdf_parser::parse_turtle_to_graph;

#[test]
fn test_convert_campaign() {
    let json_data = r#"
        {
            "hasCampaign": {
                "campaignName": "Caffeine Synthesis",
                "description": "1-step N-methylation of theobromine to caffeine"
            }
        }
    "#;
    let result = json_to_rdf(json_data, "turtle");
    let expected_ttl = r#"
        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
        PREFIX cat: <http://example.org/cat#>
        PREFIX schema: <https://schema.org/>
        PREFIX unit: <https://qudt.org/vocab/unit/>
        PREFIX allores: <http://purl.allotrope.org/ontologies/result#>
        PREFIX alloproc: <http://purl.allotrope.org/ontologies/process#>
        PREFIX qudt: <http://qudt.org/schema/qudt/>
        PREFIX alloqual: <http://purl.allotrope.org/ontologies/quality#>
        PREFIX purl: <http://purl.allotrope.org/ontologies/>
        PREFIX obo: <http://purl.obolibrary.org/obo/>
        PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

        [] a cat:Campaign;
        schema:name "Caffeine Synthesis".
    "#;
    let expected_graph = parse_turtle_to_graph(&expected_ttl).unwrap();
    let result_ttl = result.as_ref().unwrap().as_str();
    let result_graph = parse_turtle_to_graph(&result_ttl).unwrap();
    let graphs_match = isomorphic_graphs(&result_graph, &expected_graph);
    assert_eq!(graphs_match.unwrap(), true);
}
