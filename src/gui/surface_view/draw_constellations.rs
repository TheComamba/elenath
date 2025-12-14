use super::{viewport::Viewport, widget::SurfaceViewState};
use crate::{
    gui::{
        shared_canvas_functionality::canvas_contains,
        surface_view::canvas_appearance::CanvasAppearance,
    },
    model::celestial_system::CelestialSystem,
};
use astro_utils::stars::constellation::Constellation;
use iced::{
    alignment,
    widget::{
        canvas::{Frame, Path, Stroke, Style, Text},
        text::Alignment,
    },
    Color, Pixels, Rectangle, Vector,
};

impl SurfaceViewState {
    pub(super) fn draw_constellations(
        &self,
        frame: &mut Frame,
        bounds: Rectangle,
        celestial_system: &CelestialSystem,
        viewport: &Viewport,
    ) {
        for constellation in celestial_system.get_constellations() {
            self.draw_constellation(frame, bounds, constellation, viewport);
        }
    }

    fn draw_constellation(
        &self,
        frame: &mut Frame,
        bounds: Rectangle,
        constellation: &Constellation,
        viewport: &Viewport,
    ) {
        let appearances = constellation
            .get_stars()
            .iter()
            .map(|s| CanvasAppearance::from_star_appearance(s, viewport))
            .collect::<Vec<_>>();

        let color = Color {
            r: 1.,
            g: 1.,
            b: 1.,
            a: 0.5,
        };

        for connection in constellation.get_connections() {
            let (i, j) = connection.get_indices();
            if let (Some(star_i), Some(star_j)) = (&appearances[i], &appearances[j]) {
                let p_i = frame.center() + star_i.center_offset;
                let p_j = frame.center() + star_j.center_offset;
                let stroke = Stroke {
                    style: Style::Solid(Color::WHITE),
                    ..Default::default()
                };
                frame.stroke(&Path::line(p_i, p_j), stroke);
            }
        }

        let center = weighted_average_position(&appearances);
        let position = frame.center() + center;
        if canvas_contains(&bounds, position) {
            let name_widget = Text {
                content: constellation.get_name().to_string(),
                position,
                color,
                size: Pixels(20.),
                align_x: Alignment::Center,
                align_y: alignment::Vertical::Center,
                ..Default::default()
            };
            frame.fill_text(name_widget);
        }
    }
}

fn weighted_average_position(stars: &[Option<CanvasAppearance>]) -> Vector {
    let mut sum = Vector::new(0., 0.);
    let mut total_weight = 0.;
    for star in stars.iter().flatten() {
        let weight = star.radius.powi(2) * star.color.a;
        sum = sum + star.center_offset * weight;
        total_weight += weight;
    }
    sum * (1. / total_weight)
}
