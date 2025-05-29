use astro_coords::{cartesian::Cartesian, ecliptic::Ecliptic, spherical::Spherical};
use astro_utils::{
    astro_display::AstroDisplay,
    stars::{
        data::StarData, evolution::StarDataEvolution, physical_parameters::StarPhysicalParameters,
        random::random_stars::generate_random_star,
    },
    units::luminous_intensity::{
        absolute_magnitude_to_luminous_intensity, luminous_intensity_to_absolute_magnitude,
    },
};
use iced::{
    widget::{text::Shaping, Button, Column, Row, Text},
    Alignment, Element,
};
use uom::si::{
    f64::{Angle, Length, Time},
    length::light_year,
};

use crate::gui::{gui_widget::PADDING, message::GuiMessage, shared_widgets::edit};

use super::{Dialog, DialogUpdate, ElenathError};

#[derive(Debug, Clone)]
pub(crate) struct StarDialog {
    star_dialog_type: StarDialogType,
    star: StarData,
    star_index: Option<usize>,
    time_since_epoch: Time,
    mass_string: String,
    radius_string: String,
    luminosity_string: String,
    temperature_string: String,
    age_string: String,
    distance_string: String,
    longitude_string: String,
    latitude_string: String,
    error: Option<ElenathError>,
}

#[derive(Debug, Clone, PartialEq)]
enum StarDialogType {
    New,
    Edit,
}

const DEFAULT_ECLIPTIC: Ecliptic = Ecliptic {
    spherical: Spherical::new(Angle { rad: 0. }, Angle { rad: 0. }),
};

impl StarDialog {
    pub(crate) fn new(time_since_epoch: Time) -> Self {
        let params = StarPhysicalParameters::new(None, None, LUMINOSITY_ZERO, TEMPERATURE_ZERO);
        let star = StarData::new(
            String::new(),
            None,
            params,
            Cartesian::ORIGIN,
            StarDataEvolution::NONE,
        );
        let mut dialog = StarDialog {
            star_dialog_type: StarDialogType::New,
            star,
            star_index: None,
            time_since_epoch,
            mass_string: String::new(),
            radius_string: String::new(),
            luminosity_string: String::new(),
            temperature_string: String::new(),
            age_string: String::new(),
            distance_string: String::new(),
            longitude_string: String::new(),
            latitude_string: String::new(),
            error: None,
        };
        dialog.fill_string_members();
        dialog
    }

    pub(crate) fn edit(star: StarData, star_index: Option<usize>, time_since_epoch: Time) -> Self {
        let mut dialog = StarDialog {
            star_dialog_type: StarDialogType::Edit,
            star,
            star_index,
            time_since_epoch,
            mass_string: String::new(),
            radius_string: String::new(),
            luminosity_string: String::new(),
            temperature_string: String::new(),
            age_string: String::new(),
            distance_string: String::new(),
            longitude_string: String::new(),
            latitude_string: String::new(),
            error: None,
        };
        dialog.fill_string_members();
        dialog
    }

    fn fill_string_members(&mut self) {
        self.mass_string = self
            .star
            .get_mass_at_epoch()
            .map(|mass| format!("{:.2}", mass.to_solar_mass()))
            .unwrap_or_default();
        self.radius_string = self
            .star
            .get_radius_at_epoch()
            .map(|radius| format!("{:.2}", distance_to_sun_radii(&radius)))
            .unwrap_or_default();
        self.luminosity_string = format!(
            "{:.2}",
            luminous_intensity_to_absolute_magnitude(self.star.get_luminous_intensity_at_epoch())
        );
        self.temperature_string = format!("{:.0}", self.star.get_temperature_at_epoch().to_K());
        self.age_string = self
            .star
            .get_age_at_epoch()
            .map(|age| format!("{:.2}", age.to_Gyr()))
            .unwrap_or_default();
        self.distance_string = format!("{:.2}", self.star.get_distance_at_epoch().to_lyr());
        self.longitude_string = format!(
            "{:.2}",
            self.star
                .get_pos_at_epoch()
                .to_ecliptic()
                .unwrap_or(DEFAULT_ECLIPTIC)
                .spherical
                .longitude
                .to_degrees()
        );
        self.latitude_string = format!(
            "{:.2}",
            self.star
                .get_pos_at_epoch()
                .to_ecliptic()
                .unwrap_or(DEFAULT_ECLIPTIC)
                .spherical
                .latitude
                .to_degrees()
        );
    }

