use synth_converter::convert::json_to_turtle;

#[test]
fn test_convert_filtrate_action() {
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
                "subEquipmentName": "Filtration station",
                "containerID": "1",
                "containerBarcode": "1"
            }
        ]
    }
    "#;
    let result = json_to_turtle(json_data);
    assert!(result.is_ok(), "Conversion failed: {:?}", result.err());
}

#[test]
fn test_convert_set_temperature_action() {
    let json_data = r#"
    {
        "batchID": "23",
        "Actions": [
            {
                "actionName": "setTemperatureAction",
                "speedShaker": {
                    "value": 152,
                    "unit": "rpm"
                },
                "temperatureTumbleStirrer": {
                    "value": 25,
                    "unit": "°C"
                },
                "temperatureShaker": {
                    "value": 25,
                    "unit": "°C"
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

    let result = json_to_turtle(json_data);
    assert!(result.is_ok(), "Conversion failed: {:?}", result.err());
}

#[test]
fn test_convert_add_action() {
    let json_data = r#"
    {
        "batchID": "23",
        "Actions": [
            {
                "actionName": "AddAction",
                "speedShaker": {
                    "value": 152,
                    "unit": "rpm"
                },
                "equipmentName": "Chemspeed SWING XL",
                "subEquipmentName": "GDU-V",
                "hasContainerPositionAndQuantity": [
                    {
                        "position": "1A1",
                        "quantity": {
                            "value": 1,
                            "unit": "mg"
                        }
                    },
                    {
                        "position": "1B1",
                        "quantity": {
                            "value": 4,
                            "unit": "mg"
                        }
                    }
                ],
                "startTime": "2024-07-25T12:00:13",
                "endingTime": "2024-07-25T12:00:17",
                "methodName": "addition",
                "dispenseState": "Liquid",
                "dispenseType": "volume",
                "hasSample": {
                    "expectedDatum": {
                        "value": 2,
                        "unit": "mg"
                    },
                    "containerID": "17",
                    "containerBarcode": "17",
                    "vialID": "15",
                    "vialType": "storage vial",
                    "role": "solvent",
                    "hasSample": [
                        {
                            "sampleID": "123",
                            "role": "solvent",
                            "internalBarCode": "1",
                            "measuredQuantity": {
                                "value": "",
                                "unit": "mg"
                            },
                            "physicalState": "Liquid",
                            "concentration": {
                                "value": "",
                                "unit": "mol/L"
                            },
                            "purity": "",
                            "hasChemical": {
                                "chemicalID": "134",
                                "chemicalName": "Toluène",
                                "CASNumber": "108-88-3",
                                "molecularMass": {
                                    "value": 92.14,
                                    "unit": "g/mol"
                                },
                                "smiles": "CC1=CC=CC=C1"
                            }
                        }
                    ]
                }
            }
        ]
    }
    "#;
    let result = json_to_turtle(json_data);
    assert!(result.is_ok(), "Conversion failed: {:?}", result.err());
}

#[test]
fn test_convert_shake_action() {
    let json_data = r#"
    {
        "batchID": "23",
        "Actions": [
            {
                "actionName": "shakeAction",
                "speedTumbleStirrer": {
                    "value": 600,
                    "unit": "rpm"
                },
                "startTime": "2024-07-25T12:03:31",
                "endingTime": "2024-07-25T12:15:20",
                "methodName": "shake",
                "temperatureTumbleStirrer": {
                    "value": 25,
                    "unit": "°C"
                },
                "equipmentName": "Chemspeed SWING XL",
                "subEquipmentName": "Tumble Stirrer",
                "containerID": "1",
                "containerBarcode": "1"
            }
        ]
    }
    "#;

    let result = json_to_turtle(json_data);
    assert!(result.is_ok(), "Conversion failed: {:?}", result.err());
}

#[test]
fn test_convert_set_vacuum_action() {
    let json_data = r#"
    {
        "batchID": "23",
        "Actions": [
            {
                "actionName": "setVacuumAction",
                "vacuum": {
                    "value": 20,
                    "unit": "bar"
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
    let result = json_to_turtle(json_data);
    assert!(result.is_ok(), "Conversion failed: {:?}", result.err());
}
