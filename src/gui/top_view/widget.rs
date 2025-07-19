use astro_coords::ecliptic::Ecliptic;
use astro_utils::{astro_display::AstroDisplay, units::angle::normalized_angle};
use iced::{
    widget::{canvas::Cache, Column},
    Alignment, Element, Length as IcedLength,
};
use uom::si::{
    angle::degree,
    f64::{Angle, Length},
    length::astronomical_unit,
};

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
            length_per_pixel: Length::new::<astronomical_unit>(0.01),
            view_ecliptic: Ecliptic::z_direction(),
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
                if latitude.get::<degree>() < -90. {
                    latitude = Angle::new::<degree>(-90.);
                } else if latitude.get::<degree>() > 90. {
                    latitude = Angle::new::<degree>(90.);
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
        let view_angle_step: Angle = Angle::new::<degree>(10.0);
        let view_longitude = self.view_ecliptic.spherical.longitude;
        let view_longitude_control_field = control_field(
            "View longitude:",
            view_longitude.astro_display(),
            TopViewUpdate::ViewLongitude(view_longitude - view_angle_step),
            TopViewUpdate::ViewLongitude(view_longitude + view_angle_step),
        );
        let view_latitude = self.view_ecliptic.spherical.latitude;
        let view_latitude_control_field = control_field(
            "View latitude:",
            view_latitude.astro_display(),
            TopViewUpdate::ViewLatitude(view_latitude - view_angle_step),
            TopViewUpdate::ViewLatitude(view_latitude + view_angle_step),
        );
        Column::new()
            .push(length_scale_control_field)
            .push(view_longitude_control_field)
            .push(view_latitude_control_field)
            .width(IcedLength::Fixed(BIG_COLUMN_WIDTH))
            .align_x(Alignment::Center)
            .spacing(PADDING)
            .into()
    }
}
