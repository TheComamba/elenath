use super::{
    dialog::error::ErrorDialog, message::GuiMessage,
    shared_widgets::surface_and_top_view_shared_control, surface_view::widget::SurfaceViewState,
    table_view::widget::TableViewState, top_view::widget::TopViewState, Gui,
};
use iced::{
    mouse::Cursor,
    widget::{canvas, opaque, stack, Column, Container, Row, Text},
    Element, Length, Rectangle, Renderer, Theme,
};
use uom::si::f64::Time;

pub(super) const PADDING: f32 = 10.0;
pub(super) const SMALL_COLUMN_WIDTH: f32 = 150.0;
pub(super) const BIG_COLUMN_WIDTH: f32 = 3.5 * SMALL_COLUMN_WIDTH;

#[derive(Debug, Clone)]
pub(crate) enum GuiViewMode {
    Surface,
    Top,
    Table,
}

impl Default for Gui {
    fn default() -> Self {
        Gui {
            opened_file: None,
            mode: GuiViewMode::Surface,
            surface_view_state: SurfaceViewState::new(),
            top_view_state: TopViewState::new(),
            table_view_state: TableViewState::new(),
            time_step: Time::from_days(1.0),
            celestial_system: None,
            selected_planet_name: String::new(),
            display_names: true,
            display_constellations: false,
            dialog: None,
        }
    }
}

impl Gui {
    fn title(&self) -> String {
        String::from("Elenath - Imaginary Skies")
    }

    pub(crate) fn update(&mut self, message: GuiMessage) {
        if let Err(e) = self.handle_message(message) {
            self.dialog = Some(Box::new(ErrorDialog::new(e)));
        }
    }

    pub(crate) fn view(&self) -> Element<'_, GuiMessage> {
        if let Some(dialog) = self.dialog.as_ref() {
            stack!(self.main_view(), opaque(dialog.to_element())).into()
        } else {
            self.main_view()
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

impl<GuiMessage> canvas::Program<GuiMessage> for Gui {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<canvas::Geometry> {
        match self.mode {
            GuiViewMode::Surface => self.surface_view_state.canvas(
                renderer,
                bounds,
                &self.get_selected_planet(),
                &self.celestial_system,
                self.display_names,
                self.display_constellations,
            ),
            GuiViewMode::Top => self.top_view_state.canvas(
                renderer,
                bounds,
                &self.get_selected_planet(),
                &self.celestial_system,
                self.display_names,
            ),
            _ => {
                println!("Invalid Gui state: Canvas Program is called from a Gui mode that does not have a canvas.");
                vec![]
            }
        }
    }
}

impl Gui {
    fn main_view(&self) -> Element<'_, GuiMessage> {
        let toprow = Row::new()
            .push(Gui::gui_mode_tabs())
            .push(Container::new(Text::new("")).width(Length::Fill))
            .push(Gui::file_buttons(self.celestial_system.is_some()))
            .padding(PADDING)
            .spacing(PADDING);
        let mut col = Column::new().push(toprow);

        if let Some(system) = self.celestial_system.as_ref() {
            match self.mode {
                GuiViewMode::Surface => {
                    let control_row = Row::new()
                        .push(surface_and_top_view_shared_control(
                            system.get_time_since_epoch(),
                            self.time_step,
                            self.get_planet_data(),
                            self.get_selected_planet_data(),
                            self.display_names,
                            self.display_constellations,
                        ))
                        .push(self.surface_view_state.control_field());
                    col = col
                        .push(control_row)
                        .push(canvas(self).width(Length::Fill).height(Length::Fill))
                }
                GuiViewMode::Top => {
                    let control_row = Row::new()
                        .push(surface_and_top_view_shared_control(
                            system.get_time_since_epoch(),
                            self.time_step,
                            self.get_planet_data(),
                            self.get_selected_planet_data(),
                            self.display_names,
                            self.display_constellations,
                        ))
                        .push(self.top_view_state.control_field());
                    col = col
                        .push(control_row)
                        .push(canvas(self).width(Length::Fill).height(Length::Fill))
                }
                GuiViewMode::Table => {
                    col = col.push(self.table_view_state.table_view(&self.celestial_system));
                }
            }
        }

        col.width(Length::Fill)
            .height(Length::Fill)
            .spacing(PADDING)
            .into()
    }
}
