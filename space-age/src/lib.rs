// I had to look at community solutions to get to the code below.
// The books/docs were not helpful at all.

macro_rules! planet_years {
    ($planet:ident => $year_mult:expr) => {
        pub struct $planet;
        impl Planet for $planet {
            const ORBITAL_PERIOD: f64 = $year_mult;
        }
    };
}

#[derive(Debug)]
pub struct Duration (f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self{0: s as f64}
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: f64;
    const EARTH_YEAR: f64 = 31557600.0;
    fn years_during(d: &Duration) -> f64 {
        d.0 / (Self::EARTH_YEAR * Self::ORBITAL_PERIOD)
    } 
}

planet_years!(Mercury => 0.2408467);
planet_years!(Venus => 0.61519726);
planet_years!(Earth => 1.0);
planet_years!(Mars => 1.8808158);
planet_years!(Jupiter => 11.862615);
planet_years!(Saturn => 29.447498);
planet_years!(Uranus => 84.016846);
planet_years!(Neptune => 164.79132);
