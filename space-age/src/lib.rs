#![feature(associated_consts)]


macro_rules! implement_planet {
    ($planet_name:ident, $op:expr) => (
        pub struct $planet_name;

        impl Planet for $planet_name {
            const OP: f64 = $op;
        }
    )
}


pub struct Duration {
    earth_years: f64,
}


impl From<u64> for Duration {

    fn from(seconds: u64) -> Self {
        Duration { earth_years: seconds as f64 / 31557600f64 }
    }

}


pub trait Planet {

    const OP: f64;

    fn years_during(duration: &Duration) -> f64 {
        duration.earth_years / Self::OP // orbital period
    }

}


implement_planet!(Earth, 1f64);
implement_planet!(Jupiter, 11.862615);
implement_planet!(Mars, 1.8808158);
implement_planet!(Mercury, 0.2408467);
implement_planet!(Neptune, 164.79132);
implement_planet!(Saturn, 29.447498);
implement_planet!(Uranus, 84.016846);
implement_planet!(Venus, 0.61519726);
