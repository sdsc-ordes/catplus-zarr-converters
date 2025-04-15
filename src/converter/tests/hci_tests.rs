use catplus_common::{models::hci::CampaignWrapper, rdf::rdf_parser::parse_turtle_to_graph};
use converter::convert::{json_to_rdf, RdfFormat};
use sophia_isomorphism::isomorphic_graphs;

#[test]
fn test_convert_campaign() {
    let output_format = RdfFormat::Turtle;
    let json_data = r#"
        {
            "hasCampaign": {
                "campaignName": "Caffeine Synthesis",
                "description": "1-step N-methylation of theobromine to caffeine",
                "objective": "High caffeine yield at the end",
                "campaignClass": "Standard Research",
                "type": "optimization",
                "reference": "Substitution reaction - SN2",
                "hasBatch": {
                    "batchID": "23",
                    "batchName": "20240516",
                    "reactionType": "N-methylation",
                    "reactionName": "Caffeine synthesis",
                    "optimizationType": "Yield optimization",
                    "link": "https://www.sciencedirect.com/science/article/pii/S0187893X15720926"
                },
                "hasObjective": {
                    "criteria": "Yield ≥ 90%",
                    "condition": "Reflux in acetone with methyl iodide and potassium carbonate",
                    "description": "Optimize reaction conditions to maximize caffeine yield from theobromine using methyl iodide",
                    "objectiveName": "Maximize caffeine formation"
                },
                "hasChemical": [
                    {
                        "chemicalID": "19",
                        "chemicalName": "Sodium methoxide",
                        "CASNumber": "124-41-4",
                        "molecularMass": {
                            "value": 54.024,
                            "unit": "g/mol"
                        },
                        "smiles": "C[O-].[Na+]",
                        "swissCatNumber": "SwissCAT-10942334",
                        "keywords": "optional only in HCI file",
                        "Inchi": "InChI=1S/CH3O.Na/c1-2;/h1H3;/q-1;+1",
                        "molecularFormula": "CH3NaO",
                        "density": {
                            "value": 1.3,
                            "unit": "g/mL"
                        }
                    },
                    {
                        "chemicalID": "36",
                        "chemicalName": "theobromine",
                        "CASNumber": "83-67-0",
                        "molecularMass": {
                            "value": 180.160,
                            "unit": "g/mol"
                        },
                        "smiles": "CN1C=NC2=C1C(=O)NC(=O)N2C",
                        "swissCatNumber": "SwissCAT-5429",
                        "keywords": "optional only in HCI file",
                        "Inchi": "InChI=1S/C7H8N4O2/c1-10-3-8-5-4(10)6(12)9-7(13)11(5)2/h3H,1-2H3,(H,9,12,13)",
                        "molecularFormula": "C7H8N4O2",
                        "density": {
                            "value": 1.522,
                            "unit": "g/mL"
                        }
                    },
                    {
                        "chemicalID": "25",
                        "chemicalName": "methyl iodide",
                        "CASNumber": "74-88-4",
                        "molecularMass": {
                            "value": 141.939,
                            "unit": "g/mol"
                        },
                        "smiles": "CI",
                        "swissCatNumber": "SwissCAT-6328",
                        "keywords": "optional only in HCI file",
                        "Inchi": "InChI=1S/CH3I/c1-2/h1H3",
                        "molecularFormula": "CH3I",
                        "density": {
                            "value": 2.28,
                            "unit": "g/mL"
                        }
                    },
                    {
                        "chemicalID": "79",
                        "chemicalName": "methanol",
                        "CASNumber": "67-56-1",
                        "molecularMass": {
                            "value": 32.042,
                            "unit": "g/mol"
                        },
                        "smiles": "CO",
                        "swissCatNumber": "SwissCAT-887",
                        "keywords": "optional only in HCI file",
                        "Inchi": "InChI=1S/CH4O/c1-2/h2H,1H3",
                        "molecularFormula": "CH4O",
                        "density": {
                            "value": 0.79,
                            "unit": "g/mL"
                        }
                    }
                ]
            }
        }
    "#;
    let result = json_to_rdf::<CampaignWrapper>(json_data, &output_format);
    let expected_ttl = r#"
        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
        PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
        PREFIX cat: <http://example.org/cat#>
        PREFIX schema: <https://schema.org/>
        PREFIX unit: <http://qudt.org/vocab/unit/>
        PREFIX allores: <http://purl.allotrope.org/ontologies/result#>
        PREFIX allorole: <http://purl.allotrope.org/ontologies/role#>
        PREFIX alloproc: <http://purl.allotrope.org/ontologies/process#>
        PREFIX allocom: <http://purl.allotrope.org/ontologies/common#>
        PREFIX allohdf: <http://purl.allotrope.org/ontologies/hdf5/1.8#>
        PREFIX allohdfcube: <http://purl.allotrope.org/ontologies/datacube-hdf-map#>
        PREFIX qb: <http://purl.org/linked-data/cube#>
        PREFIX qudt: <http://qudt.org/schema/qudt/>
        PREFIX qudtext: <http://purl.allotrope.org/ontology/qudt-ext/unit#>
        PREFIX alloqual: <http://purl.allotrope.org/ontologies/quality#>
        PREFIX allodc: <http://purl.allotrope.org/ontologies/datacube#>
        PREFIX purl: <http://purl.allotrope.org/ontologies/>
        PREFIX obo: <http://purl.obolibrary.org/obo/>

        [] a cat:Campaign;
        cat:campaignClass "Standard Research";
        cat:campaignType "optimization";
        cat:genericObjective "High caffeine yield at the end";
        cat:hasBatch [ a cat:Batch;
            cat:optimizationType "Yield optimization";
            cat:reactionName "Caffeine synthesis";
            cat:reactionType "N-methylation";
            allohdf:HardLink "https://www.sciencedirect.com/science/article/pii/S0187893X15720926";
            purl:identifier "23";
            schema:name "20240516"];
        cat:hasChemical [ a obo:CHEBI_25367;
            cat:casNumber "74-88-4";
            cat:swissCatNumber "SwissCAT-6328";
            purl:identifier "25";
            allores:AFR_0001952 "CH3I";
            allores:AFR_0002292 "methyl iodide";
            allores:AFR_0002294 [ a cat:Observation;
                qudt:unit unit:GM-PER-MOL;
                qudt:value "141.939"^^xsd:double];
            allores:AFR_0002295 "CI";
            allores:AFR_0002296 "InChI=1S/CH3I/c1-2/h1H3";
            obo:PATO_0001019 [ a cat:Observation;
                qudt:unit unit:GM-PER-MilliL;
                qudt:value "2.28"^^xsd:double];
            schema:keywords "optional only in HCI file"],
            [ a obo:CHEBI_25367;
            cat:casNumber "67-56-1";
            cat:swissCatNumber "SwissCAT-887";
            purl:identifier "79";
            allores:AFR_0001952 "CH4O";
            allores:AFR_0002292 "methanol";
            allores:AFR_0002294 [ a cat:Observation;
                qudt:unit unit:GM-PER-MOL;
                qudt:value "32.042"^^xsd:double];
            allores:AFR_0002295 "CO";
            allores:AFR_0002296 "InChI=1S/CH4O/c1-2/h2H,1H3";
            obo:PATO_0001019 [ a cat:Observation;
                qudt:unit unit:GM-PER-MilliL;
                qudt:value "0.79"^^xsd:double];
            schema:keywords "optional only in HCI file"],
            [ a obo:CHEBI_25367;
            cat:casNumber "83-67-0";
            cat:swissCatNumber "SwissCAT-5429";
            purl:identifier "36";
            allores:AFR_0001952 "C7H8N4O2";
            allores:AFR_0002292 "theobromine";
            allores:AFR_0002294 [ a cat:Observation;
                qudt:unit unit:GM-PER-MOL;
                qudt:value "180.16"^^xsd:double];
            allores:AFR_0002295 "CN1C=NC2=C1C(=O)NC(=O)N2C";
            allores:AFR_0002296 "InChI=1S/C7H8N4O2/c1-10-3-8-5-4(10)6(12)9-7(13)11(5)2/h3H,1-2H3,(H,9,12,13)";
            obo:PATO_0001019 [ a cat:Observation;
                qudt:unit unit:GM-PER-MilliL;
                qudt:value "1.522"^^xsd:double];
            schema:keywords "optional only in HCI file"],
            [ a obo:CHEBI_25367;
            cat:casNumber "124-41-4";
            cat:swissCatNumber "SwissCAT-10942334";
            purl:identifier "19";
            allores:AFR_0001952 "CH3NaO";
            allores:AFR_0002292 "Sodium methoxide";
            allores:AFR_0002294 [ a cat:Observation;
                qudt:unit unit:GM-PER-MOL;
                qudt:value "54.024"^^xsd:double];
            allores:AFR_0002295 "C[O-].[Na+]";
            allores:AFR_0002296 "InChI=1S/CH3O.Na/c1-2;/h1H3;/q-1;+1";
            obo:PATO_0001019 [ a cat:Observation;
                qudt:unit unit:GM-PER-MilliL;
                qudt:value "1.3"^^xsd:double];
            schema:keywords "optional only in HCI file"];
        cat:hasObjective [ a obo:IAO_0000005;
            cat:criteria "Yield ≥ 90%";
            allocom:AFC_0000090 "Reflux in acetone with methyl iodide and potassium carbonate";
            schema:description "Optimize reaction conditions to maximize caffeine yield from theobromine using methyl iodide";
            schema:name "Maximize caffeine formation"];
        allores:AFR_0002764 "Substitution reaction - SN2";
        schema:description "1-step N-methylation of theobromine to caffeine";
        schema:name "Caffeine Synthesis".
    "#;
    let expected_graph = parse_turtle_to_graph(&expected_ttl).unwrap();
    let result_ttl = result.as_ref().unwrap().as_str();
    let result_graph = parse_turtle_to_graph(&result_ttl).unwrap();
    let graphs_match = isomorphic_graphs(&result_graph, &expected_graph);
    assert_eq!(graphs_match.unwrap(), true);
}
