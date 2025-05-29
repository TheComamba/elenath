use self::dialog::Dialog;
use self::gui_widget::GuiViewMode;
use self::surface_view::widget::SurfaceViewState;
use self::table_view::widget::TableViewState;
use self::top_view::widget::TopViewState;
use crate::model::celestial_system::CelestialSystem;
use crate::model::planet::Planet;
use astro_utils::planets::planet_data::PlanetData;
use std::path::PathBuf;
use uom::si::f64::Time;

mod dialog;
mod gui_widget;
mod message;
mod shared_canvas_functionality;
mod shared_widgets;
mod surface_view;
mod table_view;
mod top_view;

pub(crate) struct Gui {
    opened_file: Option<PathBuf>,
    mode: GuiViewMode,
    surface_view_state: SurfaceViewState,
    top_view_state: TopViewState,
    table_view_state: TableViewState,
    time_step: Time,
    celestial_system: Option<CelestialSystem>,
    selected_planet_name: String,
    display_names: bool,
    display_constellations: bool,
    pub(crate) dialog: Option<Box<dyn Dialog>>,
}

impl Gui {
    pub(super) fn redraw(&mut self) {
        match self.mode {
            GuiViewMode::Surface => {
                self.surface_view_state.redraw();
            }
            GuiViewMode::Top => {
                self.top_view_state.redraw();
            }
            _ => (),
        };
    }

    pub(super) fn get_selected_planet(&self) -> Option<Planet> {
        let system = self.celestial_system.as_ref()?;
        let data = self.get_selected_planet_data()?;
        let planet = Planet::new(
            (*data).clone(),
            system.get_central_body_data(),
            None,
            system.get_time_since_epoch(),
            None,
        );
        Some(planet)
    }

    pub(super) fn get_selected_planet_data(&self) -> Option<&PlanetData> {
        let planet_data = self
            .celestial_system
            .as_ref()?
            .get_planets_data()
            .iter()
            .find(|p| p.get_name().eq(&self.selected_planet_name))
            .copied();
        planet_data
    }

    pub(super) fn get_planet_data(&self) -> Vec<&PlanetData> {
        self.celestial_system
            .as_ref()
            .map(|s| s.get_planets_data())
            .unwrap_or_default()
    }
}
