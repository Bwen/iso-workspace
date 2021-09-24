mod subdivisions {
    extern crate riso_3166;
    use riso_3166::country::Alpha2;
    use riso_3166::subdivision::{Subdivision, Code};
    use riso_static_traits::{FindFor, IterFor, TryFor};

    #[cfg(test)]
    use serde::{Deserialize, Serialize};

    #[cfg(test)]
    use serde_json::Result;

    //#[test]
    //fn sanity_checks() {
    //    let total = Subdivision::count();
        // We plus 1 because of the Code::None that is added
    //    assert_eq!(Code::iter().count(), total + 1);
    //}

    #[test]
    fn subdivision_find_for() {
        let subdivision = Subdivision::find_for(Code::CA_QC);
        assert_eq!(subdivision.country, Alpha2::CA);
        assert_eq!(subdivision.category, "province");
        assert_eq!(subdivision.name, "Quebec");
        assert_eq!(subdivision.parent, Code::None);
    }

    #[test]
    fn subdivision_try_for_code() {
        let subdivision = Subdivision::try_for("CA-QC").unwrap();
        assert_eq!(subdivision.country, Alpha2::CA);
        assert_eq!(subdivision.category, "province");
        assert_eq!(subdivision.name, "Quebec");
        assert_eq!(subdivision.parent, Code::None);
    }

    #[test]
    fn subdivision_try_for_name() {
        let subdivision = Subdivision::try_for("Paul").unwrap();
        assert_eq!(subdivision.country, Alpha2::CV);
        assert_eq!(subdivision.category, "municipality");
        assert_eq!(subdivision.name, "Paul");
        assert_eq!(subdivision.code, Code::CV_PA);
        assert_eq!(subdivision.parent, Code::CV_B);

        let subdivision = Subdivision::try_for("Bas-Uélé").unwrap();
        assert_eq!(subdivision.country, Alpha2::CD);
        assert_eq!(subdivision.category, "province");
        assert_eq!(subdivision.name, "Bas-Uélé");
        assert_eq!(subdivision.code, Code::CD_BU);
        assert_eq!(subdivision.parent, Code::None);

        let subdivision = Subdivision::try_for("Es-Semara").unwrap();
        assert_eq!(subdivision.country, Alpha2::MA);
        assert_eq!(subdivision.category, "province");
        assert_eq!(subdivision.name, "Es-Semara");
        assert_eq!(subdivision.code, Code::MA_ESM);
        assert_eq!(subdivision.parent, Code::MA_11);


        let subdivision = Subdivision::try_for("CA-");
        assert!(subdivision.is_err());
    }

    #[test]
    fn subdivision_try_for_error() {
        let subdivisions = Subdivision::try_for("ZZ-ZZZ");
        assert!(subdivisions.is_err());
    }

    #[test]
    fn subdivision_iter_for_parent() {
        let subdivisions = Subdivision::iter_for(Code::FR_ARA);
        let codes: Vec<&Code> = subdivisions.iter().map(|n| &n.code).collect();
        assert!(codes.contains(&&Code::FR_01));
        assert!(codes.contains(&&Code::FR_03));
        assert!(codes.contains(&&Code::FR_07));
        assert!(codes.contains(&&Code::FR_15));
        assert!(codes.contains(&&Code::FR_26));
        assert!(codes.contains(&&Code::FR_38));
        assert!(codes.contains(&&Code::FR_42));
        assert!(codes.contains(&&Code::FR_43));
        assert!(codes.contains(&&Code::FR_63));
        assert!(codes.contains(&&Code::FR_69));
        assert!(codes.contains(&&Code::FR_73));
        assert!(codes.contains(&&Code::FR_74));
    }

    #[test]
    fn subdivision_iter_for_alpha2() {
        let subdivisions = Subdivision::iter_for(Alpha2::CA);
        assert_eq!(subdivisions.len(), 13);

        for subdivision in subdivisions {
            if subdivision.code == Code::CA_QC {
                assert_eq!(subdivision.country, Alpha2::CA);
                assert_eq!(subdivision.category, "province");
                assert_eq!(subdivision.name, "Quebec");
                assert_eq!(subdivision.parent, Code::None);
            }
        }
    }

    #[test]
    fn subdivision_parents() {
        let parents = Subdivision::parents();
        for parent in parents {
            assert!(parent.parent.is_none());
        }
    }

    #[test]
    fn subdivision_all_and_count() {
        let subdivisions = Subdivision::all();
        assert_eq!(subdivisions.count(), Subdivision::count());
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct SubdivisionSerde {
        text: String,
        subdiv: Subdivision,
    }

    #[test]
    fn serde_deserialize() -> Result<()> {
        let json = r#"{"text": "test", "subdiv": "CA-QC"}"#;
        let test_deserialize: SubdivisionSerde = serde_json::from_str(json)?;
        assert_eq!(test_deserialize.text, "test");
        assert_eq!(test_deserialize.subdiv.country, Alpha2::CA);
        assert_eq!(test_deserialize.subdiv.code, Code::CA_QC);
        assert_eq!(test_deserialize.subdiv.parent, Code::None);
        assert_eq!(test_deserialize.subdiv.name, "Quebec");
        assert_eq!(test_deserialize.subdiv.category, "province");

        Ok(())
    }

    #[test]
    fn serde_deserialize_error() -> Result<()> {
        let json = r#"{"text": "test", "subdiv": "USDSF"}"#;
        let result: Result<Subdivision> = serde_json::from_str(json);
        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn serde_serialize() -> Result<()> {
        let subdivision = Subdivision::find_for(Code::CA_QC);
        let test_serialize = SubdivisionSerde {
            text: String::from("test"),
            subdiv: *subdivision,
        };

        let test: String = serde_json::to_string(&test_serialize).unwrap();
        assert_eq!(test, "{\"text\":\"test\",\"subdiv\":{\"country\":\"CA\",\"code\":\"CA-QC\",\"parent\":\"\",\"name\":\"Quebec\",\"category\":\"province\"}}");

        Ok(())
    }

}
