use astro_coords::cartesian::Cartesian;
use astro_utils::{
    planets::planet_data::PlanetData,
    stars::{
        constellation::Constellation, data::StarData, evolution::StarDataEvolution, fate::StarFate,
        physical_parameters::StarPhysicalParameters,
    },
};
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, path::PathBuf};
use uom::si::f64::Time;

use super::star::Star;

pub(crate) mod constellations;
pub(crate) mod part;
pub(crate) mod planets;
pub(crate) mod stars;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct CelestialSystem {
    central_body: StarData,
    planets: Vec<PlanetData>,
    distant_stars: Vec<Star>,
    constellations: Vec<Constellation>,
    time_since_epoch: Time,
}

impl CelestialSystem {
    #[cfg(test)]
    pub(crate) fn new(mut central_body: StarData) -> Self {
        central_body.set_distance_at_epoch(DISTANCE_ZERO);
        CelestialSystem {
            central_body,
            planets: vec![],
            distant_stars: vec![],
            constellations: vec![],
            time_since_epoch: TIME_ZERO,
        }
    }

    pub(crate) fn empty() -> Self {
        let central_body_params =
            StarPhysicalParameters::new(None, None, LUMINOSITY_ZERO, TEMPERATURE_ZERO);
        let central_body = StarData::new(
            "".to_string(),
            None,
            central_body_params,
            Cartesian::ORIGIN,
            StarDataEvolution::NONE,
        );
        CelestialSystem {
            central_body,
            planets: vec![],
            distant_stars: vec![],
            constellations: vec![],
            time_since_epoch: TIME_ZERO,
        }
    }

    pub(crate) fn set_time_since_epoch(&mut self, time_since_epoch: Time) {
        self.time_since_epoch = time_since_epoch;
        for star in &mut self.distant_stars {
            star.recalculate_appearance_if_necessary(time_since_epoch);
        }
        self.update_constellations();
    }

    pub(crate) fn get_time_since_epoch(&self) -> Time {
        self.time_since_epoch
    }

    pub(crate) fn write_to_file(&self, path: PathBuf) -> Result<(), std::io::Error> {
        let file = std::fs::File::create(path)?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer(writer, self)?;
        Ok(())
    }

    pub(crate) fn read_from_file(path: PathBuf) -> Result<Self, std::io::Error> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let celestial_system = serde_json::from_reader(reader)?;
        Ok(celestial_system)
    }

    pub(crate) fn get_supernovae(&self) -> Vec<Star> {
        let mut supernovae: Vec<Star> = self
            .get_stars()
            .into_iter()
            .filter(|s| {
                if let Some(data) = s.get_data() {
                    data.get_fate() == &StarFate::TypeIISupernova
                } else {
                    false
                }
            })
            .collect();
        supernovae.sort_by(|a, b| self.ord_by_time_til_death(a, b));
        supernovae
    }

    fn ord_by_time_til_death(&self, a: &Star, b: &Star) -> std::cmp::Ordering {
        let data_a = a.get_data();
        let data_b = b.get_data();
        if let (Some(data_a), Some(data_b)) = (data_a, data_b) {
            let t_a = data_a.get_time_until_death(self.time_since_epoch);
            let t_b = data_b.get_time_until_death(self.time_since_epoch);
            t_a.partial_cmp(&t_b).unwrap_or(Ordering::Equal)
        } else {
            Ordering::Equal
        }
    }
}
