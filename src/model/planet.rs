use astro_coords::cartesian::Cartesian;
use astro_utils::{
    planets::{derived_data::DerivedPlanetData, planet_data::PlanetData},
    stars::data::StarData,
};
use uom::si::f64::Time;

use super::celestial_system::part::{BodyType, PartOfCelestialSystem};

pub(crate) struct Planet {
    data: PlanetData,
    derived_data: Option<DerivedPlanetData>,
    pos: Cartesian,
    index: Option<usize>,
}

impl Planet {
    pub(crate) fn new(
        data: PlanetData,
        central_body: &StarData,
        previous: Option<&DerivedPlanetData>,
        time: Time,
        index: Option<usize>,
    ) -> Self {
        let derived_data = DerivedPlanetData::new(&data, central_body, previous).ok();
        let pos = calc_pos(central_body, time, &data);
        Self {
            data,
            derived_data,
            pos,
            index,
        }
    }

    pub(crate) fn get_data(&self) -> &PlanetData {
        &self.data
    }

    pub(crate) fn get_derived_data(&self) -> Option<&DerivedPlanetData> {
        self.derived_data.as_ref()
    }

    pub(crate) fn get_position(&self) -> &Cartesian {
        &self.pos
    }
}

fn calc_pos(central_body: &StarData, time: Time, data: &PlanetData) -> Cartesian {
    let pos = if let Some(central_body_mass) = central_body.get_mass(time) {
        data.get_orbital_parameters()
            .calculate_position(data.get_mass(), central_body_mass, time)
    } else {
        Cartesian::ORIGIN
    };
    pos
}

impl PartOfCelestialSystem for Planet {
    fn get_index(&self) -> Option<usize> {
        self.index
    }

    fn get_body_type(&self) -> BodyType {
        BodyType::Planet
    }
}
