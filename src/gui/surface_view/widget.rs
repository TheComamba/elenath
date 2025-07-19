use crate::gui::{
    gui_widget::{BIG_COLUMN_WIDTH, PADDING},
    message::GuiMessage,
    shared_widgets::control_field,
};
use astro_utils::{astro_display::AstroDisplay, units::angle::normalized_angle};
use iced::{
    widget::{canvas, Column},
    Alignment, Element, Length,
};
use std::f64::consts::PI;
use uom::si::{
    angle::degree,
    f64::{Angle, SolidAngle},
    solid_angle::steradian,
};

#[inline(always)]
fn human_eye_opening_angle() -> SolidAngle {
    SolidAngle::new::<steradian>(1.)
}

#[inline(always)]
fn angle_step() -> Angle {
    Angle::new::<degree>(10.)
}

#[inline(always)]
fn srad_step() -> SolidAngle {
    SolidAngle::new::<steradian>(0.1)
}

pub(crate) struct SurfaceViewState {
    pub(super) background_cache: canvas::Cache,
    pub(super) bodies_cache: canvas::Cache,
    pub(super) surface_longitude: Angle,
    pub(super) surface_latitude: Angle,
    pub(super) view_longitude: Angle,
    pub(super) view_latitude: Angle,
    pub(super) viewport_opening_angle: SolidAngle,
}

#[derive(Debug, Clone)]
pub(crate) enum SurfaceViewUpdate {
    SurfaceLongitude(Angle),
    SurfaceLatitude(Angle),
    ViewLongitude(Angle),
    ViewLatitude(Angle),
    ViewportOpeningAngle(SolidAngle),
}

impl From<SurfaceViewUpdate> for GuiMessage {
    fn from(val: SurfaceViewUpdate) -> Self {
        GuiMessage::UpdateSurfaceView(val)
    }
}

impl SurfaceViewState {
    pub(crate) fn new() -> Self {
        SurfaceViewState {
            background_cache: canvas::Cache::default(),
            bodies_cache: canvas::Cache::default(),
            surface_longitude: Angle::new::<degree>(0.),
            surface_latitude: Angle::new::<degree>(0.),
            view_longitude: Angle::new::<degree>(0.),
            view_latitude: Angle::new::<degree>(90.),
            viewport_opening_angle: human_eye_opening_angle(),
        }
    }

    pub(crate) fn update(&mut self, message: SurfaceViewUpdate) {
        match message {
            SurfaceViewUpdate::SurfaceLongitude(mut longitude) => {
                longitude = normalized_angle(longitude);
                self.surface_longitude = longitude;
            }
            SurfaceViewUpdate::SurfaceLatitude(mut latitude) => {
                if latitude.get::<degree>() < -90. {
                    latitude = Angle::new::<degree>(-90.);
                } else if latitude.get::<degree>() > 90. {
                    latitude = Angle::new::<degree>(90.);
                }
                self.surface_latitude = latitude;
            }
            SurfaceViewUpdate::ViewLongitude(mut longitude) => {
                longitude = normalized_angle(longitude);
                self.view_longitude = longitude;
            }
            SurfaceViewUpdate::ViewLatitude(mut latitude) => {
                if latitude < angle_step() {
                    latitude = angle_step();
                } else if latitude.get::<degree>() > 90. {
                    latitude = Angle::new::<degree>(90.);
                }
                self.view_latitude = latitude;
            }
            SurfaceViewUpdate::ViewportOpeningAngle(mut angle) => {
                if angle < srad_step() {
                    angle = srad_step();
                } else if angle.get::<steradian>() > 2. * PI {
                    angle = SolidAngle::new::<steradian>(2. * PI);
                }
                self.viewport_opening_angle = angle;
            }
        }
    }

    pub(crate) fn redraw(&mut self) {
        self.bodies_cache.clear();
    }

    pub(crate) fn control_field(&self) -> Element<'_, GuiMessage> {
        let surface_long = self.surface_longitude;
        let surface_longitude_control_field = control_field(
            "Surface Longitude:",
            surface_long.astro_display(),
            SurfaceViewUpdate::SurfaceLongitude(surface_long - angle_step()),
            SurfaceViewUpdate::SurfaceLongitude(surface_long + angle_step()),
        );

        let surface_lat = self.surface_latitude;
        let surface_latitude_control_field = control_field(
            "Surface Latitude:",
            surface_lat.astro_display(),
            SurfaceViewUpdate::SurfaceLatitude(surface_lat - angle_step()),
            SurfaceViewUpdate::SurfaceLatitude(surface_lat + angle_step()),
        );

        let view_long = self.view_longitude;
        let view_longitude_control_field = control_field(
            "Observer Longitude:",
            view_long.astro_display(),
            SurfaceViewUpdate::ViewLongitude(view_long - angle_step()),
            SurfaceViewUpdate::ViewLongitude(view_long + angle_step()),
        );

        let view_lat = self.view_latitude;
        let view_latitude_control_field = control_field(
            "Observer Latitude:",
            view_lat.astro_display(),
            SurfaceViewUpdate::ViewLatitude(view_lat - angle_step()),
            SurfaceViewUpdate::ViewLatitude(view_lat + angle_step()),
        );

        let viewport_angle = self.viewport_opening_angle;
        let viewport_angle_control_field = control_field(
            "Viewport Opening Angle:",
            viewport_angle.astro_display(),
            SurfaceViewUpdate::ViewportOpeningAngle(viewport_angle - srad_step()),
            SurfaceViewUpdate::ViewportOpeningAngle(viewport_angle + srad_step()),
        );
        Column::new()
            .push(surface_longitude_control_field)
            .push(surface_latitude_control_field)
            .push(view_longitude_control_field)
            .push(view_latitude_control_field)
            .push(viewport_angle_control_field)
            .width(Length::Fixed(BIG_COLUMN_WIDTH))
            .align_x(Alignment::Center)
            .spacing(PADDING)
            .into()
    }
}
