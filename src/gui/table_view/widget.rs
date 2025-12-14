use super::col_data::{TableColData, TableDataType};
use crate::{
    gui::{
        dialog::DialogType, gui_widget::PADDING, message::GuiMessage, shared_widgets::std_button,
    },
    model::celestial_system::{
        part::{BodyType, PartOfCelestialSystem},
        CelestialSystem,
    },
};
use iced::{
    widget::{
        rule,
        scrollable::{Direction, Scrollbar},
        text::Shaping,
        Button, Column, Container, Row, Scrollable, Text,
    },
    Alignment, Element, Length,
};

const CELL_WIDTH: f32 = 150.;
const BUTTON_CELL_WIDTH: f32 = 50.;
const MAX_ROWS: usize = 250;

pub(crate) struct TableViewState {
    pub(crate) displayed_body_type: TableDataType,
}

impl TableViewState {
    pub(crate) fn new() -> TableViewState {
        TableViewState {
            displayed_body_type: TableDataType::Planet,
        }
    }

    pub(crate) fn table_view(&self, system: &Option<CelestialSystem>) -> Element<'_, GuiMessage> {
        let buttons = Row::new()
            .push(data_type_selection_tabs())
            .push(Container::new(Text::new("")).width(Length::Fill))
            .push(self.generation_buttons());

        let mut col = Column::new().push(buttons);

        if let Some(system) = system {
            let table = match self.displayed_body_type {
                TableDataType::Planet => {
                    let planet_col_data = TableColData::default_planet_col_data();
                    let planets = system.get_planets();
                    table(
                        planet_col_data,
                        planets,
                        GuiMessage::OpenDialog(DialogType::NewPlanet),
                    )
                }
                TableDataType::Star => {
                    let star_col_data = TableColData::default_star_col_data();
                    let stars = system.get_stars();
                    table(
                        star_col_data,
                        stars,
                        GuiMessage::OpenDialog(DialogType::NewStar),
                    )
                }
                TableDataType::Supernova => {
                    let supernova_col_data = TableColData::default_supernova_col_data();
                    let supernovae = system.get_supernovae();
                    table(
                        supernova_col_data,
                        supernovae,
                        GuiMessage::OpenDialog(DialogType::NewStar),
                    )
                }
            };
            col = col.push(table);
        }

        col.width(Length::Fill).height(Length::Fill).into()
    }

    fn generation_buttons(&self) -> Element<'static, GuiMessage> {
        let mut row = Row::new();
        match self.displayed_body_type {
            TableDataType::Planet => {
                let randomize_planets = std_button(
                    "Randomize Planets",
                    GuiMessage::OpenDialog(DialogType::RandomizePlanets),
                    true,
                );
                let load_real_planets = std_button(
                    "Load Real Planets",
                    GuiMessage::OpenDialog(DialogType::LoadRealPlanets),
                    true,
                );
                row = row.push(randomize_planets).push(load_real_planets);
            }
            TableDataType::Star => {
                let randomize_stars = std_button(
                    "Randomize Stars",
                    GuiMessage::OpenDialog(DialogType::RandomizeStars),
                    true,
                );
                let load_real_stars = std_button(
                    "Load Real Stars",
                    GuiMessage::OpenDialog(DialogType::LoadGaiaData),
                    true,
                );
                row = row.push(randomize_stars).push(load_real_stars);
            }
            TableDataType::Supernova => {}
        }

        row.align_y(Alignment::Center)
            .spacing(PADDING)
            .padding(PADDING)
            .into()
    }
}

fn table<T>(
    col_data: Vec<TableColData<T>>,
    bodies: Vec<T>,
    new_message: GuiMessage,
) -> Scrollable<'static, GuiMessage>
where
    T: PartOfCelestialSystem,
{
    let width = table_width(&col_data);
    let scrollbar = Scrollbar::new();
    Scrollable::new(
        Column::new()
            .push(table_header(new_message, &col_data))
            .push(Container::new(rule::horizontal(10)).width(width))
            .push(table_contents(bodies, col_data)),
    )
    .direction(Direction::Horizontal(scrollbar))
    .width(Length::Fill)
    .height(Length::Fill)
}

