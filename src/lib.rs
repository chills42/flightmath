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

pub mod airspeed {
    #[derive(Debug, PartialEq)]
    pub struct Airspeed {
        pub direction: i16,
        pub speed: f64
    }

    impl Airspeed {
        pub fn from_dir_and_speed(direction: i16, speed: f64) -> Self {
            Self { direction, speed }
        }

        pub fn components(&self, direction: i16) -> WindComponents {
            let mut x = self.direction - direction;
            if x < -180 {
                x += 180
            }
            let y = 90 - x;
            let base = (self.speed * (x as f64).to_radians().cos() * 100.0).round() / 100.0;
            let cross = (self.speed * (y as f64).to_radians().cos() * 100.0).round() / 100.0;
            WindComponents::from_raw(base, cross)
        }

        pub fn plus(&self, x: Airspeed) -> Airspeed {
            let v1 = self.speed;
            let v2 = x.speed;
            let diff = self.direction - x.direction;
            let alpha = (180 - diff.abs()).abs();
            let new_speed = (v1.powi(2) + v2.powi(2) - 2.0 * v1 * v2 * (alpha as f64).to_radians().cos()).sqrt();
            let wca = (v2 * (alpha as f64).to_radians().sin() / new_speed).asin().to_degrees().copysign(diff as f64);
            let new_dir = self.direction + wca as i16;

            Airspeed::from_dir_and_speed(new_dir, (new_speed * 100.0).round() / 100.0)
        }
    }

    impl Into<f64> for BaseComponent {
        fn into(self) -> f64 {
            match self {
                Self::Headwind(x) => x,
                Self::Tailwind(x) => x * -1.0
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct WindComponents {
        pub base: BaseComponent,
        pub cross: CrossComponent
    }

    impl WindComponents {
        pub fn from_raw(base: f64, cross: f64) -> Self {
            let base = if base > 0.0 {
                BaseComponent::Headwind(base)
            } else {
                BaseComponent::Tailwind(base.abs())
            };
            let cross = if cross > 0.0 {
                CrossComponent::RightCross(cross)
            } else {
                CrossComponent::LeftCross(cross.abs())
            };
            Self { base, cross }
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum BaseComponent {
        Headwind(f64),
        Tailwind(f64)
    }


    #[derive(Debug, PartialEq)]
    pub enum CrossComponent {
        LeftCross(f64),
        RightCross(f64)
    }
}

pub mod fuel {
    pub trait Fuel {
        fn weight(&self) -> f64;
        fn volume(&self) -> f64;
    }

    pub struct AvGas100LL {
        pub gallons: f64
    }

    impl Fuel for AvGas100LL {
        fn weight(&self) -> f64 {
            self.gallons *  6.01
        }

        fn volume(&self) -> f64 {
            self.gallons
        }
    }

    pub struct JetA {
        pub gallons: f64
    }

    impl JetA {
        pub fn from_lbs(weight: f64) -> Self {
            Self { gallons: weight / 6.55 }
        }
    }

    impl Fuel for JetA {
        fn weight(&self) -> f64 {
            self.gallons * 6.55
        }

        fn volume(&self) -> f64 {
            self.gallons
        }
    }

    pub fn fuel_consumed(gal_per_hour: f64, minutes: f64) -> f64 {
        minutes * gal_per_hour / 60.0
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

    #[test]
    fn weight_used() {
        let minutes = 45.0;
        let gal_per_hour = 12.0;
        let avgas = fuel::AvGas100LL { gallons: fuel::fuel_consumed(gal_per_hour, minutes) };
        assert_eq!(format!("{:.2}", fuel::Fuel::weight(&avgas)), "54.09");
    }

    #[test]
    fn jet_a_usage() {
        let jeta = fuel::JetA::from_lbs(830.0);
        assert_eq!(format!("{:.2}", fuel::Fuel::volume(&jeta)), "126.72");

    }

    #[test]
    fn airspeed_test() {
        assert_eq!(airspeed::Airspeed::from_dir_and_speed(270, 20.0).speed, 20.0)
    }

    #[test]
    fn airspeed_math_test1() {
        let expected = airspeed::WindComponents::from_raw(17.32, -10.00);
        println!("wind from {} at {:02} using runway {} {:?}", 270, 20.0, 30, expected);
        assert_eq!(airspeed::Airspeed::from_dir_and_speed(270, 20.0).components(300), expected);
    }

    #[test]
    fn airspeed_math_test2() {
        let expected = airspeed::WindComponents::from_raw(17.32, 10.00);
        println!("wind from {} at {:02} using runway {} {:?}", 270, 20.0, 24, expected);
        assert_eq!(airspeed::Airspeed::from_dir_and_speed(270, 20.0).components(240), expected);
    }

    #[test]
    fn airspeed_math_test3() {
        let expected = airspeed::WindComponents::from_raw(17.32, -10.00);
        println!("wind from {} at {:02} using runway {} {:?}", 350, 20.0, 2, expected);
        assert_eq!(airspeed::Airspeed::from_dir_and_speed(350, 20.0).components(20), expected);
    }

    #[test]
    fn airspeed_math_test4() {
        let expected = airspeed::WindComponents::from_raw(-18.79, -6.84);
        println!("wind from {} at {:02} using runway {} {:?}", 270, 20.0, 7, expected);
        assert_eq!(airspeed::Airspeed::from_dir_and_speed(270, 20.0).components(70), expected);
    }

    #[test]
    fn air_plus3() {
        let course = airspeed::Airspeed::from_dir_and_speed(90, 124.0);
        let wind = airspeed::Airspeed::from_dir_and_speed(130, 15.0);
        let expected = airspeed::Airspeed::from_dir_and_speed(86, 135.83);
        assert_eq!(course.plus(wind), expected);
    }

    #[test]
    fn air_plus2() {
        let course = airspeed::Airspeed::from_dir_and_speed(60, 100.0);
        let wind = airspeed::Airspeed::from_dir_and_speed(180, 40.0);
        let expected = airspeed::Airspeed::from_dir_and_speed(40, 74.00);
        assert_eq!(course.plus(wind), expected);
    }

    #[test]
    fn air_plus() {
        let course = airspeed::Airspeed::from_dir_and_speed(90, 124.0);
        let wind = airspeed::Airspeed::from_dir_and_speed(50, 15.0);
        let expected = airspeed::Airspeed::from_dir_and_speed(94, 135.83);
        assert_eq!(course.plus(wind), expected);
    }
}