    fn edit_column(&self) -> Element<'_, GuiMessage> {
        let rand_event =
            GuiMessage::DialogUpdate(DialogUpdate::StarUpdated(StarDialogEvent::Randomize));
        let randomize_button = Button::new(Text::new("Randomize")).on_press(rand_event);

        let name = edit(
            "Name",
            self.star.get_name(),
            "",
            message(StarDialogEvent::NameChanged),
            &Some(self.star.get_name()),
        );
        let mass = edit(
            "Mass at epoch",
            &self.mass_string,
            "Solar Masses",
            message(StarDialogEvent::MassChanged),
            &self.star.get_mass_at_epoch(),
        );
        let radius = edit(
            "Radius at epoch",
            &self.radius_string,
            "Solar Radii",
            message(StarDialogEvent::RadiusChanged),
            &self.star.get_radius_at_epoch(),
        );
        let luminosity = edit(
            "Luminosity at epoch",
            &self.luminosity_string,
            "mag",
            message(StarDialogEvent::LuminosityChanged),
            &Some(self.star.get_luminous_intensity_at_epoch()),
        );
        let temperature = edit(
            "Temperature at epoch",
            &self.temperature_string,
            "K",
            message(StarDialogEvent::TemperatureChanged),
            &Some(self.star.get_temperature_at_epoch()),
        );
        let age = edit(
            "Age at epoch",
            &self.age_string,
            "Gyr",
            message(StarDialogEvent::AgeChanged),
            &self.star.get_age_at_epoch(),
        );
        let distance = edit(
            "Distance at epoch",
            &self.distance_string,
            "ly",
            message(StarDialogEvent::DistanceChanged),
            &Some(self.star.get_distance_at_epoch()),
        );
        let longitude = edit(
            "Longitude at epoch",
            &self.longitude_string,
            "°",
            message(StarDialogEvent::LongitudeChanged),
            &Some(
                self.star
                    .get_pos_at_epoch()
                    .to_ecliptic()
                    .unwrap_or(DEFAULT_ECLIPTIC)
                    .spherical
                    .longitude,
            ),
        );
        let latitude = edit(
            "Latitude at epoch",
            &self.latitude_string,
            "°",
            message(StarDialogEvent::LatitudeChanged),
            &Some(
                self.star
                    .get_pos_at_epoch()
                    .to_ecliptic()
                    .unwrap_or(DEFAULT_ECLIPTIC)
                    .spherical
                    .latitude,
            ),
        );
        let constellation = edit(
            "Constellation",
            &self.star.get_constellation().clone().unwrap_or_default(),
            "",
            message(StarDialogEvent::ConstellationChanged),
            self.star.get_constellation(),
        );

        let submit_button = Button::new(Text::new("Submit")).on_press(GuiMessage::DialogSubmit);

