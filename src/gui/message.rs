use super::dialog::load_real_planets::LoadRealPlanetsDialog;
use super::dialog::load_real_stars::LoadRealStarsDialog;
use super::dialog::planet::PlanetDialog;
use super::dialog::randomize_planets::RandomizePlanetsDialog;
use super::dialog::randomize_stars::RandomizeStarsDialog;
use super::dialog::star::StarDialog;
use super::dialog::{DialogType, DialogUpdate};
use super::gui_widget::GuiViewMode;
use super::table_view::col_data::TableDataType;
use super::Gui;
use super::{
    dialog::new_system::NewSystemDialog, surface_view::widget::SurfaceViewUpdate,
    top_view::widget::TopViewUpdate,
};
use crate::error::ElenathError;
use crate::model::star::StarDataType;
use crate::{file_dialog, model::celestial_system::CelestialSystem};
use astro_utils::planets::derived_data::DerivedPlanetData;
use astro_utils::planets::planet_data::PlanetData;
use astro_utils::stars::data::StarData;
use uom::si::f64::{Length, Time};

#[derive(Debug, Clone)]
pub(crate) enum GuiMessage {
    UpdateSurfaceView(SurfaceViewUpdate),
    UpdateTopView(TopViewUpdate),
    NewSystem,
    SaveToFile,
    SaveToNewFile,
    OpenFile,
    ModeSelected(GuiViewMode),
    NewPlanet(PlanetData),
    PlanetEdited(usize, PlanetData),
    NewStar(StarData),
    StarEdited(Option<usize>, StarData),
    UpdateTime(Time),
    UpdateTimeStep(Time),
    PlanetSelected(String),
    SetDisplayNames(bool),
    SetDisplayConstellations(bool),
    TableDataTypeSelected(TableDataType),
    RandomizePlanets,
    LoadRealPlanets,
    RandomizeStars(bool, Length),
    LoadStars(StarDataType),
    OpenDialog(DialogType),
    DialogUpdate(DialogUpdate),
    DialogSubmit,
    DialogClosed,
    ErrorEncountered(ElenathError),
}

impl Gui {
    fn open_dialog(&mut self, dialog_type: DialogType) -> Result<(), ElenathError> {
        match dialog_type {
            DialogType::NewSystem => {
                self.dialog = Some(Box::new(NewSystemDialog::new()));
            }
            DialogType::NewPlanet => {
                let celestial_system = &self.get_system()?;
                let central_body = celestial_system.get_central_body_data().clone();
                self.dialog = Some(Box::new(PlanetDialog::new(central_body)?));
            }
            DialogType::EditPlanet(index) => {
                let celestial_system = &self.get_system()?;
                let central_body = celestial_system.get_central_body_data();
                let planet = celestial_system
                    .get_planet_data(index)
                    .ok_or(ElenathError::BodyNotFound)?;
                let previous_planet = celestial_system.get_planet_data(index - 1);
                let previous_planet = match previous_planet {
                    Some(p) => Some(DerivedPlanetData::new(p, central_body, None)?),
                    None => None,
                };
                self.dialog = Some(Box::new(PlanetDialog::edit(
                    planet.clone(),
                    index,
                    previous_planet,
                    central_body.clone(),
                )?));
            }
            DialogType::NewStar => {
                let system = self.get_system()?;
                self.dialog = Some(Box::new(StarDialog::new(system.get_time_since_epoch())));
            }
            DialogType::EditStar(index) => {
                let system = &self.get_system()?;
                let star = system
                    .get_star_data(index)
                    .ok_or(ElenathError::BodyNotFound)?;
                self.dialog = Some(Box::new(StarDialog::edit(
                    star.clone(),
                    index,
                    system.get_time_since_epoch(),
                )));
            }
            DialogType::RandomizePlanets => {
                self.dialog = Some(Box::new(RandomizePlanetsDialog::new()));
            }
            DialogType::LoadRealPlanets => {
                self.dialog = Some(Box::new(LoadRealPlanetsDialog::new()));
            }
            DialogType::RandomizeStars => {
                self.dialog = Some(Box::new(RandomizeStarsDialog::new()));
            }
            DialogType::LoadGaiaData => {
                self.dialog = Some(Box::new(LoadRealStarsDialog::new()));
            }
        }
        Ok(())
    }

