#[cfg(feature = "serde")]
mod serde_tests {
    use metar::{CloudDensity, Metar};
    use serde_json;

    #[test]
    fn test_metar_serialize_deserialize() {
        let metar_str = "EGHI 282120Z 19015KT 140V220 6000 RA SCT006 BKN009 16/14 Q1006";
        let metar = Metar::parse(metar_str).unwrap();

        // Test serialization
        let json = serde_json::to_string(&metar).expect("Failed to serialize METAR");
        println!("Serialized METAR: {}", json);

        // Test deserialization
        let deserialized: Metar = serde_json::from_str(&json).expect("Failed to deserialize METAR");

        // Verify the deserialized object matches the original
        assert_eq!(metar, deserialized);
    }

    #[test]
    fn test_metar_serialize_pretty() {
        let metar_str = "KJFK 121251Z 24016G24KT 10SM FEW250 28/23 A2996 RMK AO2 SLP142 T02830228";
        let metar = Metar::parse(metar_str).unwrap();

        // Test pretty serialization
        let json = serde_json::to_string_pretty(&metar).expect("Failed to serialize METAR pretty");
        println!("Pretty serialized METAR:\n{}", json);

        // Ensure it's valid JSON that can be deserialized
        let _: Metar = serde_json::from_str(&json).expect("Failed to deserialize pretty JSON");
    }

    #[test]
    fn test_individual_types_serialization() {
        use metar::{CloudLayer, CloudType, Data, Visibility, WindSpeed};

        // Test Data<T> serialization
        let known_visibility = Data::Known(Visibility::Metres(6000));
        let unknown_visibility = Data::<Visibility>::Unknown;

        let known_json = serde_json::to_string(&known_visibility).unwrap();
        let unknown_json = serde_json::to_string(&unknown_visibility).unwrap();

        assert!(known_json.contains("Known"));
        assert!(known_json.contains("6000"));
        assert_eq!(unknown_json, "\"Unknown\"");

        // Test WindSpeed serialization
        let wind_speed = WindSpeed::Knots {
            speed: Data::Known(15),
            gusting: None,
        };
        let wind_json = serde_json::to_string(&wind_speed).unwrap();
        assert!(wind_json.contains("Knot"));
        assert!(wind_json.contains("15"));

        // Test CloudLayer serialization
        let cloud_layer = CloudLayer {
            density: Data::Known(CloudDensity::Scattered),
            kind: Data::Known(CloudType::Normal),
            height: Data::Known(600),
        };
        let cloud_json = serde_json::to_string(&cloud_layer).unwrap();
        assert!(cloud_json.contains("Scattered"));
        assert!(cloud_json.contains("600"));
    }

    #[test]
    fn test_roundtrip_multiple_metars() {
        let test_metars = vec![
            "EGHI 282120Z 19015KT 140V220 6000 RA SCT006 BKN009 16/14 Q1006",
            "KJFK 121251Z 24016G24KT 10SM FEW250 28/23 A2996 RMK AO2 SLP142 T02830228",
            "KORD 121856Z 09014KT 10SM CLR 32/23 A2992 RMK AO2 SLP131 T03220228",
        ];

        for metar_str in test_metars {
            let metar = Metar::parse(metar_str).unwrap();
            let json = serde_json::to_string(&metar).unwrap();
            let deserialized: Metar = serde_json::from_str(&json).unwrap();
            assert_eq!(metar, deserialized, "Roundtrip failed for: {}", metar_str);
        }
    }
}

#[cfg(not(feature = "serde"))]
mod no_serde_tests {
    #[test]
    fn test_serde_feature_disabled() {
        // This test just ensures the crate compiles without serde feature
        use metar::Metar;
        let metar_str = "EGHI 282120Z 19015KT 140V220 6000 RA SCT006 BKN009 16/14 Q1006";
        let _metar = Metar::parse(metar_str).unwrap();
        println!("METAR parsing works without serde feature");
    }
}