        let mut col = Column::new()
            .push(randomize_button)
            .push(name)
            .push(mass)
            .push(radius)
            .push(luminosity)
            .push(temperature)
            .push(age);
        if !self.is_central_body() {
            col = col
                .push(distance)
                .push(longitude)
                .push(latitude)
                .push(constellation);
        }
        col.push(submit_button)
            .spacing(PADDING)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .into()
    }

    fn additional_info_column(&self) -> Element<'_, GuiMessage> {
        let appearance = self.star.to_star_appearance(self.time_since_epoch);

        let illuminance =
            Text::new("Illuminance: ".to_string() + &appearance.get_illuminance().astro_display())
                .shaping(Shaping::Advanced);

        let color = Text::new("Color: ".to_string() + &appearance.get_color().astro_display())
            .shaping(Shaping::Advanced);

        let mass_per_year = Text::new(
            "Mass Change per Millenium: ".to_string()
                + &(self.star.get_evolution().get_lifestage_mass_per_year() * 1000.)
                    .astro_display(),
        )
        .shaping(Shaping::Advanced);

        let radius_per_year: Text = Text::new(
            "Radius Change per Millenium: ".to_string()
                + &(self.star.get_evolution().get_lifestage_radius_per_year() * 1000.)
                    .astro_display(),
        )
        .shaping(Shaping::Advanced);

        let temperature_per_year: Text = Text::new(
            "Temperature Change per Millenium: ".to_string()
                + &(self
                    .star
                    .get_evolution()
                    .get_lifestage_temperature_per_year()
                    * 1000.)
                    .astro_display(),
        )
        .shaping(Shaping::Advanced);

        let luminous_intensity_per_year = Text::new(
            "Luminous Intensity Change per Millenium: ".to_string()
                + &(self
                    .star
                    .get_evolution()
                    .get_lifestage_luminous_intensity_per_year()
                    * 1000.)
                    .astro_display(),
        )
        .shaping(Shaping::Advanced);

        let current_mass = Text::new(
            "Current Mass: ".to_string()
                + &self.star.get_mass(self.time_since_epoch).astro_display(),
        )
        .shaping(Shaping::Advanced);

        let current_radius = Text::new(
            "Current Radius: ".to_string()
                + &self.star.get_radius(self.time_since_epoch).astro_display(),
        )
        .shaping(Shaping::Advanced);

        let current_temperature = Text::new(
            "Current Temperature: ".to_string()
                + &self
                    .star
                    .get_temperature(self.time_since_epoch)
                    .astro_display(),
        )
        .shaping(Shaping::Advanced);

        let current_luminous_intensity = Text::new(
            "Current Luminous Intensity: ".to_string()
                + &self
                    .star
                    .get_luminous_intensity(self.time_since_epoch)
                    .astro_display(),
        )
        .shaping(Shaping::Advanced);

        let current_age = Text::new(
            "Current Age: ".to_string() + &self.star.get_age(self.time_since_epoch).astro_display(),
        )
        .shaping(Shaping::Advanced);

        let current_distance = Text::new(
            "Current Distance: ".to_string()
                + &self
                    .star
                    .get_distance(self.time_since_epoch)
                    .astro_display(),
        )
        .shaping(Shaping::Advanced);

        let current_longitude = Text::new(
            "Current Longitude: ".to_string()
                + &self
                    .star
                    .get_pos(self.time_since_epoch)
                    .to_ecliptic()
                    .unwrap_or(DEFAULT_ECLIPTIC)
                    .spherical
                    .longitude
                    .astro_display(),
        )
        .shaping(Shaping::Advanced);

        let current_latitude = Text::new(
            "Current Latitude: ".to_string()
                + &self
                    .star
                    .get_pos(self.time_since_epoch)
                    .to_ecliptic()
                    .unwrap_or(DEFAULT_ECLIPTIC)
                    .spherical
                    .latitude
                    .astro_display(),
        )
        .shaping(Shaping::Advanced);

        Column::new()
            .push(illuminance)
            .push(color)
            .push(mass_per_year)
            .push(radius_per_year)
            .push(temperature_per_year)
            .push(luminous_intensity_per_year)
            .push(current_mass)
            .push(current_radius)
            .push(current_temperature)
            .push(current_luminous_intensity)
            .push(current_age)
            .push(current_distance)
            .push(current_longitude)
            .push(current_latitude)
            .spacing(PADDING)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .into()
    }

    fn is_central_body(&self) -> bool {
        self.star_dialog_type == StarDialogType::Edit && self.star_index.is_none()
    }
}

impl Dialog for StarDialog {
    fn header(&self) -> String {
        match self.star_index {
            Some(index) => format!("Edit Star {}", index),
            None => "Create Star".to_string(),
        }
    }

