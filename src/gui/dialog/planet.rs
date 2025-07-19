use astro_coords::direction::Direction;
use astro_utils::{
    astro_display::AstroDisplay,
    color::srgb::sRGBColor,
    planets::{
        derived_data::DerivedPlanetData, orbit_parameters::OrbitParameters,
        physical_parameters::PlanetPhysicalParameters, planet_data::PlanetData,
        random_planets::generate_random_planet,
    },
    stars::data::StarData,
    units::{length::earth_radii, mass::earth_mass},
};
use iced::{
    widget::{text::Shaping, Button, Column, Row, Text},
    Alignment, Element, Length as IcedLength,
};
use uom::si::{
    angle::degree,
    f64::{Angle, Length, Mass, Time},
    length::astronomical_unit,
    time::day,
};

use crate::{
    error::ElenathError,
    gui::{gui_widget::PADDING, message::GuiMessage, shared_widgets::edit},
};

use super::{Dialog, DialogUpdate};

#[derive(Debug, Clone)]
pub(crate) struct PlanetDialog {
    planet: PlanetData,
    planet_index: Option<usize>,
    previous_planet: Option<DerivedPlanetData>,
    central_body: StarData,
    mass_string: String,
    radius_string: String,
    color_string: String,
    geometric_albedo_string: String,
    semi_major_axis_string: String,
    eccentricity_string: String,
    inclination_string: String,
    longitude_of_ascending_node_string: String,
    argument_of_periapsis_string: String,
    siderial_rotation_period_string: String,
    rotation_axis_string: String,
    error: Option<ElenathError>,
}

fn message<F: Fn(String) -> PlanetDialogEvent>(event: F) -> impl Fn(String) -> GuiMessage {
    move |m| GuiMessage::DialogUpdate(DialogUpdate::PlanetUpdated(event(m)))
}

impl PlanetDialog {
    pub(crate) fn edit(
        planet: PlanetData,
        planet_index: usize,
        previous_planet: Option<DerivedPlanetData>,
        central_body: StarData,
    ) -> Result<Self, ElenathError> {
        let mut dialog = PlanetDialog {
            planet: planet.clone(),
            planet_index: Some(planet_index),
            previous_planet,
            central_body,
            mass_string: String::new(),
            radius_string: String::new(),
            color_string: String::new(),
            geometric_albedo_string: String::new(),
            semi_major_axis_string: String::new(),
            eccentricity_string: String::new(),
            inclination_string: String::new(),
            longitude_of_ascending_node_string: String::new(),
            argument_of_periapsis_string: String::new(),
            siderial_rotation_period_string: String::new(),
            rotation_axis_string: String::new(),
            error: None,
        };
        dialog.fill_string_members()?;
        Ok(dialog)
    }

    pub(crate) fn new(central_body: StarData) -> Result<Self, ElenathError> {
        let physical_parameters = PlanetPhysicalParameters::new(
            Mass::new::<earth_mass>(0.),
            Length::new::<earth_radii>(0.),
            0.0,
            sRGBColor::from_sRGB(0., 0., 0.),
            Time::new::<day>(0.),
            Direction::Z,
        );
        let orbital_parameters = OrbitParameters::new(
            Length::new::<astronomical_unit>(0.),
            0.0,
            Angle::new::<degree>(0.),
            Angle::new::<degree>(0.),
            Angle::new::<degree>(0.),
        );
        let planet = PlanetData::new(String::new(), physical_parameters, orbital_parameters);
        let mut dialog = PlanetDialog {
            planet,
            planet_index: None,
            previous_planet: None,
            central_body,
            mass_string: String::new(),
            radius_string: String::new(),
            color_string: String::new(),
            geometric_albedo_string: String::new(),
            semi_major_axis_string: String::new(),
            eccentricity_string: String::new(),
            inclination_string: String::new(),
            longitude_of_ascending_node_string: String::new(),
            argument_of_periapsis_string: String::new(),
            siderial_rotation_period_string: String::new(),
            rotation_axis_string: String::new(),
            error: None,
        };
        dialog.fill_string_members()?;
        Ok(dialog)
    }