    pub(crate) fn handle_message(&mut self, message: GuiMessage) -> Result<(), ElenathError> {
        if let Some(dialog) = &mut self.dialog {
            if let Some(e) = dialog.get_error() {
                return Err(e);
            }
        }
        match message {
            GuiMessage::UpdateSurfaceView(message) => {
                self.surface_view_state.update(message);
            }
            GuiMessage::UpdateTopView(message) => {
                self.top_view_state.update(message);
            }
            GuiMessage::NewPlanet(planet) => {
                self.get_system()?.add_planet_data(planet);
                self.dialog = None;
            }
            GuiMessage::PlanetEdited(index, planet_data) => {
                self.get_system()?.overwrite_planet_data(index, planet_data);
                self.dialog = None;
            }
            GuiMessage::NewStar(star) => {
                self.get_system()?.add_stars_from_data(vec![star]);
                self.dialog = None;
            }
            GuiMessage::StarEdited(index, star_data) => {
                self.get_system()?.overwrite_star_data(index, star_data);
                self.dialog = None;
            }
            GuiMessage::NewSystem => {
                self.celestial_system = Some(CelestialSystem::empty());
                self.dialog = None;
            }
            GuiMessage::SaveToFile => {
                if self.opened_file.is_none() {
                    self.opened_file = file_dialog::new();
                }
                if let Some(path) = &self.opened_file {
                    self.get_system_const()?.write_to_file(path.clone())?;
                }
            }
            GuiMessage::SaveToNewFile => {
                self.opened_file = file_dialog::new();
                if let Some(path) = &self.opened_file {
                    self.get_system_const()?.write_to_file(path.clone())?;
                }
            }
            GuiMessage::OpenFile => {
                self.opened_file = file_dialog::open();
                if let Some(path) = &self.opened_file {
                    self.celestial_system = Some(CelestialSystem::read_from_file(path.clone())?);
                }
            }
            GuiMessage::ModeSelected(mode) => {
                self.mode = mode;
            }
            GuiMessage::UpdateTime(time) => {
                self.get_system()?.set_time_since_epoch(time);
            }
            GuiMessage::UpdateTimeStep(time_step) => {
                self.time_step = time_step;
            }
            GuiMessage::PlanetSelected(name) => {
                self.selected_planet_name = name;
            }
            GuiMessage::SetDisplayNames(display_names) => {
                self.display_names = display_names;
            }
            GuiMessage::SetDisplayConstellations(display_constellations) => {
                self.display_constellations = display_constellations;
            }
            GuiMessage::TableDataTypeSelected(body_type) => {
                self.table_view_state.displayed_body_type = body_type;
            }
            GuiMessage::RandomizePlanets => {
                self.get_system()?.randomize_planets();
                self.dialog = None;
            }
            GuiMessage::LoadRealPlanets => {
                self.get_system()?.load_real_planets();
                self.dialog = None;
            }
            GuiMessage::RandomizeStars(keep_central_body, max_distance) => {
                self.get_system()?
                    .randomize_stars(keep_central_body, max_distance)?;
                self.dialog = None;
            }
            GuiMessage::LoadStars(data_type) => {
                self.get_system()?.load_real_stars(data_type)?;
                self.dialog = None;
            }
            GuiMessage::OpenDialog(dialog_type) => {
                self.open_dialog(dialog_type)?;
            }
            GuiMessage::DialogClosed => {
                self.dialog = None;
            }
            GuiMessage::ErrorEncountered(error) => {
                return Err(error);
            }
            GuiMessage::DialogUpdate(update) => {
                if let Some(dialog) = &mut self.dialog {
                    dialog.update(update);
                }
            }
            GuiMessage::DialogSubmit => {
                if let Some(dialog) = &self.dialog {
                    self.handle_message(dialog.on_submit());
                }
            }
        }
        self.redraw();
        Ok(())
    }

    fn get_system(&mut self) -> Result<&mut CelestialSystem, ElenathError> {
        self.celestial_system
            .as_mut()
            .ok_or(ElenathError::NoCelestialSystem)
    }

    fn get_system_const(&self) -> Result<&CelestialSystem, ElenathError> {
        self.celestial_system
            .as_ref()
            .ok_or(ElenathError::NoCelestialSystem)
    }
}
