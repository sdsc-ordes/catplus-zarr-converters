use catplus_common::{models::synth::SynthBatch, rdf::rdf_parser::parse_turtle_to_graph};
use converter::convert::{json_to_rdf, RdfFormat};
use sophia_isomorphism::isomorphic_graphs;

#[test]
fn test_convert_filtrate_action() {
    let output_format = RdfFormat::Turtle;
    let json_data = r#"
        {
            "batchID": "23",
            "Actions": [
                {
                    "actionName": "filtrateAction",
                    "startTime": "2024-07-25T12:15:23",
                    "endingTime": "2024-07-25T12:16:50",
                    "methodName": "filtrate",
                    "equipmentName": "Chemspeed SWING XL",
                    "subEquipmentName": "Filtration unit",
                    "containerID": "1",
                    "containerBarcode": "1"
                }
            ]
        }
    "#;
    let result = json_to_rdf::<SynthBatch>(json_data, &output_format);
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

        [] a cat:FiltrateAction;
        cat:hasBatch [ a cat:Batch;
            purl:identifier "23"];
        cat:hasPlate [ a cat:Plate;
            cat:containerBarcode "1";
            cat:containerID "1"];
        cat:subEquipmentName "Filtration unit";
        allores:AFR_0001606 "filtrate";
        allores:AFR_0001723 "Chemspeed SWING XL";
        allores:AFR_0002423 "2024-07-25T12:16:50"^^xsd:dateTime;
        allores:AFX_0000622 "2024-07-25T12:15:23"^^xsd:dateTime.
    "#;
    let expected_graph = parse_turtle_to_graph(&expected_ttl).unwrap();
    let result_ttl = result.as_ref().unwrap().as_str();
    let result_graph = parse_turtle_to_graph(&result_ttl).unwrap();
    let graphs_match = isomorphic_graphs(&result_graph, &expected_graph);
    assert_eq!(graphs_match.unwrap(), true);
}

#[test]
fn test_convert_pressure_action() {
    let output_format = RdfFormat::Turtle;
    let json_data = r#"
        {
            "batchID": "23",
            "Actions": [
                {
                    "actionName": "setPressureAction",
                    "pressureMeasurement": {
                        "value": 5,
                        "unit": "bar",
                        "errorMargin": {
                            "value": 1,
                            "unit": "bar"
                        }
                    },
                    "startTime": "2024-07-25T12:03:50",
                    "endingTime": "2024-07-25T12:04:05",
                    "methodName": "set_pressure",
                    "equipmentName": "Chemspeed SWING XL",
                    "subEquipmentName": "MTP_Pressure",
                    "containerID": "1",
                    "containerBarcode": "1"
                }
            ]
        }
    "#;
    let result = json_to_rdf::<SynthBatch>(json_data, &output_format);
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

        [] a cat:SetPressureAction;
        cat:hasBatch [ a cat:Batch;
            purl:identifier "23"];
        cat:hasPlate [ a cat:Plate;
            cat:containerBarcode "1";
            cat:containerID "1"];
        cat:subEquipmentName "MTP_Pressure";
        alloproc:AFP_0002677 [ a cat:Observation;
            cat:errorMargin [ a cat:errorMargin;
                qudt:unit unit:Bar;
                qudt:value "1"^^xsd:double];
            qudt:unit unit:Bar;
            qudt:value "5"^^xsd:double];
        allores:AFR_0001606 "set_pressure";
        allores:AFR_0001723 "Chemspeed SWING XL";
        allores:AFR_0002423 "2024-07-25T12:04:05"^^xsd:dateTime;
        allores:AFX_0000622 "2024-07-25T12:03:50"^^xsd:dateTime.
    "#;
    let expected_graph = parse_turtle_to_graph(&expected_ttl).unwrap();
    let result_ttl = result.as_ref().unwrap().as_str();
    let result_graph = parse_turtle_to_graph(&result_ttl).unwrap();
    let graphs_match = isomorphic_graphs(&result_graph, &expected_graph);
    assert_eq!(graphs_match.unwrap(), true);
}

