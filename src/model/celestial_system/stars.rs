use astro_coords::cartesian::Cartesian;
use astro_units::illuminance::Illuminance;
use astro_utils::{
    real_data::stars::{all::get_many_stars, sun},
    stars::{
        appearance::StarAppearance,
        data::StarData,
        gaia::{
            gaia_source::fetch_brightest_stars,
            gaia_universe_simulation::fetch_brightest_stars_simulated_data,
        },
        random::random_stars::{generate_random_star, generate_random_stars},
    },
};
use std::cmp::Ordering;
use uom::si::f64::Length;

use crate::{
    error::ElenathError,
    model::star::{Star, StarDataType},
};

use super::CelestialSystem;

impl CelestialSystem {
    pub(crate) fn add_stars_from_data(&mut self, star_data: Vec<StarData>) {
        let index = self.distant_stars.len();
        for data in star_data {
            self.distant_stars
                .push(Star::from_data(data, Some(index), self.time_since_epoch));
        }
        self.process_stars();
    }

    fn remove_known_star_from_list(
        star_appearances: &mut Vec<StarAppearance>,
        known_star: &StarAppearance,
    ) {
        let mut index_to_remove = None;

        for (index, star_appearance) in star_appearances.iter().enumerate() {
            if star_appearance.apparently_the_same(known_star) {
                index_to_remove = Some(index);
                break;
            }
        }

        if let Some(index) = index_to_remove {
            star_appearances.remove(index);
        }
    }

    pub(crate) fn add_star_appearances_without_duplicates(
        &mut self,
        mut star_appearances: Vec<StarAppearance>,
    ) {
        for known_star in self.get_distant_star_appearances() {
            Self::remove_known_star_from_list(&mut star_appearances, known_star);
        }

        for star_appearance in star_appearances {
            let index = self.distant_stars.len();
            self.distant_stars
                .push(Star::from_appearance(star_appearance, Some(index)));
        }
        self.process_stars();
    }

    pub(crate) fn overwrite_star_data(&mut self, index: Option<usize>, star_data: StarData) {
        match index {
            Some(index) => {
                self.distant_stars[index] =
                    Star::from_data(star_data, Some(index), self.time_since_epoch)
            }
            None => self.central_body = star_data,
        }
        self.process_stars();
    }

    fn process_stars(&mut self) {
        self.sort_stars_by_brightness();
        self.update_constellations();
    }

    fn sort_stars_by_brightness(&mut self) {
        fn illum(b: &Star) -> Illuminance {
            b.get_appearance().get_illuminance()
        }

        self.distant_stars
            .sort_by(|a, b| illum(b).partial_cmp(&illum(a)).unwrap_or(Ordering::Equal));
        for (i, star) in self.distant_stars.iter_mut().enumerate() {
            star.set_index(i);
        }
    }

    pub(crate) fn randomize_stars(
        &mut self,
        keep_central_body: bool,
        max_distance: Length,
    ) -> Result<(), ElenathError> {
        if !keep_central_body {
            self.central_body = generate_random_star(None)?
        };
        let stars = generate_random_stars(max_distance)?;
        self.add_stars_from_data(stars);
        Ok(())
    }

    pub(crate) fn load_real_stars(&mut self, data_type: StarDataType) -> Result<(), ElenathError> {
        self.central_body = sun().to_star_data();
        self.distant_stars.clear();
        match data_type {
            StarDataType::Hardcoded => {
                let stars = get_many_stars().iter().map(|s| s.to_star_data()).collect();
                self.add_stars_from_data(stars);
            }
            StarDataType::GaiaMeasurementSmall => {
                self.load_gaia_data(6.)?;
            }
            StarDataType::GaiaMeasurementLarge => {
                self.load_gaia_data(11.0)?;
            }
            StarDataType::GaiaSimulation => {
                let stars = fetch_brightest_stars_simulated_data()?;
                self.add_stars_from_data(stars);
            }
        }
        Ok(())
    }