    fn fill_string_members(&mut self) -> Result<(), ElenathError> {
        self.mass_string = format!("{:.2}", self.planet.get_mass().get::<earth_mass>());
        self.radius_string = format!("{:.2}", &self.planet.get_radius().get::<earth_radii>());
        self.color_string = serde_json::to_string(self.planet.get_color()).map_err(|e| {
            ElenathError::Generic(format!("Converting planet color to json failed: {:?}", e))
        })?;
        self.geometric_albedo_string = format!("{:.2}", self.planet.get_geometric_albedo());
        self.semi_major_axis_string = format!(
            "{:.2}",
            self.planet
                .get_orbital_parameters()
                .get_semi_major_axis()
                .get::<astronomical_unit>()
        );
        self.eccentricity_string = format!(
            "{:.4}",
            self.planet.get_orbital_parameters().get_eccentricity()
        );
        self.inclination_string = format!(
            "{:.2}",
            self.planet
                .get_orbital_parameters()
                .get_inclination()
                .get::<degree>()
        );
        self.longitude_of_ascending_node_string = format!(
            "{:.2}",
            self.planet
                .get_orbital_parameters()
                .get_longitude_of_ascending_node()
                .get::<degree>()
        );
        self.argument_of_periapsis_string = format!(
            "{:.2}",
            self.planet
                .get_orbital_parameters()
                .get_argument_of_periapsis()
                .get::<degree>()
        );
        self.siderial_rotation_period_string = format!(
            "{:.4}",
            self.planet.get_sideral_rotation_period().get::<day>()
        );
        self.rotation_axis_string = serde_json::to_string(self.planet.get_rotation_axis())
            .map_err(|e| {
                ElenathError::Generic(format!(
                    "Converting planet rotation axPlanetUpdatedis to json failed: {:?}",
                    e
                ))
            })?;
        Ok(())
    }

    fn edit_column(&self) -> Element<'_, GuiMessage> {
        let randomize_message =
            GuiMessage::DialogUpdate(DialogUpdate::PlanetUpdated(PlanetDialogEvent::Randomize));
        let randomize_button = Button::new(Text::new("Randomize")).on_press(randomize_message);

        let name = edit(
            "Name",
            self.planet.get_name(),
            "",
            message(PlanetDialogEvent::NameChanged),
            &Some(self.planet.get_name()),
        );
        let mass = edit(
            "Mass",
            &self.mass_string,
            "Earth Masses",
            message(PlanetDialogEvent::MassChanged),
            &Some(self.planet.get_mass()),
        );
        let radius = edit(
            "Radius",
            &self.radius_string,
            "Earth Radii",
            message(PlanetDialogEvent::RadiusChanged),
            &Some(self.planet.get_radius()),
        );
        let color = edit(
            "Color",
            &self.color_string,
            "",
            message(PlanetDialogEvent::ColorChanged),
            &Some(self.planet.get_color()),
        );
        let geometric_albedo = edit(
            "Geometric Albedo",
            &self.geometric_albedo_string,
            "",
            message(PlanetDialogEvent::GeometricAlbedoChanged),
            &Some(self.planet.get_geometric_albedo()),
        );
        let semi_major_axis = edit(
            "Semi-major Axis",
            &self.semi_major_axis_string,
            "AU",
            message(PlanetDialogEvent::SemiMajorAxisChanged),
            &Some(self.planet.get_orbital_parameters().get_semi_major_axis()),
        );
        let eccentricity = edit(
            "Eccentricity",
            &self.eccentricity_string,
            "",
            message(PlanetDialogEvent::EccentricityChanged),
            &Some(self.planet.get_orbital_parameters().get_eccentricity()),
        );
        let inclination = edit(
            "Inclination",
            &self.inclination_string,
            "°",
            message(PlanetDialogEvent::InclinationChanged),
            &Some(self.planet.get_orbital_parameters().get_inclination()),
        );
        let longitude_of_ascending_node = edit(
            "Ascending Node",
            &self.longitude_of_ascending_node_string,
            "°",
            message(PlanetDialogEvent::LongitudeOfAscendingNodeChanged),
            &Some(
                self.planet
                    .get_orbital_parameters()
                    .get_longitude_of_ascending_node(),
            ),
        );
        let argument_of_periapsis = edit(
            "Arg. of Periapsis",
            &self.argument_of_periapsis_string,
            "°",
            message(PlanetDialogEvent::ArgumentOfPeriapsisChanged),
            &Some(
                self.planet
                    .get_orbital_parameters()
                    .get_argument_of_periapsis(),
            ),
        );
        let siderial_rotation_period = edit(
            "Siderial Day",
            &self.siderial_rotation_period_string,
            "Earth Days",
            message(PlanetDialogEvent::SiderialRotationPeriodChanged),
            &Some(self.planet.get_sideral_rotation_period()),
        );
        let rotation_axis = edit(
            "Rotation Axis",
            &self.rotation_axis_string,
            "",
            message(PlanetDialogEvent::RotationAxisChanged),
            &Some(self.planet.get_rotation_axis()),
        );

