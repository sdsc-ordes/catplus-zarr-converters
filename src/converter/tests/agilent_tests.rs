use catplus_common::{
    models::agilent::LiquidChromatographyAggregateDocumentWrapper,
    rdf::rdf_parser::parse_turtle_to_graph,
};
use converter::convert::{json_to_rdf, RdfFormat};
use sophia_isomorphism::isomorphic_graphs;

#[test]
fn test_convert_liquid_chromatography() {
    let output_format = RdfFormat::Turtle;
    let json_data = r#"
    {
        "liquid chromatography aggregate document": {
            "liquid chromatography document": [
                {
                    "analyst": "Swisscat (swisscat)",
                    "measurement aggregate document": {
                        "measurement document": [
                            {
                                "measurement identifier": "DAD1A",
                                "chromatography column document": "temporary",
                                "device control aggregate document": {
                                    "device control document": [
                                        {
                                            "device identifier": "",
                                            "device type": "Diode array uv detector",
                                            "product manufacturer": "Agilent",
                                            "equipment serial number": "DEAC617961",
                                            "model number": "G7115A",
                                            "firmware version": "D.07.38 [0001]",
                                            "detection type": "single channel"
                                        }
                                    ]
                                },
                                "sample document": {
                                    "sample identifier": "0659d110-49d0-4e98-8f3a-1aaf9c4ec0d9",
                                    "written name": "1-4 PYRIDYL PIPERAZINE-2024-04-12 10-23-04+02-00-20.dx"
                                },
                                "injection document": {
                                    "autosampler injection volume setting (chromatography)": {
                                        "value": 5,
                                        "unit": "mm^3"
                                    },
                                    "injection identifier": "2024-04-12 10-23-04+02-00-20.dx",
                                    "injection time": "2024-04-12T08:23:47.113+00:00"
                                },
                                "detection type": "single channel",
                                "chromatogram data cube": {
                                    "label": "DAD1A,Sig=215,4  Ref=off",
                                    "cube-structure": {
                                        "dimensions": [
                                            {
                                                "@componentDatatype": "double",
                                                "concept": "retention time",
                                                "unit": "s"
                                            }
                                        ],
                                        "measures": [
                                            {
                                                "@componentDatatype": "double",
                                                "concept": "absorbance",
                                                "unit": "mAU"
                                            }
                                        ]
                                    },
                                    "data": {
                                        "measures": [
                                            [
                                                
                                                -0.870228
                                            ]
                                        ],
                                        "dimensions": [
                                            [
                                                0.2
                                            
                                            ]
                                        ]
                                    },
                                    "identifier": "DAD1A"
                                },
                                "processed data document": {
                                    "peak list": {
                                        "peak": [
                                            {
                                                "@index": 1,
                                                "peak area": {
                                                    "value": 34034.5,
                                                    "unit": "mAU.s"
                                                },
                                                "retention time": {
                                                    "value": 1.19008,
                                                    "unit": "min"
                                                },
                                                "peakIdentifier": "f81b4bcb-4d4a-41c7-8b34-5610e940d3ca",
                                                "peak end": {
                                                    "value": 1.68996,
                                                    "unit": "min"
                                                },
                                                "relative peak height": {
                                                    "value": 100,
                                                    "unit": "%"
                                                },
                                                "peak height": {
                                                    "value": 3058.31,
                                                    "unit": "mAU"
                                                },
                                                "peak start": {
                                                    "value": 0.984987,
                                                    "unit": "min"
                                                },
                                                "relative peak area": {
                                                    "value": 100,
                                                    "unit": "%"
                                                },
                                                "peak value at start": {
                                                    "value": -169.679,
                                                    "unit": "mAU"
                                                },
                                                "peak value at end": {
                                                    "value": -183.143,
                                                    "unit": "mAU"
                                                }
                                            }
                                        ]
                                    }
                                }
                            },
                            {
                                "measurement identifier": "DAD1B",
                                "chromatography column document": {},
                                "device control aggregate document": {
                                    "device control document": [
                                        {
                                            "device identifier": "",
                                            "device type": "Diode array uv detector",
                                            "product manufacturer": "Agilent",
                                            "equipment serial number": "DEAC617961",
                                            "model number": "G7115A",
                                            "firmware version": "D.07.38 [0001]",
                                            "detection type": "single channel"
                                        }
                                    ]
                                },
                                "sample document": {
                                    "sample identifier": "0659d110-49d0-4e98-8f3a-1aaf9c4ec0d9",
                                    "written name": "1-4 PYRIDYL PIPERAZINE-2024-04-12 10-23-04+02-00-20.dx"
                                },
                                "injection document": {
                                    "autosampler injection volume setting (chromatography)": {
                                        "value": 5,
                                        "unit": "mm^3"
                                    },
                                    "injection identifier": "2024-04-12 10-23-04+02-00-20.dx",
                                    "injection time": "2024-04-12T08:23:47.113+00:00"
                                },
                                "detection type": "single channel",
                                "chromatogram data cube": {
                                    "label": "DAD1B,Sig=254,4  Ref=off",
                                    "cube-structure": {
                                        "dimensions": [
                                            {
                                                "@componentDatatype": "double",
                                                "concept": "retention time",
                                                "unit": "s"
                                            }
                                        ],
                                        "measures": [
                                            {
                                                "@componentDatatype": "double",
                                                "concept": "absorbance",
                                                "unit": "mAU"
                                            }
                                        ]
                                    },
                                    "data": {
                                        "measures": [
                                            [
                                                0.130653
                                                
                                            ]
                                        ],
                                        "dimensions": [
                                            [
                                                0.2
                                                
                                            ]
                                        ]
                                    },
                                    "identifier": "DAD1B"
                                },
                                "processed data document": {
                                    "peak list": {
                                        "peak": []
                                    }
                                }
                            }
                        ]
                    }
                }
            ]
        }
    }
    "#;
    let result = json_to_rdf::<LiquidChromatographyAggregateDocumentWrapper>(json_data, &output_format);
    println!("{:?}", result);
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

    [] a allores:AFR_0002524;
    cat:hasLiquidChromatography [ a allores:AFR_0002525;
        allores:AFR_0001116 "Swisscat (swisscat)";
        allores:AFR_0002374 [ a allores:AFR_0002375;
            allores:AFR_0001121 "DAD1B";
            allores:AFR_0002083 [ a cat:SampleDocument;
                allores:AFR_0001118 "0659d110-49d0-4e98-8f3a-1aaf9c4ec0d9";
                obo:IAO_0000590 "1-4 PYRIDYL PIPERAZINE-2024-04-12 10-23-04+02-00-20.dx"];
            allores:AFR_0002526 [ a cat:DeviceSystemDocument;
                allores:AFR_0002722 [ a allores:AFR_0002567;
                    allores:AFR_0001119 "DEAC617961";
                    allores:AFR_0001258 "Agilent";
                    allores:AFR_0001259 "D.07.38 [0001]";
                    allores:AFR_0002018 "";
                    allores:AFR_0002534 "single channel";
                    allores:AFR_0002568 "Diode array uv detector";
                    obo:IAO_0000017 "G7115A"]];
            allores:AFR_0002529 [ a cat:InjectionDocument;
                allores:AFR_0001267 [ a cat:AutosamplerInjectionVolumeSetting;
                    qudt:unit unit:MilliM3;
                    qudt:value "5"^^xsd:double];
                allores:AFR_0002535 "2024-04-12 10-23-04+02-00-20.dx";
                allores:AFR_0002536 "2024-04-12T08:23:47.113+00:00"^^xsd:dateTime];
            allores:AFR_0002534 "single channel";
            allores:AFR_0002550 [ a cat:ChromatogramDataCube;
                allores:AFR_0000917 "DAD1B";
                obo:IAO_0000009 "DAD1B,Sig=254,4  Ref=off";
                qb:structure [ a cat:CubeStructure;
                    cat:dimension [ a cat:Dimension;
                        allodc:componentDataType "double";
                        qudt:unit unit:SEC;
                        <http://www.w3.org/2000/01/rdf-schema#label> "retention time"];
                    cat:measure [ a allorole:AFRL_0000157;
                        allodc:componentDataType "double";
                        qudt:unit qudtext:MilliAbsorbanceUnit;
                        <http://www.w3.org/2000/01/rdf-schema#label> "absorbance"]]];
            allores:AFR_0002659 [ a cat:ProcessedDataDocument;
                allores:AFR_0000432 [ a cat:PeakList]]],
            [ a allores:AFR_0002375;
            allores:AFR_0001121 "DAD1A";
            allores:AFR_0002083 [ a cat:SampleDocument;
                allores:AFR_0001118 "0659d110-49d0-4e98-8f3a-1aaf9c4ec0d9";
                obo:IAO_0000590 "1-4 PYRIDYL PIPERAZINE-2024-04-12 10-23-04+02-00-20.dx"];
            allores:AFR_0002526 [ a cat:DeviceSystemDocument;
                allores:AFR_0002722 [ a allores:AFR_0002567;
                    allores:AFR_0001119 "DEAC617961";
                    allores:AFR_0001258 "Agilent";
                    allores:AFR_0001259 "D.07.38 [0001]";
                    allores:AFR_0002018 "";
                    allores:AFR_0002534 "single channel";
                    allores:AFR_0002568 "Diode array uv detector";
                    obo:IAO_0000017 "G7115A"]];
            allores:AFR_0002529 [ a cat:InjectionDocument;
                allores:AFR_0001267 [ a cat:AutosamplerInjectionVolumeSetting;
                    qudt:unit unit:MilliM3;
                    qudt:value "5"^^xsd:double];
                allores:AFR_0002535 "2024-04-12 10-23-04+02-00-20.dx";
                allores:AFR_0002536 "2024-04-12T08:23:47.113+00:00"^^xsd:dateTime];
            allores:AFR_0002534 "single channel";
            allores:AFR_0002550 [ a cat:ChromatogramDataCube;
                allores:AFR_0000917 "DAD1A";
                obo:IAO_0000009 "DAD1A,Sig=215,4  Ref=off";
                qb:structure [ a cat:CubeStructure;
                    cat:dimension [ a cat:Dimension;
                        allodc:componentDataType "double";
                        qudt:unit unit:SEC;
                        <http://www.w3.org/2000/01/rdf-schema#label> "retention time"];
                    cat:measure [ a allorole:AFRL_0000157;
                        allodc:componentDataType "double";
                        qudt:unit qudtext:MilliAbsorbanceUnit;
                        <http://www.w3.org/2000/01/rdf-schema#label> "absorbance"]]];
            allores:AFR_0002659 [ a cat:ProcessedDataDocument;
                allores:AFR_0000432 [ a cat:PeakList;
                    cat:Peak [ a allores:AFR_0000413;
                        allores:AFR_0000948 [ a cat:Measurement;
                            qudt:unit qudtext:MilliAbsorbanceUnit;
                            qudt:value "3058.31"^^xsd:double];
                        allores:AFR_0000949 [ a cat:Measurement;
                            qudt:unit unit:PERCENT;
                            qudt:value "100"^^xsd:double];
                        allores:AFR_0001073 [ a cat:Measurement;
                            qudt:unit qudtext:MilliAbsorbanceUnitTimesSecond;
                            qudt:value "34034.5"^^xsd:double];
                        allores:AFR_0001089 [ a cat:Measurement;
                            qudt:unit unit:MIN;
                            qudt:value "1.19008"^^xsd:double];
                        allores:AFR_0001164 "f81b4bcb-4d4a-41c7-8b34-5610e940d3ca";
                        allores:AFR_0001165 [ a cat:Measurement;
                            qudt:unit unit:PERCENT;
                            qudt:value "100"^^xsd:double];
                        allores:AFR_0001178 [ a cat:Measurement;
                            qudt:unit unit:MIN;
                            qudt:value "0.984987"^^xsd:double];
                        allores:AFR_0001179 [ a cat:Measurement;
                            qudt:unit qudtext:MilliAbsorbanceUnit;
                            qudt:value "-169.679"^^xsd:double];
                        allores:AFR_0001180 [ a cat:Measurement;
                            qudt:unit unit:MIN;
                            qudt:value "1.68996"^^xsd:double];
                        allores:AFR_0001181 [ a cat:Measurement;
                            qudt:unit qudtext:MilliAbsorbanceUnit;
                            qudt:value "-183.143"^^xsd:double]]]]]].

      "#;
    let expected_graph = parse_turtle_to_graph(&expected_ttl).unwrap();
    let result_ttl = result.as_ref().unwrap().as_str();
    let result_graph = parse_turtle_to_graph(&result_ttl).unwrap();
    let graphs_match = isomorphic_graphs(&result_graph, &expected_graph);
    assert_eq!(graphs_match.unwrap(), true);
}

#[test]
fn test_convert_device_system_document() {
    let output_format = RdfFormat::Turtle;
    let json_data = r#"
    {
    "liquid chromatography aggregate document": {
        "device system document": {
            "asset management identifier": "a7155146-e1d0-41be-99bf-eb2e55f9766e",
            "device document": [
                {
                    "device identifier": "LC Pump",
                    "device type": "Pump",
                    "model number": "G7104C",
                    "product manufacturer": "Agilent",
                    "equipment serial number": "DEAGZ02881",
                    "firmware version": "B.07.38 [0003]",
                    "@index": 1
                },
                {
                    "device identifier": "Sampler",
                    "device type": "Autosampler",
                    "model number": "G7167A",
                    "product manufacturer": "Agilent",
                    "equipment serial number": "DEAGW00219",
                    "firmware version": "D.07.38 [0003]",
                    "@index": 2
                }
            ]
            }
        }
    }
    "#;
    let result = json_to_rdf::<LiquidChromatographyAggregateDocumentWrapper>(json_data, &output_format);
    let expected_ttl = r#"
    PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
    PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
    PREFIX cat: <http://example.org/cat#>
    PREFIX schema: <https://schema.org/>
    PREFIX unit: <https://qudt.org/vocab/unit/>
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

    [] a allores:AFR_0002524;
    allores:AFR_0002526 [ a cat:DeviceSystemDocument;
        allores:AFR_0001976 "a7155146-e1d0-41be-99bf-eb2e55f9766e";
        allores:AFR_0002722 [ a allores:AFR_0002567;
            allores:AFR_0001119 "DEAGZ02881";
            allores:AFR_0001258 "Agilent";
            allores:AFR_0001259 "B.07.38 [0003]";
            allores:AFR_0002018 "LC Pump";
            allores:AFR_0002568 "Pump";
            obo:IAO_0000017 "G7104C"],
            [ a allores:AFR_0002567;
            allores:AFR_0001119 "DEAGW00219";
            allores:AFR_0001258 "Agilent";
            allores:AFR_0001259 "D.07.38 [0003]";
            allores:AFR_0002018 "Sampler";
            allores:AFR_0002568 "Autosampler";
            obo:IAO_0000017 "G7167A"]].

    "#;
    let expected_graph = parse_turtle_to_graph(&expected_ttl).unwrap();
    let result_ttl = result.as_ref().unwrap().as_str();
    let result_graph = parse_turtle_to_graph(&result_ttl).unwrap();
    let graphs_match = isomorphic_graphs(&result_graph, &expected_graph);
    assert_eq!(graphs_match.unwrap(), true);
}
