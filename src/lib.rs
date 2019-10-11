pub mod distance {
    struct StatuteMiles {
        pub value: f64
    }

    impl StatuteMiles {
        fn new(value: f64) -> Self {
            StatuteMiles { value }
        }
    }

    impl From<NauticalMiles> for StatuteMiles {
        fn from(item: NauticalMiles) -> Self {
            StatuteMiles::new(item.value * 1.1507823082754423)
        }
    }

    struct NauticalMiles {
        pub value: f64
    }

    impl NauticalMiles {
        fn new(value: f64) -> Self {
            NauticalMiles { value }
        }
    }

    impl From<StatuteMiles> for NauticalMiles {
        fn from(item: StatuteMiles) -> Self {
            NauticalMiles::new(item.value * 0.8689740820734341)
        }
    }

    pub fn statute_to_nautical(stat_dist: f64) -> f64 {
        NauticalMiles::from(StatuteMiles::new(stat_dist)).value
    }

    pub fn nautical_to_statute(naut_dist: f64) -> f64 {
        StatuteMiles::from(NauticalMiles::new(naut_dist)).value
    }
}

pub mod speed {
    pub fn mph_to_knots(mph: f64) -> f64 {
        crate::distance::statute_to_nautical(mph)
    }

    pub fn knots_to_mph(knots: f64) -> f64 {
        crate::distance::nautical_to_statute(knots)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sm_to_nm() {
        assert_eq!(distance::statute_to_nautical(1.0), 0.8689740820734341);
    }

    #[test]
    fn sm_to_nm_disp() {
        assert_eq!(format!("{:.2}", distance::statute_to_nautical(1.0)), "0.87");
    }

    #[test]
    fn mph_to_knots_test() {
        assert_eq!(speed::mph_to_knots(100.0), 86.8974082073434);
    }

    #[test]
    fn mph_to_knots_test_disp() {
        assert_eq!(format!("{:.0}", speed::mph_to_knots(150.0)), "130");
    }

    #[test]
    fn knots_to_mph_test() {
        assert_eq!(speed::knots_to_mph(124.0), 142.69700622615483);
    }

    #[test]
    fn knots_to_mph_test_disp() {
        assert_eq!(format!("{:.0}", speed::knots_to_mph(124.0)), "143");
    }

    #[test]
    fn nm_to_sm() {
        assert_eq!(distance::nautical_to_statute(1.0), 1.1507823082754423);
    }

    #[test]
    fn nm_to_sm_disp() {
        assert_eq!(format!("{:.2}", distance::nautical_to_statute(1.0)), "1.15");
    }
}
