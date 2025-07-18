use crate::model::{planet::Planet, star::Star};
use astro_utils::astro_display::AstroDisplay;
use uom::si::{f64::Time, time::year};

pub(super) struct TableColData<T> {
    pub(super) header: &'static str,
    pub(super) content_closure: Box<dyn Fn(&T) -> Option<String>>,
}

#[derive(Debug, Clone)]
pub(crate) enum TableDataType {
    Planet,
    Star,
    Supernova,
}

impl TableColData<Planet> {
    pub(super) fn default_planet_col_data() -> Vec<TableColData<Planet>> {
        vec![
            TableColData {
                header: "Planet Name",
                content_closure: Box::new(|body| {
                    let name = body.get_data().get_name();
                    Some(name.to_string())
                }),
            },
            TableColData {
                header: "Mass",
                content_closure: Box::new(|body| {
                    let mass = body.get_data().get_mass();
                    Some(mass.astro_display())
                }),
            },
            TableColData {
                header: "Radius",
                content_closure: Box::new(|body| {
                    let radius = body.get_data().get_radius();
                    Some(radius.astro_display())
                }),
            },
            TableColData {
                header: "Density",
                content_closure: Box::new(|body| {
                    let density = body.get_derived_data()?.get_density();
                    Some(density.astro_display())
                }),
            },
            TableColData {
                header: "Surface Gravity",
                content_closure: Box::new(|body| {
                    let surface_gravity = body.get_derived_data()?.get_surface_gravity();
                    Some(surface_gravity.astro_display())
                }),
            },
            TableColData {
                header: "Escape Velocity",
                content_closure: Box::new(|body| {
                    let escape_velocity = body.get_derived_data()?.get_escape_velocity();
                    Some(escape_velocity.astro_display())
                }),
            },
            TableColData {
                header: "Color",
                content_closure: Box::new(|body| {
                    let color = body.get_data().get_color();
                    Some(color.astro_display())
                }),
            },
            TableColData {
                header: "Geometric Albedo",
                content_closure: Box::new(|body| {
                    let albedo = body.get_data().get_geometric_albedo();
                    Some(format!("{:.2}", albedo))
                }),
            },
            TableColData {
                header: "Black Body Temp.",
                content_closure: Box::new(|body| {
                    let temperature = body.get_derived_data()?.get_black_body_temperature();
                    Some(temperature.astro_display())
                }),
            },
            TableColData {
                header: "Semi-major Axis",
                content_closure: Box::new(|body| {
                    let semi_major_axis = body
                        .get_data()
                        .get_orbital_parameters()
                        .get_semi_major_axis();
                    Some(semi_major_axis.astro_display())
                }),
            },
            TableColData {
                header: "Eccentricity",
                content_closure: Box::new(|body| {
                    let eccentricity = body.get_data().get_orbital_parameters().get_eccentricity();
                    Some(format!("{:.2}", eccentricity))
                }),
            },
            TableColData {
                header: "Inclination",
                content_closure: Box::new(|body| {
                    let inclination = body.get_data().get_orbital_parameters().get_inclination();
                    Some(inclination.astro_display())
                }),
            },
            TableColData {
                header: "Ascending Node",
                content_closure: Box::new(|body| {
                    let ascending_node = body
                        .get_data()
                        .get_orbital_parameters()
                        .get_longitude_of_ascending_node();
                    Some(ascending_node.astro_display())
                }),
            },
            TableColData {
                header: "Arg. of Periapsis",
                content_closure: Box::new(|body| {
                    let arg_of_periapsis = body
                        .get_data()
                        .get_orbital_parameters()
                        .get_argument_of_periapsis();
                    Some(arg_of_periapsis.astro_display())
                }),
            },
            TableColData {
                header: "Orbital Period",
                content_closure: Box::new(|body| {
                    let orbital_period = body.get_derived_data()?.get_orbital_period();
                    Some(orbital_period.astro_display())
                }),
            },
            TableColData {
                header: "Orbital Resonance",
                content_closure: Box::new(|body| {
                    let orbital_resonance = body.get_derived_data()?.get_orbital_resonance()?;
                    Some(orbital_resonance.astro_display())
                }),
            },
            TableColData {
                header: "Sideral Day",
                content_closure: Box::new(|body| {
                    let siderial_day = body.get_data().get_sideral_rotation_period();
                    Some(siderial_day.astro_display())
                }),
            },
            TableColData {
                header: "Synodic Day",
                content_closure: Box::new(|body| {
                    let synodic_day = body.get_derived_data()?.get_mean_synodic_day();
                    Some(synodic_day.astro_display())
                }),
            },
            TableColData {
                header: "Rotation Axis",
                content_closure: Box::new(|body| {
                    let rotation_axis = body.get_data().get_rotation_axis();
                    Some(format!("{}", rotation_axis))
                }),
            },
            TableColData {
                header: "Axial Tilt",
                content_closure: Box::new(|body| {
                    let axial_tilt = body.get_derived_data()?.get_axial_tilt();
                    Some(axial_tilt.astro_display())
                }),
            },
        ]
    }
}

