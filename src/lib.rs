
pub mod distance {
    pub const STATUTE_MILE: f64 = 1609.34;
    pub const NAUTICAL_MILE: f64 = 1852.0;

    pub fn statute_to_nautical(stat_dist: f64) -> f64 {
        (stat_dist/STATUTE_MILE) * NAUTICAL_MILE
    }
}

pub mod speed {
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sm_to_nm() {
        assert_eq!(distance::statute_to_nautical(1.0), 1.1507823082754423);
    }

    #[test]
    fn sm_to_nm_disp() {
        assert_eq!(format!("{:.2}", distance::statute_to_nautical(1.0)), "1.15");
    }
}
