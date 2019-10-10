
pub mod distance {
    pub const STATUTE_MILE: f64 = 1609.34;
    pub const NAUTICAL_MILE: f64 = 1852.0;

    pub fn statute_to_nautical(stat_dist: f64) -> f64 {
        stat_dist * STATUTE_MILE / NAUTICAL_MILE
    }

    pub fn nautical_to_statute(naut_dist: f64) -> f64 {
        naut_dist * NAUTICAL_MILE / STATUTE_MILE
    }
}

pub mod speed {
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
    fn nm_to_sm() {
        assert_eq!(distance::nautical_to_statute(1.0), 1.1507823082754423);
    }

    #[test]
    fn nm_to_sm_disp() {
        assert_eq!(format!("{:.2}", distance::nautical_to_statute(1.0)), "1.15");
    }
}