#[test]
fn test_convert_set_temperature_action() {
    let output_format = RdfFormat::Turtle;
    let json_data = r#"
        {
            "batchID": "23",
            "Actions": [
                {
                    "actionName": "setTemperatureAction",
                    "speedShaker": {
                        "value": 152,
                        "unit": "rpm",
                        "errorMargin": {
                            "value": 5,
                            "unit": "rpm"
                        }
                    },
                    "temperatureTumbleStirrer": {
                        "value": 25,
                        "unit": "°C",
                        "errorMargin": {
                            "value": 2,
                            "unit": "°C"
                        }
                    },
                    "temperatureShaker": {
                        "value": 25,
                        "unit": "°C",
                        "errorMargin": {
                            "value": 1,
                            "unit": "°C"
                        }
                    },
                    "startTime": "2024-07-25T12:00:00",
                    "endingTime": "2024-07-25T12:00:02",
                    "methodName": "set_temperature",
                    "equipmentName": "Chemspeed SWING XL",
                    "subEquipmentName": "heater",
                    "containerID": "1",
                    "containerBarcode": "1"
                    }
                ]
            }
        "#;
    let result = json_to_rdf::<SynthBatch>(json_data, &output_format);
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

        [] a cat:SetTemperatureAction;
        cat:hasBatch [ a cat:Batch;
            purl:identifier "23"];
        cat:hasPlate [ a cat:Plate;
            cat:containerBarcode "1";
            cat:containerID "1"];
        cat:speedInRPM [ a cat:Observation;
            cat:errorMargin [ a cat:errorMargin;
                qudt:unit unit:REV-PER-MIN;
                qudt:value "5"^^xsd:double];
            qudt:unit unit:REV-PER-MIN;
            qudt:value "152"^^xsd:double];
        cat:subEquipmentName "heater";
        cat:temperatureShakerShape [ a cat:Observation;
            cat:errorMargin [ a cat:errorMargin;
                qudt:unit unit:DEG-C;
                qudt:value "1"^^xsd:double];
            qudt:unit unit:DEG-C;
            qudt:value "25"^^xsd:double];
        cat:temperatureTumbleStirrerShape [ a cat:Observation;
            cat:errorMargin [ a cat:errorMargin;
                qudt:unit unit:DEG-C;
                qudt:value "2"^^xsd:double];
            qudt:unit unit:DEG-C;
            qudt:value "25"^^xsd:double];
        allores:AFR_0001606 "set_temperature";
        allores:AFR_0001723 "Chemspeed SWING XL";
        allores:AFR_0002423 "2024-07-25T12:00:02"^^xsd:dateTime;
        allores:AFX_0000622 "2024-07-25T12:00:00"^^xsd:dateTime.
        "#;
    let expected_graph = parse_turtle_to_graph(&expected_ttl).unwrap();
    let result_ttl = result.as_ref().unwrap().as_str();
    let result_graph = parse_turtle_to_graph(&result_ttl).unwrap();
    let graphs_match = isomorphic_graphs(&result_graph, &expected_graph);
    assert_eq!(graphs_match.unwrap(), true);
}