        let submit_button = Button::new(Text::new("Submit")).on_press(GuiMessage::DialogSubmit);

        Column::new()
            .push(randomize_button)
            .push(name)
            .push(mass)
            .push(radius)
            .push(color)
            .push(geometric_albedo)
            .push(semi_major_axis)
            .push(eccentricity)
            .push(inclination)
            .push(longitude_of_ascending_node)
            .push(argument_of_periapsis)
            .push(siderial_rotation_period)
            .push(rotation_axis)
            .push(submit_button)
            .spacing(PADDING)
            .width(IcedLength::Fill)
            .align_x(Alignment::Center)
            .into()
    }

    fn additional_info_column(&self) -> Element<'_, GuiMessage> {
        let derived_data = DerivedPlanetData::new(
            &self.planet,
            &self.central_body,
            self.previous_planet.as_ref(),
        );

        let mut col = Column::new();
        match derived_data {
            Ok(derived_data) => {
                let density_text = Text::new(
                    "Density: ".to_string() + &derived_data.get_density().astro_display(),
                )
                .shaping(Shaping::Advanced);

                let surface_gravity_text = Text::new(
                    "Surface Gravity: ".to_string()
                        + &derived_data.get_surface_gravity().astro_display(),
                )
                .shaping(Shaping::Advanced);

                let escape_velocity_text = Text::new(
                    "Escape Velocity: ".to_string()
                        + &derived_data.get_escape_velocity().astro_display(),
                )
                .shaping(Shaping::Advanced);

                let orbital_period_text = Text::new(
                    "Orbital Period: ".to_string()
                        + &derived_data.get_orbital_period().astro_display(),
                )
                .shaping(Shaping::Advanced);

                let orbital_resonance_text = Text::new(
                    "Orbital Resonance: ".to_string()
                        + &derived_data.get_orbital_resonance().astro_display(),
                )
                .shaping(Shaping::Advanced);

                let synodic_period_text = Text::new(
                    "Mean Synodic Day: ".to_string()
                        + &derived_data.get_mean_synodic_day().astro_display(),
                )
                .shaping(Shaping::Advanced);

                let axial_tilt_text = Text::new(
                    "Axial Tilt: ".to_string() + &derived_data.get_axial_tilt().astro_display(),
                )
                .shaping(Shaping::Advanced);

                let black_body_temperature_text = Text::new(
                    "Black Body Temperature: ".to_string()
                        + &derived_data.get_black_body_temperature().astro_display(),
                )
                .shaping(Shaping::Advanced);
                col = col
                    .push(density_text)
                    .push(surface_gravity_text)
                    .push(escape_velocity_text)
                    .push(orbital_period_text)
                    .push(orbital_resonance_text)
                    .push(synodic_period_text)
                    .push(axial_tilt_text)
                    .push(black_body_temperature_text);
            }
            Err(e) => {
                let message = Text::new(format!("Error: {:?}", e));
                col = col.push(message);
            }
        }

        col.spacing(PADDING)
            .width(IcedLength::Fill)
            .align_x(Alignment::Center)
            .into()
    }
}

impl Dialog for PlanetDialog {
    fn header(&self) -> String {
        match self.planet_index {
            Some(index) => format!("Edit Planet {}", index),
            None => "Create Planet".to_string(),
        }
    }

