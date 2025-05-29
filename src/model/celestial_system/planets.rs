use std::cmp::Ordering;

use astro_utils::{planets::planet_data::PlanetData, real_data::planets::*};

use crate::model::planet::Planet;

use super::CelestialSystem;

impl CelestialSystem {
    pub(crate) fn add_planet_data(&mut self, planet: PlanetData) {
        self.planets.push(planet);
        self.sort_planets_by_semimajor_axis();
    }

    pub(crate) fn overwrite_planet_data(&mut self, index: usize, planet: PlanetData) {
        self.planets[index] = planet;
        self.sort_planets_by_semimajor_axis();
    }

    fn sort_planets_by_semimajor_axis(&mut self) {
        fn sma(a: &PlanetData) -> Length {
            a.get_orbital_parameters().get_semi_major_axis()
        }
        self.planets
            .sort_by(|a, b| sma(a).partial_cmp(&sma(b)).unwrap_or(Ordering::Equal));
    }

    pub(crate) fn randomize_planets(&mut self) {
        todo!();
        self.sort_planets_by_semimajor_axis();
    }

    pub(crate) fn load_real_planets(&mut self) {
        self.planets.clear();
        self.add_planet_data(MERCURY.to_planet_data());
        self.add_planet_data(VENUS.to_planet_data());
        self.add_planet_data(EARTH.to_planet_data());
        self.add_planet_data(MARS.to_planet_data());
        self.add_planet_data(CERES.to_planet_data());
        self.add_planet_data(JUPITER.to_planet_data());
        self.add_planet_data(SATURN.to_planet_data());
        self.add_planet_data(URANUS.to_planet_data());
        self.add_planet_data(NEPTUNE.to_planet_data());
        self.add_planet_data(PLUTO.to_planet_data());
    }

    pub(crate) fn get_planets_data(&self) -> Vec<&PlanetData> {
        let mut bodies = Vec::new();
        for planet in &self.planets {
            bodies.push(planet);
        }
        bodies
    }

    pub(crate) fn get_planet_data(&self, index: usize) -> Option<&PlanetData> {
        self.planets.get(index)
    }

    pub(crate) fn get_planets(&self) -> Vec<Planet> {
        let mut bodies: Vec<Planet> = Vec::new();
        for (i, planet_data) in self.planets.iter().enumerate() {
            let previous = if i > 0 {
                bodies[i - 1].get_derived_data()
            } else {
                None
            };
            let planet = Planet::new(
                planet_data.clone(),
                &self.central_body,
                previous,
                self.time_since_epoch,
                Some(i),
            );
            bodies.push(planet);
        }
        bodies
    }
}

#[cfg(test)]
mod tests {
    use astro_utils::real_data::planets::*;

    use crate::model::celestial_system::CelestialSystem;

    #[test]
    fn planets_are_sorted_by_semimajor_axis() {
        let mut system = CelestialSystem::empty();
        system.add_planet_data(VENUS.to_planet_data());
        system.add_planet_data(MERCURY.to_planet_data());
        system.add_planet_data(MARS.to_planet_data());
        system.add_planet_data(EARTH.to_planet_data());
        let planets = system.get_planets_data();
        assert_eq!(planets[0].get_name(), "Mercury");
        assert_eq!(planets[1].get_name(), "Venus");
        assert_eq!(planets[2].get_name(), "Earth");
        assert_eq!(planets[3].get_name(), "Mars");
    }

    #[test]
    fn edited_planets_are_sorted_by_semimajor_axis() {
        let mut system = CelestialSystem::empty();
        system.add_planet_data(MERCURY.to_planet_data());
        system.add_planet_data(EARTH.to_planet_data());
        system.overwrite_planet_data(0, JUPITER.to_planet_data());
        let planets = system.get_planets_data();
        assert_eq!(planets[0].get_name(), "Earth");
        assert_eq!(planets[1].get_name(), "Jupiter");
    }
}