    fn load_gaia_data(&mut self, magnitude_threshold: f64) -> Result<(), ElenathError> {
        let hardcoded_stars = get_many_stars().iter().map(|s| s.to_star_data()).collect();
        self.add_stars_from_data(hardcoded_stars);
        let gaia_stars = fetch_brightest_stars(magnitude_threshold)?;
        println!("Fetched {} stars from Gaia", gaia_stars.len());
        self.add_star_appearances_without_duplicates(gaia_stars);
        Ok(())
    }

    pub(crate) fn get_central_body_data(&self) -> &StarData {
        &self.central_body
    }

    pub(crate) fn get_central_body_appearance(&self, observer_pos: &Cartesian) -> StarAppearance {
        let mut body = self.central_body.clone();
        let relative_position = -observer_pos;
        body.set_pos_at_epoch(relative_position);
        body.to_star_appearance(self.time_since_epoch)
    }

    pub(crate) fn get_stars(&self) -> Vec<Star> {
        let mut bodies = Vec::new();
        bodies.push(Star::from_data(
            self.central_body.clone(),
            None,
            self.time_since_epoch,
        ));
        for star in &self.distant_stars {
            bodies.push(star.clone());
        }
        bodies
    }

    pub(crate) fn get_distant_star_appearances(&self) -> Vec<&StarAppearance> {
        let mut stars = Vec::new();
        for star in &self.distant_stars {
            stars.push(star.get_appearance());
        }
        stars
    }

    pub(crate) fn get_star_data(&self, index: Option<usize>) -> Option<&StarData> {
        match index {
            Some(index) => self.distant_stars.get(index).and_then(|s| s.get_data()),
            None => Some(&self.central_body),
        }
    }
}

#[cfg(test)]
mod tests {
    use astro_units::luminous_intensity::absolute_magnitude_to_luminous_intensity;
    use uom::si::length::light_year;

    use crate::model::celestial_system::part::PartOfCelestialSystem;

    use super::*;

    #[test]
    fn central_body_has_distance_zero() {
        for star in get_many_stars().iter() {
            let system = CelestialSystem::new(star.to_star_data());
            assert!(
                system.get_central_body_data().get_distance_at_epoch()
                    < Length::new::<light_year>(1e-20)
            );
        }
    }

    #[test]
    fn stars_are_sorted_by_brightness() {
        let mut system = CelestialSystem::new(sun().to_star_data());
        let reverse_stars = get_many_stars()
            .iter()
            .rev()
            .map(|s| s.to_star_data())
            .collect();
        system.add_stars_from_data(reverse_stars);
        let stars = system.get_stars();
        for i in 1..stars.len() - 1 {
            assert!(
                stars[i].get_appearance().get_illuminance()
                    >= stars[i + 1].get_appearance().get_illuminance()
            );
        }
    }

    #[test]
    fn edited_stars_are_sorted_by_brightness() {
        let mut system = CelestialSystem::new(sun().to_star_data());
        let stars = get_many_stars().iter().map(|s| s.to_star_data()).collect();
        system.add_stars_from_data(stars);
        let mut bright_star = sun().to_star_data();
        bright_star.set_distance_at_epoch(Length::new::<light_year>(1.));
        bright_star.set_luminous_intensity_at_epoch(absolute_magnitude_to_luminous_intensity(-10.));
        system.overwrite_star_data(Some(17), bright_star);
        let stars = system.get_stars();
        for i in 1..stars.len() - 1 {
            assert!(
                stars[i].get_appearance().get_illuminance()
                    >= stars[i + 1].get_appearance().get_illuminance()
            );
        }
    }

    #[test]
    fn star_index_is_correct_after_sorting() {
        let mut system = CelestialSystem::new(sun().to_star_data());
        let reversed_stars = get_many_stars()
            .iter()
            .rev()
            .map(|s| s.to_star_data())
            .collect();
        system.add_stars_from_data(reversed_stars);
        for (i, star) in system.get_stars().iter().enumerate() {
            if i == 0 {
                assert_eq!(star.get_index(), None);
            } else {
                assert_eq!(star.get_index(), Some(i - 1));
            }
        }
    }
}