    fn body<'a>(&'a self) -> Element<'a, GuiMessage> {
        Row::new()
            .push(self.edit_column())
            .push(self.additional_info_column())
            .into()
    }

    fn update(&mut self, event: DialogUpdate) {
        if let DialogUpdate::PlanetUpdated(event) = event {
            match event {
                PlanetDialogEvent::NameChanged(name) => {
                    self.planet.set_name(name);
                }
                PlanetDialogEvent::MassChanged(mass_string) => {
                    if let Ok(mass) = mass_string.parse::<f64>() {
                        self.planet.set_mass(Mass::new::<earth_mass>(mass));
                        self.mass_string = mass_string;
                    }
                }
                PlanetDialogEvent::RadiusChanged(radius_string) => {
                    if let Ok(radius) = radius_string.parse::<f64>() {
                        self.planet.set_radius(Length::new::<earth_radii>(radius));
                        self.radius_string = radius_string;
                    }
                }
                PlanetDialogEvent::ColorChanged(color_string) => {
                    if let Ok(color) = serde_json::from_str::<sRGBColor>(&color_string) {
                        self.planet.set_color(color);
                    }
                    self.color_string = color_string;
                }
                PlanetDialogEvent::GeometricAlbedoChanged(geometric_albedo_string) => {
                    if let Ok(geometric_albedo) = geometric_albedo_string.parse::<f64>() {
                        self.planet.set_geometric_albedo(geometric_albedo);
                        self.geometric_albedo_string = geometric_albedo_string;
                    }
                }
                PlanetDialogEvent::SemiMajorAxisChanged(semi_major_axis_string) => {
                    if let Ok(semi_major_axis) = semi_major_axis_string.parse::<f64>() {
                        self.planet
                            .set_semi_major_axis(Length::new::<astronomical_unit>(semi_major_axis));
                        self.semi_major_axis_string = semi_major_axis_string;
                    }
                }
                PlanetDialogEvent::EccentricityChanged(eccentricity_string) => {
                    if let Ok(eccentricity) = eccentricity_string.parse::<f64>() {
                        self.planet.set_eccentricity(eccentricity);
                        self.eccentricity_string = eccentricity_string;
                    }
                }
                PlanetDialogEvent::InclinationChanged(inclination_string) => {
                    if let Ok(inclination) = inclination_string.parse::<f64>() {
                        self.planet
                            .set_inclination(Angle::new::<degree>(inclination));
                        self.inclination_string = inclination_string;
                    }
                }
                PlanetDialogEvent::LongitudeOfAscendingNodeChanged(
                    longitude_of_ascending_node_string,
                ) => {
                    if let Ok(longitude_of_ascending_node) =
                        longitude_of_ascending_node_string.parse::<f64>()
                    {
                        self.planet
                            .set_longitude_of_ascending_node(Angle::new::<degree>(
                                longitude_of_ascending_node,
                            ));
                        self.longitude_of_ascending_node_string =
                            longitude_of_ascending_node_string;
                    }
                }
                PlanetDialogEvent::ArgumentOfPeriapsisChanged(argument_of_periapsis_string) => {
                    if let Ok(argument_of_periapsis) = argument_of_periapsis_string.parse::<f64>() {
                        self.planet
                            .set_argument_of_periapsis(Angle::new::<degree>(argument_of_periapsis));
                        self.argument_of_periapsis_string = argument_of_periapsis_string;
                    }
                }
                PlanetDialogEvent::SiderialRotationPeriodChanged(
                    siderial_rotation_period_string,
                ) => {
                    if let Ok(siderial_rotation_period) =
                        siderial_rotation_period_string.parse::<f64>()
                    {
                        self.planet.set_sideral_rotation_period(Time::new::<day>(
                            siderial_rotation_period,
                        ));
                        self.siderial_rotation_period_string = siderial_rotation_period_string;
                    }
                }
                PlanetDialogEvent::RotationAxisChanged(rotation_axis_string) => {
                    if let Ok(axis) = serde_json::from_str::<Direction>(&rotation_axis_string) {
                        if let Ok(rotation_axis) = Direction::new(axis.x(), axis.y(), axis.z()) {
                            self.planet.set_rotation_axis(rotation_axis);
                        }
                    }
                    self.rotation_axis_string = rotation_axis_string;
                }
                PlanetDialogEvent::Randomize => {
                    let name = self.planet.get_name().clone();
                    self.planet = generate_random_planet();
                    self.planet.set_name(name);
                    if let Err(e) = self.fill_string_members() {
                        self.error = Some(e);
                    };
                }
            }
        }
    }

    fn on_submit(&self) -> GuiMessage {
        match self.planet_index {
            Some(index) => GuiMessage::PlanetEdited(index, self.planet.clone()),
            None => GuiMessage::NewPlanet(self.planet.clone()),
        }
    }

    fn get_error(&self) -> Option<ElenathError> {
        self.error.clone()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum PlanetDialogEvent {
    NameChanged(String),
    MassChanged(String),
    RadiusChanged(String),
    ColorChanged(String),
    GeometricAlbedoChanged(String),
    SemiMajorAxisChanged(String),
    EccentricityChanged(String),
    InclinationChanged(String),
    LongitudeOfAscendingNodeChanged(String),
    ArgumentOfPeriapsisChanged(String),
    SiderialRotationPeriodChanged(String),
    RotationAxisChanged(String),
    Randomize,
}

impl From<PlanetDialogEvent> for GuiMessage {
    fn from(event: PlanetDialogEvent) -> Self {
        GuiMessage::DialogUpdate(DialogUpdate::PlanetUpdated(event))
    }
}
