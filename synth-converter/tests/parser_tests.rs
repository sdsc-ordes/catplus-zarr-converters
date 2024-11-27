use synth_converter::parser::parser::parse_json;

#[test]
fn test_parse_filtrate_action() {
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

    let result = parse_json(json_data);
    assert!(result.is_ok(), "Parsing failed: {:?}", result.err());
}

