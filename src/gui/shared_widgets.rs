use super::{
    dialog::DialogType,
    gui_widget::{BIG_COLUMN_WIDTH, PADDING, SMALL_COLUMN_WIDTH},
    message::GuiMessage,
    Gui, GuiViewMode,
};
use astro_utils::{astro_display::AstroDisplay, planets::planet_data::PlanetData};
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{text::Shaping, Button, Column, Container, PickList, Row, Text, TextInput, Toggler},
    Alignment, Element, Length,
};
use uom::si::f64::Time;

impl Gui {
    pub(super) fn gui_mode_tabs() -> Element<'static, GuiMessage> {
        let local_view_button = std_button(
            "Local View",
            GuiMessage::ModeSelected(GuiViewMode::Surface),
            true,
        );
        let top_view_button =
            std_button("Top View", GuiMessage::ModeSelected(GuiViewMode::Top), true);
        let table_view_button = std_button(
            "Table View",
            GuiMessage::ModeSelected(GuiViewMode::Table),
            true,
        );
        Row::new()
            .push(local_view_button)
            .push(top_view_button)
            .push(table_view_button)
            .align_y(Alignment::Center)
            .spacing(PADDING)
            .into()
    }

    pub(super) fn file_buttons(has_system: bool) -> Element<'static, GuiMessage> {
        let new_button = std_button(
            "New system",
            GuiMessage::OpenDialog(DialogType::NewSystem),
            true,
        );
        let save_to_file_button = std_button("Save to file", GuiMessage::SaveToFile, has_system);
        let save_to_new_file_button =
            std_button("Save to new file", GuiMessage::SaveToNewFile, has_system);
        let open_file_button = std_button("Open file", GuiMessage::OpenFile, true);

        Row::new()
            .push(new_button)
            .push(save_to_file_button)
            .push(save_to_new_file_button)
            .push(open_file_button)
            .align_y(Alignment::Center)
            .spacing(PADDING)
            .into()
    }
}

pub(crate) fn std_button(
    text: &str,
    message: GuiMessage,
    is_enabled: bool,
) -> Button<'_, GuiMessage> {
    let mut button = Button::new(
        Text::new(text)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center),
    );
    if is_enabled {
        button = button.on_press(message);
    }
    button.width(SMALL_COLUMN_WIDTH)
}

pub(super) fn planet_picker<'a>(
    planets: Vec<&PlanetData>,
    selected_planet: Option<&PlanetData>,
) -> Element<'a, GuiMessage> {
    let text = Text::new("Focused body:")
        .width(SMALL_COLUMN_WIDTH)
        .align_x(Horizontal::Right)
        .align_y(Vertical::Center);
    let mut planet_names = vec![String::new()];
    for name in planets.iter().map(|p| p.get_name()) {
        planet_names.push(name.clone());
    }
    let selected_planet_name = match selected_planet {
        Some(planet) => planet.get_name().clone(),
        None => String::new(),
    };
    let pick_list = PickList::new(
        planet_names,
        Some(selected_planet_name),
        GuiMessage::PlanetSelected,
    )
    .width(1.25 * SMALL_COLUMN_WIDTH + PADDING);
    Row::new()
        .push(text)
        .push(pick_list)
        .spacing(PADDING)
        .align_y(Alignment::Center)
        .into()
}

pub(super) fn surface_and_top_view_shared_control<'a>(
    time_since_epoch: Time,
    time_step: Time,
    planets: Vec<&PlanetData>,
    selected_planet: Option<&PlanetData>,
    display_names: bool,
    display_constellations: bool,
) -> Element<'a, GuiMessage> {
    let time_control_field = control_field(
        "Time since Epoch:",
        time_since_epoch.astro_display(),
        GuiMessage::UpdateTime(time_since_epoch - time_step),
        GuiMessage::UpdateTime(time_since_epoch + time_step),
    );

    let time_step_control_field = control_field(
        "Time step:",
        time_step.astro_display(),
        GuiMessage::UpdateTimeStep(time_step / 2.),
        GuiMessage::UpdateTimeStep(time_step * 2.),
    );

    let planet_picker = planet_picker(planets, selected_planet);

    let display_names_toggle = Container::new(
        Toggler::new(display_names)
            .label("Display Names")
            .on_toggle(GuiMessage::SetDisplayNames),
    )
    .width(Length::Fixed(1.5 * SMALL_COLUMN_WIDTH));

    let diplay_constellations_toggle = Container::new(
        Toggler::new(display_constellations)
            .label("Display Constellations")
            .on_toggle(GuiMessage::SetDisplayConstellations),
    )
    .width(Length::Fixed(1.5 * SMALL_COLUMN_WIDTH));

    Column::new()
        .push(time_control_field)
        .push(time_step_control_field)
        .push(planet_picker)
        .push(display_names_toggle)
        .push(diplay_constellations_toggle)
        .width(Length::Fixed(BIG_COLUMN_WIDTH))
        .align_x(Alignment::Center)
        .spacing(PADDING)
        .into()
}

pub(crate) fn control_field<M>(
    label: &str,
    value: String,
    decrease: M,
    increase: M,
) -> Row<'_, GuiMessage>
where
    M: Into<GuiMessage>,
{
    let label = Text::new(label)
        .align_y(Vertical::Center)
        .align_x(Horizontal::Right)
        .width(Length::Fixed(SMALL_COLUMN_WIDTH));
    let decrease_button = Container::new(Button::new(Text::new("<<")).on_press(decrease.into()))
        .align_x(Horizontal::Center)
        .width(Length::Fixed(0.25 * SMALL_COLUMN_WIDTH));
    let value = Text::new(value)
        .shaping(Shaping::Advanced)
        .width(Length::Fixed(0.75 * SMALL_COLUMN_WIDTH))
        .align_x(Horizontal::Center);
    let increase_button = Container::new(Button::new(Text::new(">>")).on_press(increase.into()))
        .align_x(Horizontal::Center)
        .width(Length::Fixed(0.25 * SMALL_COLUMN_WIDTH));
    Row::new()
        .push(label)
        .push(decrease_button)
        .push(value)
        .push(increase_button)
        .spacing(PADDING)
        .align_y(Alignment::Center)
}

pub(crate) fn edit<'a, Fun, Mes, Val>(
    description: &'static str,
    data: &str,
    units: &'static str,
    message: Fun,
    actual_value: &Option<Val>,
) -> Element<'a, Mes>
where
    Fun: 'a + Fn(String) -> Mes,
    Mes: 'a + Clone,
    Val: 'a + AstroDisplay,
{
    let description = if description.ends_with(':') {
        description.to_string()
    } else {
        description.to_string() + ":"
    };
    let description = Text::new(description)
        .width(SMALL_COLUMN_WIDTH)
        .align_x(Horizontal::Right);
    let data = TextInput::new("", data)
        .on_input(message)
        .width(SMALL_COLUMN_WIDTH);
    let units = Text::new(units)
        .shaping(Shaping::Advanced)
        .width(SMALL_COLUMN_WIDTH);
    let parsed_text = match actual_value {
        Some(value) => "Parsed value:\n".to_string() + &value.astro_display(),
        None => "Parsed value:\nNone".to_string(),
    };
    let value = Text::new(parsed_text)
        .shaping(Shaping::Advanced)
        .width(SMALL_COLUMN_WIDTH);
    Row::new()
        .push(description)
        .push(data)
        .push(units)
        .push(value)
        .spacing(PADDING)
        .into()
}
