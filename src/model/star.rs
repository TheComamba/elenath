use astro_utils::stars::{appearance::StarAppearance, data::StarData};
use serde::{Deserialize, Serialize};
use uom::si::f64::Time;

use super::celestial_system::part::{BodyType, PartOfCelestialSystem};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Star {
    data: Option<StarData>,
    appearance: StarAppearance,
    index: Option<usize>,
}

impl Star {
    pub(crate) fn from_data(data: StarData, index: Option<usize>, time_since_epoch: Time) -> Self {
        let appearance = data.to_star_appearance(time_since_epoch);
        Star {
            data: Some(data),
            appearance,
            index,
        }
    }

    pub(crate) fn from_appearance(appearance: StarAppearance, index: Option<usize>) -> Self {
        Star {
            data: None,
            appearance,
            index,
        }
    }

    pub(crate) fn get_data(&self) -> Option<&StarData> {
        self.data.as_ref()
    }

    pub(crate) fn get_appearance(&self) -> &StarAppearance {
        &self.appearance
    }

    pub(super) fn set_index(&mut self, index: usize) {
        self.index = Some(index);
    }

    pub(super) fn recalculate_appearance_if_necessary(&mut self, time_since_epoch: Time) {
        if let Some(data) = &self.data {
            let then = self.appearance.get_time_since_epoch();
            if data.has_changed(*then, time_since_epoch) {
                self.appearance = data.to_star_appearance(time_since_epoch);
            }
        }
    }
}

impl PartOfCelestialSystem for Star {
    fn get_index(&self) -> Option<usize> {
        self.index
    }

    fn get_body_type(&self) -> BodyType {
        BodyType::Star
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub(crate) enum StarDataType {
    Hardcoded,
    GaiaMeasurementSmall,
    GaiaMeasurementLarge,
    GaiaSimulation,
}
