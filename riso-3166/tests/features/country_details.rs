mod country_details {
    extern crate riso_3166;
    use riso_static_traits::{TryFor, FindFor, IterFor};
    use riso_3166::country::{Country, Detail, Alpha2, Tld};
    use riso_3166::continent::{Continent, Code};

    #[cfg(test)]
    use serde::{Deserialize, Serialize};

    #[cfg(test)]
    use serde_json::Result;

    #[test]
    fn find_for() {
        let detail = Detail::find_for(Alpha2::CA);
        assert_eq!(Alpha2::CA, detail.alpha2);
        assert_eq!(Tld::CA, detail.tld);
        assert!(detail.population > 30_000_000 as usize);
        assert_eq!("1", detail.phone_prefix);
        assert_eq!(Code::NA, detail.continent().code);
        assert_eq!("CAD", detail.currency);
    }

    #[test]
    fn iter_for_population_greater() {
        let countries = Country::iter_for_population_greater(200_000_000 as usize);
        for country in &countries {
            assert!(country.details().population > 200_000_000 as usize);
        }

        let alpha2s: Vec<&Alpha2> = countries.iter().map(|n| &n.alpha2).collect();
        assert!(alpha2s.contains(&&Alpha2::BR));
        assert!(alpha2s.contains(&&Alpha2::CN));
        assert!(alpha2s.contains(&&Alpha2::ID));
        assert!(alpha2s.contains(&&Alpha2::IN));
        assert!(alpha2s.contains(&&Alpha2::US));
    }

    #[test]
    fn country_neightboors() {
        let country = Country::find_for(Alpha2::MX);
        let neightboors = country.neightboors();
        assert_eq!(3, neightboors.len());

        let alpha2s: Vec<&Alpha2> = neightboors.iter().map(|n| &n.alpha2).collect();
        assert!(alpha2s.contains(&&Alpha2::BZ));
        assert!(alpha2s.contains(&&Alpha2::GT));
        assert!(alpha2s.contains(&&Alpha2::US));
    }

    #[test]
    fn country_languages() {
        let country = Country::find_for(Alpha2::CA);
        let languages = country.languages();
        assert_eq!(3, languages.len());
    }

    #[test]
    fn country_continent() {
        let country = Country::find_for(Alpha2::CA);
        let continent = country.continent();
        assert_eq!(continent, Continent::find_for(Code::NA));
    }

    #[test]
    fn details_iter_for_continent() {
        let continent = Continent::find_for(Code::NA);
        let details = Detail::iter_for(*continent);
        assert_eq!(details.len(), 42);

        for detail in details {
            assert_eq!(detail.continent(), continent);

            if detail.alpha2 == Alpha2::CA {
                assert_eq!(detail.tld, Tld::CA);
                assert_eq!(detail.phone_prefix, "1");
                assert_eq!(detail.currency, "CAD");
            }
        }
    }

    #[test]
    fn details_iter_try_for_country_alpha2() {
        let detail = Detail::try_for("CA").unwrap();
        assert_eq!(detail.continent(), Continent::find_for(Code::NA));
        assert_eq!(detail.tld, Tld::CA);
        assert_eq!(detail.phone_prefix, "1");
        assert_eq!(detail.currency, "CAD");
    }

    #[test]
    fn details_iter_try_for_country_alpha2_error() {
        let detail = Detail::try_for("ZZ");
        assert!(detail.is_err());
    }

    #[test]
    fn tld_is_none() {
        let detail = Detail::try_for("SS").unwrap();
        assert!(detail.tld.is_none());
    }

    #[test]
    fn continent_all() {
        let continents = Continent::all();
        assert_eq!(continents.len(), 7);
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct DetailSerde {
        text: String,
        detail: Detail,
    }

    #[test]
    fn serde_deserialize_country() -> Result<()> {
        let json = r#"{"text": "test", "detail": "US"}"#;
        let test_deserialize: DetailSerde = serde_json::from_str(json)?;
        assert_eq!(test_deserialize.text, "test");
        assert_eq!(test_deserialize.detail.alpha2, Alpha2::US);
        assert_eq!(test_deserialize.detail.tld, Tld::US);
        assert_eq!(test_deserialize.detail.phone_prefix, "1");
        assert_eq!(test_deserialize.detail.currency, "USD");
        assert_eq!(test_deserialize.detail.continent(), Continent::find_for(Code::NA));

        Ok(())
    }

    #[test]
    fn serde_deserialize_country_error() -> Result<()> {
        let json = r#"{"text": "test", "detail": "USDSF"}"#;
        let result: Result<Country> = serde_json::from_str(json);
        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn serde_serialize_country() -> Result<()> {
        let detail = Detail::find_for(Alpha2::US);
        let test_serialize = DetailSerde {
            text: String::from("test"),
            detail: *detail,
        };

        let test: String = serde_json::to_string(&test_serialize).unwrap();
        assert_eq!(test, "{\"text\":\"test\",\"detail\":{\"alpha2\":\"US\",\"tld\":\".US\",\"population\":310232863,\"phone_prefix\":\"1\",\"postal_regex\":\"^\\\\d{5}(-\\\\d{4})?$\",\"currency\":\"USD\",\"continent\":\"NA\",\"languages\":\"en-US,es-US,haw,fr\",\"neightboors\":\"CA,MX,CU\"}}");

        Ok(())
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct ContinentSerde {
        text: String,
        cont: Continent,
    }

    #[test]
    fn serde_deserialize_continent() -> Result<()> {
        let json = r#"{"text": "test", "cont": "NA"}"#;
        let test_deserialize: ContinentSerde = serde_json::from_str(json)?;
        //{ text: "test", cont: Continent { code: NA, name: "North America" } }
        assert_eq!(test_deserialize.text, "test");
        assert_eq!(test_deserialize.cont.code, Code::NA);
        assert_eq!(test_deserialize.cont.name, "North America");

        Ok(())
    }

    #[test]
    fn serde_deserialize_continent_error() -> Result<()> {
        let json = r#"{"text": "test", "cont": "USDSF"}"#;
        let result: Result<Country> = serde_json::from_str(json);
        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn serde_serialize_continent() -> Result<()> {
        let continent = Continent::find_for(Code::NA);
        let test_serialize = ContinentSerde {
            text: String::from("test"),
            cont: *continent,
        };

        let test: String = serde_json::to_string(&test_serialize).unwrap();
        assert_eq!(test, "{\"text\":\"test\",\"cont\":{\"code\":\"NA\",\"name\":\"North America\"}}");

        Ok(())
    }

}