#[test]
fn test_convert_add_action() {
    let output_format = RdfFormat::Turtle;
    let json_data = r#"
    {
        "batchID": "23",
        "Actions": [
            {
                "actionName": "AddAction",
                "speedShaker": {
                    "value": 152,
                    "unit": "rpm",
                    "errorMargin": {
                        "value": 1,
                        "unit": "rpm"
                    }
                },
                "equipmentName": "Chemspeed SWING XL",
                "subEquipmentName": "GDU-V",
                "hasWell": [
                    {
                        "position": "A1",
                        "containerID": "1",
                        "quantity": {
                            "value": 0.024,
                            "unit": "mg",
                            "errorMargin": {
                                "value": 0.001,
                                "unit": "mg"
                            }
                        }
                    },
                    {
                        "position": "B1",
                        "containerID": "1",
                        "quantity": {
                            "value": 0.034,
                            "unit": "mg",
                            "errorMargin": {
                                "value": 0.002,
                                "unit": "mg"
                            }
                        }
                    }
                ],
                "startTime": "2024-07-25T12:01:29",
                "endingTime": "2024-07-25T12:01:35",
                "methodName": "addition",
                "dispenseState": "Liquid",
                "dispenseType": "volume",
                "hasSample": {
                    "expectedDatum": {
                        "value": 2,
                        "unit": "mg"
                    },
                    "containerID": "18",
                    "containerBarcode": "18",
                    "vialID": "17",
                    "vialType": "storage vial",
                    "role": "reagent",
                    "hasSample": [
                        {
                            "sampleID": "124",
                            "role": "reagent",
                            "internalBarCode": "2",
                            "expectedDatum": {
                                "value": 5,
                                "unit": "mg"
                            },
                            "measuredQuantity": {
                                "value": 1,
                                "unit": "mg",
                                "errorMargin": {
                                    "value": 0.001,
                                    "unit": "mg"
                                }
                            },
                            "physicalState": "Liquid",
                            "hasChemical": {
                                "chemicalID": "134",
                                "chemicalName": "4-methoxybenzaldehyde",
                                "CASNumber": "123-11-5",
                                "molecularMass": {
                                    "value": 136.15,
                                    "unit": "g/mol"
                                },
                                "smiles": "COC1=CC=C(C=C1)C=O",
                                "Inchi": "1S/C8H8O2/c1-10-8-4-2-7(6-9)3-5-8/h2-6H,1H3",
                                "molecularFormula": "C8H8O2",
                                "density": {
                                    "value": 1.119,
                                    "unit": "g/mL"
                                }
                            }
                        }
                    ]
                }
            }
        ]
    }
    "#;
    let result = json_to_rdf::<SynthBatch>(json_data, &output_format);
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

        [] a cat:AddAction;
        cat:dispenseType "volume";
        cat:hasBatch [ a cat:Batch;
            purl:identifier "23"];
        cat:hasSample [ a cat:Sample;
            cat:expectedDatum [ a cat:Observation;
                qudt:unit unit:MilliGM;
                qudt:value "2"^^xsd:double];
            cat:hasPlate [ a cat:Plate;
                cat:containerBarcode "18";
                cat:containerID "18"];
            cat:hasSample [ a cat:Sample;
                cat:expectedDatum [ a cat:Observation;
                    qudt:unit unit:MilliGM;
                    qudt:value "5"^^xsd:double];
                cat:hasChemical [ a obo:CHEBI_25367;
                    cat:casNumber "123-11-5";
                    purl:identifier "134";
                    allores:AFR_0001952 "C8H8O2";
                    allores:AFR_0002292 "4-methoxybenzaldehyde";
                    allores:AFR_0002294 [ a cat:Observation;
                        qudt:unit unit:GM-PER-MOL;
                        qudt:value "136.15"^^xsd:double];
                    allores:AFR_0002295 "COC1=CC=C(C=C1)C=O";
                    allores:AFR_0002296 "1S/C8H8O2/c1-10-8-4-2-7(6-9)3-5-8/h2-6H,1H3";
                    obo:PATO_0001019 [ a cat:Observation;
                        qudt:unit unit:GM-PER-MilliL;
                        qudt:value "1.119"^^xsd:double]];
                cat:internalBarCode "2";
                cat:measuredQuantity [ a cat:Observation;
                    cat:errorMargin [ a cat:errorMargin;
                        qudt:unit unit:MilliGM;
                        qudt:value "0.001"^^xsd:double];
                    qudt:unit unit:MilliGM;
                    qudt:value "1"^^xsd:double];
                cat:role "reagent";
                purl:identifier "124";
                alloqual:AFQ_0000111 "Liquid"];
            cat:role "reagent";
            cat:vialType "storage vial";
            allores:AFR_0002464 "17"];
        cat:hasWell [ a cat:Well;
            cat:hasPlate [ a cat:Plate;
                cat:containerID "1"];
            allores:AFR_0002240 "B1";
            qudt:quantity [ a cat:Observation;
                cat:errorMargin [ a cat:errorMargin;
                    qudt:unit unit:MilliGM;
                    qudt:value "0.002"^^xsd:double];
                qudt:unit unit:MilliGM;
                qudt:value "0.034"^^xsd:double]],
            [ a cat:Well;
            cat:hasPlate [ a cat:Plate;
                cat:containerID "1"];
            allores:AFR_0002240 "A1";
            qudt:quantity [ a cat:Observation;
                cat:errorMargin [ a cat:errorMargin;
                    qudt:unit unit:MilliGM;
                    qudt:value "0.001"^^xsd:double];
                qudt:unit unit:MilliGM;
                qudt:value "0.024"^^xsd:double]];
        cat:speedInRPM [ a cat:Observation;
            cat:errorMargin [ a cat:errorMargin;
                qudt:unit unit:REV-PER-MIN;
                qudt:value "1"^^xsd:double];
            qudt:unit unit:REV-PER-MIN;
            qudt:value "152"^^xsd:double];
        cat:subEquipmentName "GDU-V";
        alloqual:AFQ_0000111 "Liquid";
        allores:AFR_0001606 "addition";
        allores:AFR_0001723 "Chemspeed SWING XL";
        allores:AFR_0002423 "2024-07-25T12:01:35"^^xsd:dateTime;
  allores:AFX_0000622 "2024-07-25T12:01:29"^^xsd:dateTime.
    "#;
    let expected_graph = parse_turtle_to_graph(&expected_ttl).unwrap();
    let result_ttl = result.as_ref().unwrap().as_str();
    let result_graph = parse_turtle_to_graph(&result_ttl).unwrap();
    let graphs_match = isomorphic_graphs(&result_graph, &expected_graph);
    assert_eq!(graphs_match.unwrap(), true);
}