fn table_width<T>(table_col_data: &[TableColData<T>]) -> Length {
    Length::Fixed(table_col_data.len() as f32 * CELL_WIDTH + 2. * BUTTON_CELL_WIDTH)
}

fn data_type_selection_tabs() -> Element<'static, GuiMessage> {
    let planet_button = std_button(
        "Planets",
        GuiMessage::TableDataTypeSelected(TableDataType::Planet),
        true,
    );
    let star_button = std_button(
        "Stars",
        GuiMessage::TableDataTypeSelected(TableDataType::Star),
        true,
    );
    let supernova_button = std_button(
        "Supernovae",
        GuiMessage::TableDataTypeSelected(TableDataType::Supernova),
        true,
    );
    Row::new()
        .push(planet_button)
        .push(star_button)
        .push(supernova_button)
        .align_y(Alignment::Center)
        .spacing(PADDING)
        .padding(PADDING)
        .into()
}

fn table_contents<T>(
    bodies: Vec<T>,
    table_col_data: Vec<TableColData<T>>,
) -> Element<'static, GuiMessage>
where
    T: PartOfCelestialSystem,
{
    let mut col = Column::new();
    let length = bodies.len();
    for (sorting_index, body) in bodies.into_iter().enumerate().take(MAX_ROWS) {
        col = col.push(table_row(sorting_index, body, &table_col_data));
    }
    if length > MAX_ROWS {
        col = col.push(Text::new(format!("... and {} more", length - MAX_ROWS)));
    }
    let scrollbar = Scrollbar::new();
    Scrollable::new(col)
        .direction(Direction::Vertical(scrollbar))
        .height(Length::Fill)
        .into()
}

fn table_header<T>(
    new_dialog_message: GuiMessage,
    table_col_data: &Vec<TableColData<T>>,
) -> Row<'static, GuiMessage> {
    let new_button = Button::new("New").on_press(new_dialog_message);

    let mut row = Row::new()
        .push(Container::new(new_button).width(Length::Fixed(BUTTON_CELL_WIDTH)))
        .push(Container::new(Text::new("")).width(Length::Fixed(BUTTON_CELL_WIDTH)));
    for col in table_col_data {
        row = row.push(table_cell(Text::new(col.header).into()));
    }
    row.align_y(Alignment::Center)
}

fn table_row<T>(
    sorting_index: usize,
    data: T,
    table_col_data: &[TableColData<T>],
) -> Row<'static, GuiMessage>
where
    T: PartOfCelestialSystem,
{
    let mut edit_button = Button::new(Text::new("Edit"));
    let index = data.get_index();
    match data.get_body_type() {
        BodyType::Planet => {
            if let Some(index) = index {
                edit_button =
                    edit_button.on_press(GuiMessage::OpenDialog(DialogType::EditPlanet(index)));
            }
        }
        BodyType::Star => {
            edit_button = edit_button.on_press(GuiMessage::OpenDialog(DialogType::EditStar(
                data.get_index(),
            )));
        }
    }
    let mut row = Row::new()
        .push(Container::new(edit_button).width(Length::Fixed(BUTTON_CELL_WIDTH)))
        .push(
            Container::new(Text::new(format!("{}", sorting_index + 1)))
                .width(Length::Fixed(BUTTON_CELL_WIDTH)),
        );
    for col in table_col_data.iter() {
        let content = (col.content_closure)(&data).unwrap_or("N/A".to_string());
        let text = Text::new(content).shaping(Shaping::Advanced);
        row = row.push(table_cell(text.into()));
    }
    row.align_y(Alignment::Center)
}

fn table_cell(content: Element<'_, GuiMessage>) -> Container<'_, GuiMessage> {
    Container::new(content).width(Length::Fixed(CELL_WIDTH))
}