    fn body<'a>(&'a self) -> Element<'a, GuiMessage> {
        Row::new()
            .push(self.edit_column())
            .push(self.additional_info_column())
            .into()
    }

    fn update(&mut self, message: super::DialogUpdate) {
        if let DialogUpdate::StarUpdated(event) = message {
            match event {
                StarDialogEvent::NameChanged(name) => {
                    self.star.set_name(name);
                }
                StarDialogEvent::MassChanged(mass_string) => {
                    if let Ok(mass) = mass_string.parse::<f64>() {
                        self.star.set_mass_at_epoch(Some(mass * SOLAR_MASS));
                    }
                    self.mass_string = mass_string;
                }
                StarDialogEvent::RadiusChanged(radius_string) => {
                    if let Ok(radius) = radius_string.parse::<f64>() {
                        self.star.set_radius_at_epoch(Some(radius * SOLAR_RADIUS));
                    }
                    self.radius_string = radius_string;
                }
                StarDialogEvent::LuminosityChanged(luminosity_string) => {
                    if let Ok(luminosity) = luminosity_string.parse::<f64>() {
                        self.star.set_luminous_intensity_at_epoch(
                            absolute_magnitude_to_luminous_intensity(luminosity),
                        );
                    }
                    self.luminosity_string = luminosity_string;
                }
                StarDialogEvent::TemperatureChanged(temperature_string) => {
                    if let Ok(temperature) = temperature_string.parse::<f64>() {
                        self.star
                            .set_temperature_at_epoch(Temperature::from_K(temperature));
                    }
                    self.temperature_string = temperature_string;
                }
                StarDialogEvent::AgeChanged(age_string) => {
                    if let Ok(age) = age_string.parse::<f64>() {
                        self.star.set_age_at_epoch(Some(Time::from_Gyr(age)));
                    }
                    self.age_string = age_string;
                }
                StarDialogEvent::DistanceChanged(distance_string) => {
                    if let Ok(distance) = distance_string.parse::<f64>() {
                        self.star
                            .set_distance_at_epoch(Length::new::<light_year>(distance));
                    }
                    self.distance_string = distance_string;
                }
                StarDialogEvent::LongitudeChanged(longitude_string) => {
                    if let Ok(longitude) = longitude_string.parse::<f64>() {
                        let mut pos = self
                            .star
                            .get_pos_at_epoch()
                            .to_ecliptic()
                            .unwrap_or(DEFAULT_ECLIPTIC);
                        pos.spherical.longitude = Angle::from_degrees(longitude);
                        let pos = pos
                            .to_direction()
                            .to_cartesian(self.star.get_distance_at_epoch());
                        self.star.set_pos_at_epoch(pos);
                    }
                    self.longitude_string = longitude_string;
                }
                StarDialogEvent::LatitudeChanged(latitude_string) => {
                    if let Ok(latitude) = latitude_string.parse::<f64>() {
                        let mut pos = self
                            .star
                            .get_pos_at_epoch()
                            .to_ecliptic()
                            .unwrap_or(DEFAULT_ECLIPTIC);
                        pos.spherical.latitude = Angle::from_degrees(latitude);
                        let pos = pos
                            .to_direction()
                            .to_cartesian(self.star.get_distance_at_epoch());
                        self.star.set_pos_at_epoch(pos);
                    }
                    self.latitude_string = latitude_string;
                }
                StarDialogEvent::ConstellationChanged(constellation) => {
                    if constellation.is_empty() {
                        self.star.set_constellation(None);
                    } else {
                        self.star.set_constellation(Some(constellation));
                    }
                }
                StarDialogEvent::Randomize => {
                    let max_distance = Length::new::<light_year>(2000.);
                    let name = self.star.get_name().clone();
                    self.star = match generate_random_star(Some(max_distance)) {
                        Ok(star) => star,
                        Err(e) => {
                            self.error = Some(e.into());
                            return;
                        }
                    };
                    self.star.set_name(name);
                    self.fill_string_members();
                }
            }
        }
    }

    fn on_submit(&self) -> GuiMessage {
        match self.star_dialog_type {
            StarDialogType::Edit => {
                let mut star = self.star.clone();
                if self.is_central_body() {
                    star.set_distance_at_epoch(DISTANCE_ZERO);
                }
                return GuiMessage::StarEdited(self.star_index, star);
            }
            StarDialogType::New => return GuiMessage::NewStar(self.star.clone()),
        }
    }

    fn get_error(&self) -> Option<super::ElenathError> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum StarDialogEvent {
    NameChanged(String),
    MassChanged(String),
    RadiusChanged(String),
    LuminosityChanged(String),
    TemperatureChanged(String),
    AgeChanged(String),
    DistanceChanged(String),
    LongitudeChanged(String),
    LatitudeChanged(String),
    ConstellationChanged(String),
    Randomize,
}

fn message<F: Fn(String) -> StarDialogEvent>(event: F) -> impl Fn(String) -> GuiMessage {
    move |m| GuiMessage::DialogUpdate(DialogUpdate::StarUpdated(event(m)))
}