#[test]
fn test_convert_shake_action() {
    let output_format = RdfFormat::Turtle;
    let json_data = r#"
        {
            "batchID": "23",
            "Actions": [
                {
                    "actionName": "shakeAction",
                    "speedTumbleStirrer": {
                        "value": 600,
                        "unit": "rpm",
                        "errorMargin": {
                            "value": 1,
                            "unit": "rpm"
                        }
                    },
                    "startTime": "2024-07-25T12:03:31",
                    "endingTime": "2024-07-25T12:15:20",
                    "methodName": "shake",
                    "temperatureTumbleStirrer": {
                        "value": 25,
                        "unit": "°C",
                        "errorMargin": {
                            "value": 1,
                            "unit": "°C"
                        }
                    },
                    "temperatureShaker": {
                        "value": 25,
                        "unit": "°C",
                        "errorMargin": {
                            "value": 2,
                            "unit": "°C"
                        }
                    },
                    "equipmentName": "Chemspeed SWING XL",
                    "subEquipmentName": "Tumble Stirrer",
                    "containerID": "1",
                    "containerBarcode": "1"
                }
            ]
        }
    "#;
    let result = json_to_rdf::<SynthBatch>(json_data, &output_format);
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

        [] a cat:ShakeAction;
        cat:hasBatch [ a cat:Batch;
            purl:identifier "23"];
        cat:hasPlate [ a cat:Plate;
            cat:containerBarcode "1";
            cat:containerID "1"];
        cat:speedTumbleStirrerShape [ a cat:Observation;
            cat:errorMargin [ a cat:errorMargin;
                qudt:unit unit:REV-PER-MIN;
                qudt:value "1"^^xsd:double];
            qudt:unit unit:REV-PER-MIN;
            qudt:value "600"^^xsd:double];
        cat:subEquipmentName "Tumble Stirrer";
        cat:temperatureShakerShape [ a cat:Observation;
            cat:errorMargin [ a cat:errorMargin;
                qudt:unit unit:DEG-C;
                qudt:value "2"^^xsd:double];
            qudt:unit unit:DEG-C;
            qudt:value "25"^^xsd:double];
        cat:temperatureTumbleStirrerShape [ a cat:Observation;
            cat:errorMargin [ a cat:errorMargin;
                qudt:unit unit:DEG-C;
                qudt:value "1"^^xsd:double];
            qudt:unit unit:DEG-C;
            qudt:value "25"^^xsd:double];
        allores:AFR_0001606 "shake";
        allores:AFR_0001723 "Chemspeed SWING XL";
        allores:AFR_0002423 "2024-07-25T12:15:20"^^xsd:dateTime;
        allores:AFX_0000622 "2024-07-25T12:03:31"^^xsd:dateTime.
    "#;
    let expected_graph = parse_turtle_to_graph(&expected_ttl).unwrap();
    let result_ttl = result.as_ref().unwrap().as_str();
    let result_graph = parse_turtle_to_graph(&result_ttl).unwrap();
    let graphs_match = isomorphic_graphs(&result_graph, &expected_graph);
    assert_eq!(graphs_match.unwrap(), true);
}

#[test]
fn test_convert_set_vacuum_action() {
    let output_format = RdfFormat::Turtle;
    let json_data = r#"
        {
            "batchID": "23",
            "Actions": [
                {
                    "actionName": "setVacuumAction",
                    "vacuum": {
                        "value": 20,
                        "unit": "bar",
                        "errorMargin": {
                            "value": 0.5,
                            "unit": "bar"
                        }
                    },
                    "startTime": "2024-07-25T12:03:41",
                    "endingTime": "2024-07-25T12:03:50",
                    "methodName": "set_vacuum",
                    "equipmentName": "Chemspeed SWING XL",
                    "subEquipmentName": "vacuum",
                    "containerID": "1",
                    "containerBarcode": "1"
                }
            ]
        }
    "#;
    let result = json_to_rdf::<SynthBatch>(json_data, &output_format);
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

        [] a cat:SetVacuumAction;
        cat:hasBatch [ a cat:Batch;
            purl:identifier "23"];
        cat:hasPlate [ a cat:Plate;
            cat:containerBarcode "1";
            cat:containerID "1"];
        cat:subEquipmentName "vacuum";
        cat:vacuum [ a cat:Observation;
            cat:errorMargin [ a cat:errorMargin;
                qudt:unit unit:Bar;
                qudt:value "0.5"^^xsd:double];
            qudt:unit unit:Bar;
            qudt:value "20"^^xsd:double];
        allores:AFR_0001606 "set_vacuum";
        allores:AFR_0001723 "Chemspeed SWING XL";
        allores:AFR_0002423 "2024-07-25T12:03:50"^^xsd:dateTime;
        allores:AFX_0000622 "2024-07-25T12:03:41"^^xsd:dateTime.
    "#;
    let expected_graph = parse_turtle_to_graph(&expected_ttl).unwrap();
    let result_ttl = result.as_ref().unwrap().as_str();
    let result_graph = parse_turtle_to_graph(&result_ttl).unwrap();
    let graphs_match = isomorphic_graphs(&result_graph, &expected_graph);
    assert_eq!(graphs_match.unwrap(), true);
}
