use super::{Dialog, DialogUpdate};
use crate::gui::{
    gui_widget::{PADDING, SMALL_COLUMN_WIDTH},
    message::GuiMessage,
};
use astro_utils::astro_display::AstroDisplay;
use iced::{
    self,
    widget::{Button, Column, Radio, Row, Text, Toggler},
    Alignment, Element,
};
use uom::si::{f64::Length, length::light_year};

#[derive(Debug, Clone)]
pub(crate) struct RandomizeStarsDialog {
    keep_central_body: bool,
    generation_distance: GenerationDistance,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub(crate) enum GenerationDistance {
    Decent,
    Realistic,
    VeryFar,
}

impl RandomizeStarsDialog {
    pub(crate) fn new() -> Self {
        RandomizeStarsDialog {
            keep_central_body: true,
            generation_distance: GenerationDistance::Decent,
        }
    }
}

fn max_generation_distance(distance: GenerationDistance) -> Length {
    match distance {
        GenerationDistance::Decent => Length::new::<light_year>(1000.0),
        GenerationDistance::Realistic => Length::new::<light_year>(5000.0),
        GenerationDistance::VeryFar => Length::new::<light_year>(25_000.0),
    }
}

fn message<F: Fn(GenerationDistance) -> RandomizeStarsDialogEvent>(
    event: F,
) -> impl Fn(GenerationDistance) -> GuiMessage {
    move |m| GuiMessage::DialogUpdate(DialogUpdate::RandmoizeStarsUpdated(event(m)))
}

impl Dialog for RandomizeStarsDialog {
    fn header(&self) -> String {
        "Generate Random Stars".to_string()
    }

    fn body<'a>(&'a self) -> Element<'a, GuiMessage> {
        let warning = Text::new("This will overwrite all stars in the current system.");

        let keep_central_body_toggler = Toggler::new(self.keep_central_body)
            .label("Keep Central Body")
            .on_toggle(|b| {
                GuiMessage::DialogUpdate(DialogUpdate::RandmoizeStarsUpdated(
                    RandomizeStarsDialogEvent::KeepCentralBodySelected(b),
                ))
            })
            .width(2. * SMALL_COLUMN_WIDTH);

        let decent_distance_radio = Radio::new(
            format!(
                "Decent\n{}",
                max_generation_distance(GenerationDistance::Decent).astro_display()
            ),
            GenerationDistance::Decent,
            Some(self.generation_distance),
            message(RandomizeStarsDialogEvent::MaxGenerationDistanceChanged),
        )
        .width(SMALL_COLUMN_WIDTH);
        let realistic_distance_radio = Radio::new(
            format!(
                "Realistic\n{}",
                max_generation_distance(GenerationDistance::Realistic).astro_display()
            ),
            GenerationDistance::Realistic,
            Some(self.generation_distance),
            message(RandomizeStarsDialogEvent::MaxGenerationDistanceChanged),
        )
        .width(SMALL_COLUMN_WIDTH);
        let very_far_distance_radio = Radio::new(
            format!(
                "Very Far\n{}",
                max_generation_distance(GenerationDistance::VeryFar).astro_display()
            ),
            GenerationDistance::VeryFar,
            Some(self.generation_distance),
            message(RandomizeStarsDialogEvent::MaxGenerationDistanceChanged),
        )
        .width(SMALL_COLUMN_WIDTH);
        let generation_distance_row = Row::new()
            .push(decent_distance_radio)
            .push(realistic_distance_radio)
            .push(very_far_distance_radio)
            .padding(PADDING)
            .spacing(PADDING);
        let submit_button = Button::new(Text::new("Submit")).on_press(GuiMessage::DialogSubmit);

        Column::new()
            .push(warning)
            .push(keep_central_body_toggler)
            .push(Text::new("Maximum Generation Distance"))
            .push(generation_distance_row)
            .push(submit_button)
            .padding(PADDING)
            .spacing(PADDING)
            .width(iced::Length::Fill)
            .align_x(Alignment::Center)
            .into()
    }

    fn update(&mut self, message: super::DialogUpdate) {
        if let DialogUpdate::RandmoizeStarsUpdated(event) = message {
            match event {
                RandomizeStarsDialogEvent::KeepCentralBodySelected(keep_central_body) => {
                    self.keep_central_body = keep_central_body;
                }
                RandomizeStarsDialogEvent::MaxGenerationDistanceChanged(generation_distance) => {
                    self.generation_distance = generation_distance;
                }
            }
        }
    }

    fn on_submit(&self) -> GuiMessage {
        let max_distance = max_generation_distance(self.generation_distance);
        GuiMessage::RandomizeStars(self.keep_central_body, max_distance)
    }

    fn get_error(&self) -> Option<super::ElenathError> {
        None
    }
}

#[derive(Debug, Clone)]
pub(crate) enum RandomizeStarsDialogEvent {
    KeepCentralBodySelected(bool),
    MaxGenerationDistanceChanged(GenerationDistance),
}
