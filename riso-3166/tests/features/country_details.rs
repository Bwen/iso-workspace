mod details {
    extern crate riso_3166;
    use static_traits::{TryFor, FindFor};
    use riso_3166::country::{Country, Detail, Alpha2, Tld};
    use riso_3166::continent::Code;
    use riso_639::{
        Alpha2 as LangAlpha2, 
        Alpha3 as LangAlpha3
    };
    
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

        let langAlpha2s: Vec<&LangAlpha2> = languages.iter().map(|n| &n.alpha2).collect();
        assert!(langAlpha2s.contains(&&LangAlpha2::EN));
        assert!(langAlpha2s.contains(&&LangAlpha2::FR));
        assert!(langAlpha2s.contains(&&LangAlpha2::IU));

        let langAlpha3s: Vec<&LangAlpha3> = languages.iter().map(|n| &n.alpha3).collect();
        assert!(langAlpha3s.contains(&&LangAlpha3::ENG));
        assert!(langAlpha3s.contains(&&LangAlpha3::FRE));
        assert!(langAlpha3s.contains(&&LangAlpha3::IKU));
    }
}