impl TableColData<Star> {
    pub(super) fn default_star_col_data() -> Vec<TableColData<Star>> {
        vec![
            TableColData {
                header: "Star Name",
                content_closure: Box::new(|body| {
                    let name = body.get_appearance().get_name();
                    Some(name.to_string())
                }),
            },
            TableColData {
                header: "Mass",
                content_closure: Box::new(|body| {
                    let mass = body.get_data()?.get_mass_at_epoch()?;
                    Some(mass.astro_display())
                }),
            },
            TableColData {
                header: "Radius",
                content_closure: Box::new(|body| {
                    let radius = body.get_data()?.get_radius_at_epoch()?;
                    Some(radius.astro_display())
                }),
            },
            TableColData {
                header: "Luminous Intensity",
                content_closure: Box::new(|body| {
                    let luminous_intensity = body.get_data()?.get_luminous_intensity_at_epoch();
                    Some(luminous_intensity.astro_display())
                }),
            },
            TableColData {
                header: "Temperature",
                content_closure: Box::new(|body| {
                    Some(body.get_data()?.get_temperature_at_epoch().astro_display())
                }),
            },
            TableColData {
                header: "Color",
                content_closure: Box::new(|body| {
                    let color = body.get_appearance().get_color();
                    Some(color.astro_display())
                }),
            },
            TableColData {
                header: "Age",
                content_closure: Box::new(|body| {
                    let age = body.get_data()?.get_age_at_epoch()?;
                    Some(age.astro_display())
                }),
            },
            TableColData {
                header: "Distance",
                content_closure: Box::new(|body| {
                    Some(body.get_data()?.get_distance_at_epoch().astro_display())
                }),
            },
            TableColData {
                header: "Vis. Mag.",
                content_closure: Box::new(|body| {
                    let illuminance = body.get_appearance().get_illuminance();
                    Some(illuminance.astro_display())
                }),
            },
            TableColData {
                header: "Ecl. Lon.",
                content_closure: Box::new(|body| {
                    let longitude = body.get_appearance().get_pos().spherical.longitude;
                    Some(longitude.astro_display())
                }),
            },
            TableColData {
                header: "Ecl. Lat.",
                content_closure: Box::new(|body| {
                    let latitude = body.get_appearance().get_pos().spherical.latitude;
                    Some(latitude.astro_display())
                }),
            },
            TableColData {
                header: "Const.",
                content_closure: Box::new(|body| {
                    let constellation = body.get_data()?.get_constellation().clone()?;
                    Some(constellation.astro_display())
                }),
            },
            TableColData {
                header: "Lifetime",
                content_closure: Box::new(|body| {
                    let lifetime = body.get_data()?.get_lifetime();
                    Some(lifetime.astro_display())
                }),
            },
            TableColData {
                header: "Fate",
                content_closure: Box::new(|body| {
                    let fate = body.get_data()?.get_fate();
                    Some(fate.astro_display())
                }),
            },
        ]
    }

    pub(super) fn default_supernova_col_data() -> Vec<TableColData<Star>> {
        vec![
            TableColData {
                header: "Star Name",
                content_closure: Box::new(|body| {
                    let name = body.get_appearance().get_name();
                    Some(name.to_string())
                }),
            },
            TableColData {
                header: "Time Until Death",
                content_closure: Box::new(|body| {
                    let time_until_death =
                        body.get_data()?
                            .get_time_until_death(Time::new::<year>(0.))?;
                    Some(time_until_death.astro_display())
                }),
            },
            TableColData {
                header: "Mass",
                content_closure: Box::new(|body| {
                    let mass = body.get_data()?.get_mass_at_epoch()?;
                    Some(mass.astro_display())
                }),
            },
            TableColData {
                header: "Distance",
                content_closure: Box::new(|body| {
                    Some(body.get_data()?.get_distance_at_epoch().astro_display())
                }),
            },
            TableColData {
                header: "Vis. Mag.",
                content_closure: Box::new(|body| {
                    let illuminance = body.get_appearance().get_illuminance();
                    Some(illuminance.astro_display())
                }),
            },
        ]
    }
}
