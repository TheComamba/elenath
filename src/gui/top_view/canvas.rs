use astro_coords::{
    cartesian::Cartesian, direction::Direction, traits::*,
    transformations::rotations::get_rotation_parameters,
};
use astro_utils::{
    astro_display::AstroDisplay, color::srgb::sRGBColor, units::length::solar_radii,
};
use iced::{
    alignment::Horizontal,
    widget::canvas::{self, Path, Style},
    Color, Point, Rectangle, Renderer, Vector,
};
use uom::si::{
    f64::{Angle, Length},
    length::kilometer,
};

use crate::{
    gui::shared_canvas_functionality::{
        canvas_contains, display_info_text, draw_background, draw_name,
    },
    model::{celestial_system::CelestialSystem, planet::Planet},
};

use super::widget::TopViewState;

impl TopViewState {
    fn canvas_position(
        &self,
        pos: &Cartesian,
        view_angle: Angle,
        view_rotation_axis: &Direction,
    ) -> Vector {
        let rotated_position = pos.rotated(-view_angle, view_rotation_axis); //passive transformation
        let x = (rotated_position.x / self.length_per_pixel).value as f32;
        let y = (-rotated_position.y / self.length_per_pixel).value as f32; // y axis is inverted
        Vector::new(x, y)
    }

    pub(crate) fn canvas(
        &self,
        renderer: &Renderer,
        bounds: Rectangle,
        selected_planet: &Option<Planet>,
        celestial_system: &Option<CelestialSystem>,
        display_names: bool,
    ) -> Vec<canvas::Geometry> {
        let background = self
            .background_cache
            .draw(renderer, bounds.size(), |frame| {
                draw_background(bounds, frame);
            });

        let bodies = self.bodies_cache.draw(renderer, bounds.size(), |frame| {
            if let Some(celestial_system) = celestial_system {
                self.draw_bodies(
                    selected_planet,
                    celestial_system,
                    &bounds,
                    frame,
                    display_names,
                );
            } else {
                display_info_text(frame, "Please load or generate a celestial system.");
            }
        });

        let scale = self.scale_cache.draw(renderer, bounds.size(), |frame| {
            self.draw_scale(bounds, frame);
        });

        vec![background, bodies, scale]
    }

    fn draw_bodies(
        &self,
        selected_planet: &Option<Planet>,
        celestial_system: &CelestialSystem,
        bounds: &Rectangle,
        frame: &mut canvas::Frame,
        display_names: bool,
    ) {
        let view_direction = &self.view_ecliptic.spherical.to_direction();
        let (angle, view_rotation_axis) = get_rotation_parameters(&Direction::Z, view_direction);

        let offset = match selected_planet {
            Some(focus) => self.canvas_position(focus.get_position(), angle, &view_rotation_axis),
            None => Vector::new(0.0, 0.0),
        };

        let central_body_view = ViewParams {
            view_angle: angle,
            rotation_axis: &view_rotation_axis,
            offset,
            display_names,
        };
        self.draw_central_body(celestial_system, frame, bounds, &central_body_view);

        for planet in celestial_system.get_planets().iter() {
            let data = planet.get_data();
            let body = BodyParams {
                name: data.get_name(),
                pos3d: planet.get_position(),
                color: data.get_color(),
                albedo: Some(data.get_geometric_albedo()),
                radius: data.get_radius(),
            };
            let view = ViewParams {
                view_angle: angle,
                rotation_axis: &view_rotation_axis,
                offset,
                display_names,
            };
            self.draw_body(frame, bounds, &body, &view);
        }
    }

    fn draw_central_body(
        &self,
        celestial_system: &CelestialSystem,
        frame: &mut canvas::Frame,
        bounds: &Rectangle,
        view: &ViewParams,
    ) {
        let time = celestial_system.get_time_since_epoch();
        let data = celestial_system.get_central_body_data();
        let pos3d = Cartesian::origin();
        let color = sRGBColor::from_temperature(data.get_temperature(time));
        let radius = data
            .get_radius(time)
            .unwrap_or(Length::new::<solar_radii>(0.));
        let body = BodyParams {
            name: data.get_name(),
            pos3d: &pos3d,
            color: &color,
            albedo: None,
            radius,
        };

        self.draw_body(frame, bounds, &body, view);
    }

    fn draw_body(
        &self,
        frame: &mut canvas::Frame,
        bounds: &Rectangle,
        body: &BodyParams,
        view: &ViewParams,
    ) {
        let radius = canvas_radius(&body.radius);
        let pos = frame.center()
            + self.canvas_position(body.pos3d, view.view_angle, view.rotation_axis)
            - view.offset;
        if canvas_contains(bounds, pos) {
            let circle = Path::circle(pos, radius);
            let color = canvas_color(body.color, body.albedo);
            frame.fill(&circle, color);

            if view.display_names {
                draw_name(body.name, color, pos, frame);
            }
        }
    }

    fn draw_scale(&self, bounds: Rectangle, frame: &mut canvas::Frame) {
        const LENGTH_IN_PX: f32 = 200.0;
        let start_pos = Point::ORIGIN + Vector::new(50., bounds.height - 50.);
        let middle_pos = start_pos + Vector::new(LENGTH_IN_PX / 2., 0.0);
        let end_pos = start_pos + Vector::new(LENGTH_IN_PX, 0.0);
        let delimitor_vec = Vector::new(0.0, 5.);

        let scale = Path::new(|path_builder| {
            path_builder.move_to(start_pos + delimitor_vec);
            path_builder.line_to(start_pos - delimitor_vec);
            path_builder.move_to(start_pos);
            path_builder.line_to(end_pos);
            path_builder.move_to(end_pos + delimitor_vec);
            path_builder.line_to(end_pos - delimitor_vec);
        });
        let stroke = canvas::Stroke {
            style: Style::Solid(Color::WHITE),
            ..Default::default()
        };
        frame.stroke(&scale, stroke);

        let text = canvas::Text {
            color: Color::WHITE,
            content: (LENGTH_IN_PX as f64 * self.length_per_pixel).astro_display(),
            position: middle_pos,
            horizontal_alignment: Horizontal::Center,
            ..Default::default()
        };
        frame.fill_text(text);
    }
}

fn canvas_radius(radius: &Length) -> f32 {
    const SIZE_NUMBER: f32 = 0.3;
    (radius.get::<kilometer>() as f32).powf(SIZE_NUMBER) * SIZE_NUMBER
}

fn canvas_color(color: &sRGBColor, albedo: Option<f64>) -> Color {
    let (r, g, b) = color.maximized_sRGB_tuple();
    let a = albedo.unwrap_or(1.) as f32;
    Color {
        r: r as f32,
        g: g as f32,
        b: b as f32,
        a,
    }
}

struct BodyParams<'a> {
    name: &'a str,
    pos3d: &'a Cartesian,
    color: &'a sRGBColor,
    albedo: Option<f64>,
    radius: Length,
}

struct ViewParams<'a> {
    view_angle: Angle,
    rotation_axis: &'a Direction,
    offset: Vector,
    display_names: bool,
}
