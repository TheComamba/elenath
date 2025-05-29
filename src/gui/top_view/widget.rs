use astro_coords::ecliptic::Ecliptic;
use astro_utils::{astro_display::AstroDisplay, units::angle::normalized_angle};
use iced::{
    widget::{canvas::Cache, Column},
    Alignment, Element, Length,
};
use std::f64::consts::PI;
use uom::si::f64::Angle;

use crate::gui::{
    gui_widget::{BIG_COLUMN_WIDTH, PADDING},
    message::GuiMessage,
    shared_widgets::control_field,
};

pub(crate) struct TopViewState {
    pub(super) background_cache: Cache,
    pub(super) bodies_cache: Cache,
    pub(super) scale_cache: Cache,
    pub(super) length_per_pixel: Length,
    pub(super) view_ecliptic: Ecliptic,
}

#[derive(Debug, Clone)]
pub(crate) enum TopViewUpdate {
    LengthScale(Length),
    ViewLongitude(Angle),
    ViewLatitude(Angle),
}

impl From<TopViewUpdate> for GuiMessage {
    fn from(val: TopViewUpdate) -> Self {
        GuiMessage::UpdateTopView(val)
    }
}

impl TopViewState {
    pub(crate) fn new() -> Self {
        TopViewState {
            background_cache: Cache::default(),
            bodies_cache: Cache::default(),
            scale_cache: Cache::default(),
            length_per_pixel: Distance::from_au(0.01),
            view_ecliptic: Ecliptic::Z_DIRECTION,
        }
    }

    pub(crate) fn update(&mut self, message: TopViewUpdate) {
        match message {
            TopViewUpdate::LengthScale(length_per_pixel) => {
                self.length_per_pixel = length_per_pixel;
            }
            TopViewUpdate::ViewLongitude(mut longitude) => {
                longitude = normalized_angle(longitude);
                self.view_ecliptic.spherical.longitude = longitude;
            }
            TopViewUpdate::ViewLatitude(mut latitude) => {
                if latitude.to_degrees() < -90. {
                    latitude = Angle::from_degrees(-90.);
                } else if latitude.to_degrees() > 90. {
                    latitude = Angle::from_degrees(90.);
                }
                self.view_ecliptic.spherical.latitude = latitude;
            }
        }
    }

    pub(crate) fn redraw(&mut self) {
        self.bodies_cache.clear();
        self.scale_cache.clear();
    }

    pub(crate) fn control_field(&self) -> Element<'_, GuiMessage> {
        let length_scale_control_field = control_field(
            "Length per 100px:",
            (self.length_per_pixel * 100.).astro_display(),
            TopViewUpdate::LengthScale(self.length_per_pixel / 2.),
            TopViewUpdate::LengthScale(self.length_per_pixel * 2.),
        );
        const VIEW_ANGLE_STEP: Angle = Angle {
            rad: 10. * 2. * PI / 360.,
        };
        let view_longitude = self.view_ecliptic.spherical.longitude;
        let view_longitude_control_field = control_field(
            "View longitude:",
            view_longitude.astro_display(),
            TopViewUpdate::ViewLongitude(view_longitude - VIEW_ANGLE_STEP),
            TopViewUpdate::ViewLongitude(view_longitude + VIEW_ANGLE_STEP),
        );
        let view_latitude = self.view_ecliptic.spherical.latitude;
        let view_latitude_control_field = control_field(
            "View latitude:",
            view_latitude.astro_display(),
            TopViewUpdate::ViewLatitude(view_latitude - VIEW_ANGLE_STEP),
            TopViewUpdate::ViewLatitude(view_latitude + VIEW_ANGLE_STEP),
        );
        Column::new()
            .push(length_scale_control_field)
            .push(view_longitude_control_field)
            .push(view_latitude_control_field)
            .width(Length::Fixed(BIG_COLUMN_WIDTH))
            .align_x(Alignment::Center)
            .spacing(PADDING)
            .into()
    }
}
