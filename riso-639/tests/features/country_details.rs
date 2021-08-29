mod country_details {
    extern crate riso_3166;
    extern crate riso_639;
    use static_traits::{IterFor, TryFor, FindFor};
    use riso_3166::country::Country;
    use riso_3166::country::Alpha2 as CountryAlpha2;
    use riso_639::{Language, Alpha2};

    #[test]
    fn languages_for_country() {
        let country = Country::find_for(CountryAlpha2::CA);
        let languages = Language::iter_for(country);
        assert_eq!(languages.len(), 3);
        assert!(languages.iter().any(|lang| lang.alpha2 == Alpha2::EN));
        assert!(languages.iter().any(|lang| lang.alpha2 == Alpha2::FR));
        assert!(languages.iter().any(|lang| lang.alpha2 == Alpha2::IU));
    }
}
