use astro_coords::{
    cartesian::Cartesian, transformations::relative_direction::direction_relative_to_normal,
};
use astro_units::illuminance::{lux, Illuminance};
use astro_utils::{color::srgb::sRGBColor, stars::appearance::StarAppearance};
use iced::{Color, Vector};

use crate::model::{celestial_system::CelestialSystem, planet::Planet};

use super::viewport::Viewport;

pub(super) struct CanvasAppearance {
    pub(super) name: String,
    pub(super) center_offset: Vector,
    pub(super) radius: f32,
    pub(super) color: Color,
}

impl CanvasAppearance {
    pub(super) const MIN_RADIUS: f32 = 1.5;
    const MAX_RADIUS: f32 = 1e5;
    const RADIUS_EXPONENT: f32 = 0.23;
    const ALPHA_EXPONENT: f32 = 0.75;

    #[inline(always)]
    fn illuminance_at_min_radius() -> Illuminance {
        Illuminance::new::<lux>(8e-8)
    }

    pub(super) fn from_star_appearance(
        appearance: &StarAppearance,
        viewport: &Viewport,
    ) -> Option<CanvasAppearance> {
        let (color, radius) = Self::color_and_radius(appearance);
        Some(Self {
            name: appearance.get_name().to_string(),
            center_offset: offset(appearance, viewport)?,
            radius,
            color,
        })
    }

    pub(super) fn from_central_body(
        celestial_system: &CelestialSystem,
        viewport: &Viewport,
        observer_position: &Cartesian,
    ) -> Option<CanvasAppearance> {
        let central_body_appearance =
            celestial_system.get_central_body_appearance(observer_position);
        CanvasAppearance::from_star_appearance(&central_body_appearance, viewport)
    }

    pub(super) fn from_planet(
        celestial_system: &CelestialSystem,
        planet: &Planet,
        viewport: &Viewport,
        observer_position: &Cartesian,
    ) -> Option<CanvasAppearance> {
        let planet_appearance = planet.get_data().to_star_appearance(
            celestial_system.get_central_body_data(),
            planet.get_position(),
            observer_position,
            celestial_system.get_time_since_epoch(),
        );
        let planet_appearance = match planet_appearance {
            Ok(appearance) => appearance,
            Err(_) => {
                return None;
            }
        };

        CanvasAppearance::from_star_appearance(&planet_appearance, viewport)
    }

    fn color_and_radius(body: &StarAppearance) -> (Color, f32) {
        const WHITE: sRGBColor = sRGBColor::from_sRGB(1., 1., 1.);
        let color = body.get_color();
        let (r, g, b) = color.maximized_sRGB_tuple();
        let color = &sRGBColor::from_sRGB(r, g, b) + &WHITE;
        let (r, g, b) = color.maximized_sRGB_tuple();

        let illuminance = body.get_illuminance();
        let ratio = (illuminance / Self::illuminance_at_min_radius()).value as f32;
        if ratio < 1. {
            let radius = Self::MIN_RADIUS;
            let alpha = ratio.powf(Self::ALPHA_EXPONENT);
            let color = Color::from_rgba(r as f32, g as f32, b as f32, alpha);
            (color, radius)
        } else {
            let radius = ratio.powf(Self::RADIUS_EXPONENT) * Self::MIN_RADIUS;
            let color = Color::from_rgb(r as f32, g as f32, b as f32);
            if radius > Self::MAX_RADIUS {
                (color, Self::MAX_RADIUS)
            } else {
                (color, radius)
            }
        }
    }
}

fn offset(appearance: &StarAppearance, viewport: &Viewport) -> Option<Vector> {
    let direction = direction_relative_to_normal(
        &appearance.get_pos().to_direction(),
        &viewport.center_direction,
        &viewport.top_direction,
    );
    if direction.z() > 0.0 {
        let x = direction.y() as f32 * viewport.px_per_distance; // rotation_reference corresponds to the x axis while iced y corresponds to top.
        let y = -direction.x() as f32 * viewport.px_per_distance; // y axis is inverted
        Some(Vector::new(x as f32, y as f32))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use astro_coords::{direction::Direction, ecliptic::Ecliptic, traits::*};
    use astro_units::{
        illuminance::apparent_magnitude_to_illuminance, length::earth_radius, mass::earth_mass,
    };
    use astro_utils::{
        astro_display::AstroDisplay,
        color::srgb::sRGBColor,
        planets::{
            orbit_parameters::OrbitParameters, physical_parameters::PlanetPhysicalParameters,
            planet_data::PlanetData,
        },
        real_data::stars::sun,
    };
    use uom::si::{
        angle::{degree, radian},
        f64::{Angle, Length, Mass, Time},
        length::astronomical_unit,
        time::year,
    };

    use super::*;

    fn some_illuminance() -> Illuminance {
        Illuminance::new::<lux>(100.)
    }
    const SOME_COLOR: sRGBColor = sRGBColor::from_sRGB(0., 1., 0.);
    const SOME_FLOAT: f32 = 1.;

    fn vecs_equal(p1: Vector, p2: Vector) -> bool {
        (p1.x - p2.x).abs() < 1e-4 && (p1.y - p2.y).abs() < 1e-4
    }

    #[test]
    fn star_at_center() {
        let ordinates = vec![-1., 0., 1., 12.];
        for x in ordinates.clone().iter() {
            for y in ordinates.clone().iter() {
                for z in ordinates.clone().iter() {
                    let center_direction = Direction::new(*x, *y, *z);
                    if center_direction.is_err() {
                        continue;
                    }
                    let center_direction = center_direction.unwrap();
                    let top_direction = center_direction.some_orthogonal_vector();
                    let viewport = Viewport {
                        center_direction: center_direction.clone(),
                        top_direction,
                        px_per_distance: SOME_FLOAT,
                    };
                    let star_appearance = StarAppearance::new(
                        String::new(),
                        some_illuminance(),
                        SOME_COLOR,
                        center_direction.to_ecliptic(),
                        Time::new::<year>(0.),
                    );
                    let canvas_appearance =
                        CanvasAppearance::from_star_appearance(&star_appearance, &viewport)
                            .unwrap();
                    assert!(vecs_equal(
                        canvas_appearance.center_offset,
                        Vector { x: 0., y: 0. }
                    ));
                }
            }
        }
    }

    #[test]
    fn stars_at_boundaries() {
        let ordinates = vec![0., 1., 1., 12.];
        for x1 in ordinates.clone().iter() {
            for y1 in ordinates.clone().iter() {
                for z1 in ordinates.clone().iter() {
                    for x2 in ordinates.clone().iter() {
                        for y2 in ordinates.clone().iter() {
                            for z2 in ordinates.clone().iter() {
                                let center_direction = Direction::new(*x1, *y1, *z1);
                                let top_direction = Direction::new(*x2, *y2, *z2);
                                if center_direction.is_err() || top_direction.is_err() {
                                    continue;
                                }
                                let center = center_direction.unwrap();
                                let top = top_direction.unwrap();
                                if center.eq_within(&top, 1e-5) || center.eq_within(&(-&top), 1e-5)
                                {
                                    continue;
                                }
                                let left = top.rotated(Angle::new::<degree>(-90.), &center);
                                let bottom = left.rotated(Angle::new::<degree>(-90.), &center);
                                let right = bottom.rotated(Angle::new::<degree>(-90.), &center);

                                println!(
                                    "center: {}, top: {}, left: {}, bottom: {}, right: {}",
                                    center, top, left, bottom, right
                                );

                                let viewport = Viewport {
                                    center_direction: center.clone(),
                                    top_direction: top.clone(),
                                    px_per_distance: SOME_FLOAT,
                                };
                                let half_opening_angle = center.angle_to(&top);
                                if half_opening_angle.get::<degree>().abs() > 89. {
                                    continue;
                                }
                                let expected_offset = half_opening_angle.get::<radian>().sin()
                                    as f32
                                    * viewport.px_per_distance;

                                println!(
                                    "half opening angle: {}",
                                    half_opening_angle.astro_display()
                                );
                                println!("expected offset: {}", expected_offset);

                                let top = StarAppearance::new(
                                    String::new(),
                                    some_illuminance(),
                                    SOME_COLOR,
                                    top.to_ecliptic(),
                                    Time::new::<year>(0.),
                                );
                                let left = StarAppearance::new(
                                    String::new(),
                                    some_illuminance(),
                                    SOME_COLOR,
                                    left.to_ecliptic(),
                                    Time::new::<year>(0.),
                                );
                                let bottom = StarAppearance::new(
                                    String::new(),
                                    some_illuminance(),
                                    SOME_COLOR,
                                    bottom.to_ecliptic(),
                                    Time::new::<year>(0.),
                                );
                                let right = StarAppearance::new(
                                    String::new(),
                                    some_illuminance(),
                                    SOME_COLOR,
                                    right.to_ecliptic(),
                                    Time::new::<year>(0.),
                                );

                                let top = CanvasAppearance::from_star_appearance(&top, &viewport)
                                    .unwrap();
                                let left = CanvasAppearance::from_star_appearance(&left, &viewport)
                                    .unwrap();
                                let bottom =
                                    CanvasAppearance::from_star_appearance(&bottom, &viewport)
                                        .unwrap();
                                let right =
                                    CanvasAppearance::from_star_appearance(&right, &viewport)
                                        .unwrap();

                                println!(
                                    "top: {:?}, left: {:?}, bottom: {:?}, right: {:?}",
                                    top.center_offset,
                                    left.center_offset,
                                    bottom.center_offset,
                                    right.center_offset
                                );

                                assert!(vecs_equal(
                                    top.center_offset,
                                    Vector {
                                        x: 0.,
                                        y: -expected_offset
                                    }
                                ));
                                assert!(vecs_equal(
                                    left.center_offset,
                                    Vector {
                                        x: -expected_offset,
                                        y: 0.
                                    }
                                ));
                                assert!(vecs_equal(
                                    bottom.center_offset,
                                    Vector {
                                        x: 0.,
                                        y: expected_offset
                                    }
                                ));
                                assert!(vecs_equal(
                                    right.center_offset,
                                    Vector {
                                        x: expected_offset,
                                        y: 0.
                                    }
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn view_direction_z() {
        let viewport = Viewport {
            center_direction: Direction::Z,
            top_direction: Direction::Y,
            px_per_distance: SOME_FLOAT,
        };
        for x in [-0.1, 0.1] {
            for y in [-0.1, 0.1] {
                let star_direction = Direction::new(x, y, 1.).unwrap();
                println!("star direction: {}", star_direction);
                let star = StarAppearance::new(
                    "".to_string(),
                    some_illuminance(),
                    SOME_COLOR,
                    star_direction.to_ecliptic(),
                    Time::new::<year>(0.),
                );
                let appearance = CanvasAppearance::from_star_appearance(&star, &viewport);
                let center_offset = appearance.unwrap().center_offset;
                println!("center offset: {:?}", center_offset);
                if x > 0. {
                    assert!(center_offset.x < 0.);
                } else {
                    assert!(center_offset.x > 0.);
                }
                if y > 0. {
                    assert!(center_offset.y < 0.);
                } else {
                    assert!(center_offset.y > 0.);
                }
            }
        }
    }

    #[test]
    fn view_direction_x() {
        let viewport = Viewport {
            center_direction: Direction::X,
            top_direction: Direction::Z,
            px_per_distance: SOME_FLOAT,
        };
        for y in [-0.1, 0.1] {
            for z in [-0.1, 0.1] {
                let star_direction = Direction::new(1., y, z).unwrap();
                println!("star direction: {}", star_direction);
                let star = StarAppearance::new(
                    "".to_string(),
                    some_illuminance(),
                    SOME_COLOR,
                    star_direction.to_ecliptic(),
                    Time::new::<year>(0.),
                );
                let appearance = CanvasAppearance::from_star_appearance(&star, &viewport);
                let center_offset = appearance.unwrap().center_offset;
                println!("center offset: {:?}", center_offset);
                if y > 0. {
                    assert!(center_offset.x < 0.);
                } else {
                    assert!(center_offset.x > 0.);
                }
                if z > 0. {
                    assert!(center_offset.y < 0.);
                } else {
                    assert!(center_offset.y > 0.);
                }
            }
        }
    }

    #[test]
    fn apparent_magnitude_6p5_star_is_dim() {
        let star_appearance = StarAppearance::new(
            String::new(),
            apparent_magnitude_to_illuminance(6.5),
            SOME_COLOR,
            Ecliptic::x_direction(),
            Time::new::<year>(0.),
        );
        let viewport = Viewport {
            center_direction: Direction::X,
            top_direction: Direction::Y,
            px_per_distance: SOME_FLOAT,
        };
        let canvas_appearance =
            CanvasAppearance::from_star_appearance(&star_appearance, &viewport).unwrap();
        println!("radius: {}", canvas_appearance.radius);
        assert!(canvas_appearance.radius > 0.);
        assert!(canvas_appearance.color.a > 0.);
        assert!(canvas_appearance.color.a < 0.3);
    }

    #[test]
    fn apparent_magnitude_0_star_is_bright() {
        let star_appearance = StarAppearance::new(
            String::new(),
            apparent_magnitude_to_illuminance(0.),
            SOME_COLOR,
            Ecliptic::x_direction(),
            Time::new::<year>(0.),
        );
        let viewport = Viewport {
            center_direction: Direction::X,
            top_direction: Direction::Y,
            px_per_distance: SOME_FLOAT,
        };
        let canvas_appearance =
            CanvasAppearance::from_star_appearance(&star_appearance, &viewport).unwrap();
        println!("radius: {}", canvas_appearance.radius);
        assert!(canvas_appearance.radius > 1.);
        assert!(canvas_appearance.radius < 10.);
    }

    #[test]
    fn venus_is_not_too_big() {
        let star_appearance = StarAppearance::new(
            String::new(),
            apparent_magnitude_to_illuminance(-4.92),
            SOME_COLOR,
            Ecliptic::x_direction(),
            Time::new::<year>(0.),
        );
        let viewport = Viewport {
            center_direction: Direction::X,
            top_direction: Direction::Y,
            px_per_distance: SOME_FLOAT,
        };
        let canvas_appearance =
            CanvasAppearance::from_star_appearance(&star_appearance, &viewport).unwrap();
        println!("radius: {}", canvas_appearance.radius);
        assert!(canvas_appearance.radius > 1.);
        assert!(canvas_appearance.radius < 10.);
    }

    #[test]
    fn the_sun_is_very_bright() {
        let star_appearance = StarAppearance::new(
            String::new(),
            apparent_magnitude_to_illuminance(-26.72),
            SOME_COLOR,
            Ecliptic::x_direction(),
            Time::new::<year>(0.),
        );
        let viewport = Viewport {
            center_direction: Direction::X,
            top_direction: Direction::Y,
            px_per_distance: SOME_FLOAT,
        };
        let canvas_appearance =
            CanvasAppearance::from_star_appearance(&star_appearance, &viewport).unwrap();
        println!("radius: {}", canvas_appearance.radius);
        assert!(canvas_appearance.radius > 500.);
    }

    #[test]
    fn recreating_picture_appearance() {
        const PICTURE_MIN_RADIUS: f32 = 1.5;
        struct PictureStar {
            name: &'static str,
            magnitude: f64,
            diameter: i32,
            alpha: f32,
        }
        let picture_stars = vec![
            PictureStar {
                name: "Elnath",
                magnitude: 1.65,
                diameter: 5,
                alpha: 1.,
            },
            PictureStar {
                name: "Zeta Tauri",
                magnitude: 3.01,
                diameter: 4,
                alpha: 1.,
            },
            PictureStar {
                name: "Epsilon Tauri",
                magnitude: 3.53,
                diameter: 3,
                alpha: 0.87,
            },
            PictureStar {
                name: "Aldebaran",
                magnitude: 0.87,
                diameter: 6,
                alpha: 1.,
            },
            PictureStar {
                name: "Gamma Tauri",
                magnitude: 3.65,
                diameter: 3,
                alpha: 0.67,
            },
            PictureStar {
                name: "Meissa",
                magnitude: 3.47,
                diameter: 3,
                alpha: 0.92,
            },
            PictureStar {
                name: "Betelgeuse",
                magnitude: 0.42,
                diameter: 6,
                alpha: 1.,
            },
            PictureStar {
                name: "Bellatrix",
                magnitude: 1.64,
                diameter: 5,
                alpha: 1.,
            },
            PictureStar {
                name: "Pi1 Ori",
                magnitude: 4.74,
                diameter: 3,
                alpha: 0.10,
            },
            PictureStar {
                name: "Pi2 Ori",
                magnitude: 4.35,
                diameter: 3,
                alpha: 0.20,
            },
            PictureStar {
                name: "Pi3 Ori",
                magnitude: 3.16,
                diameter: 3,
                alpha: 0.90,
            },
            PictureStar {
                name: "Pi4 Ori",
                magnitude: 3.69,
                diameter: 3,
                alpha: 0.75,
            },
            PictureStar {
                name: "Pi5 Ori",
                magnitude: 3.69,
                diameter: 3,
                alpha: 0.70,
            },
            PictureStar {
                name: "Pi6 Ori",
                magnitude: 4.47,
                diameter: 3,
                alpha: 0.30,
            },
            PictureStar {
                name: "Alnitak",
                magnitude: 1.88,
                diameter: 5,
                alpha: 1.,
            },
            PictureStar {
                name: "Alnilam",
                magnitude: 1.69,
                diameter: 5,
                alpha: 1.,
            },
            PictureStar {
                name: "Mintaka",
                magnitude: 2.20,
                diameter: 4,
                alpha: 1.,
            },
            PictureStar {
                name: "Saiph",
                magnitude: 2.07,
                diameter: 5,
                alpha: 1.,
            },
            PictureStar {
                name: "Riegel",
                magnitude: 0.18,
                diameter: 7,
                alpha: 1.,
            },
            PictureStar {
                name: "Sirius",
                magnitude: -1.46,
                diameter: 10,
                alpha: 1.,
            },
        ];
        let accuracy = 0.5;

        let mut failures = 0;
        for picture_star in picture_stars.iter() {
            let illuminance = apparent_magnitude_to_illuminance(picture_star.magnitude);
            let star_appearance = StarAppearance::new(
                picture_star.name.to_string(),
                illuminance,
                SOME_COLOR,
                Ecliptic::x_direction(),
                Time::new::<year>(0.),
            );
            let (color, radius) = CanvasAppearance::color_and_radius(&star_appearance);
            let expected_radius = picture_star.diameter as f32 / 2. * CanvasAppearance::MIN_RADIUS
                / PICTURE_MIN_RADIUS;
            let expected_alpha = picture_star.alpha;
            if (radius / expected_radius - 1.).abs() > accuracy
                || (color.a - expected_alpha).abs() > accuracy
            {
                failures += 1;
                println!("\nname: {}", picture_star.name);
                println!(
                    "illuminance: {} ({:2.2e} lux)",
                    illuminance.astro_display(),
                    illuminance.get::<lux>(),
                );
                println!("radius: {}, alpha: {}", radius, color.a);
                println!(
                    "expected radius: {}, expected alpha: {}",
                    expected_radius, expected_alpha
                );
            }
        }
        println!("failures: {} / {}", failures, picture_stars.len());
        assert!(failures == 0);
    }

    #[test]
    fn aligned_planet_sun_and_observer() {
        const CENTER: Vector = Vector { x: 0., y: 0. };

        let mut celestial_system = CelestialSystem::new(sun().to_star_data());
        let orbit = OrbitParameters::new(
            Length::new::<astronomical_unit>(1.),
            0.,
            Angle::new::<degree>(0.),
            Angle::new::<degree>(0.),
            Angle::new::<degree>(0.),
        );
        let planet_physical_params = PlanetPhysicalParameters::new(
            Mass::new::<earth_mass>(1.),
            Length::new::<earth_radius>(1.),
            1.,
            sRGBColor::from_sRGB(1., 1., 1.),
            Time::new::<year>(0.),
            Direction::Z,
        );
        let planet_data = PlanetData::new("Inner".to_string(), planet_physical_params, orbit);
        celestial_system.add_planet_data(planet_data);
        let planets = celestial_system.get_planets();
        let planet = planets.first().unwrap();
        let planet_position = planet.get_position();

        let away_from_sun = planet_position.to_direction().unwrap();
        let to_sun = -&away_from_sun;
        let viewport_away_from_sun = Viewport {
            center_direction: away_from_sun,
            top_direction: Direction::Z,
            px_per_distance: SOME_FLOAT,
        };
        let viewport_to_sun = Viewport {
            center_direction: to_sun,
            top_direction: Direction::Z,
            px_per_distance: SOME_FLOAT,
        };

        let inner_observer = planet_position * 0.5;

        let sun_appearance = CanvasAppearance::from_central_body(
            &celestial_system,
            &viewport_to_sun,
            &inner_observer,
        );
        assert!(sun_appearance.is_some());
        let sun_appearance = sun_appearance.unwrap();
        assert!(vecs_equal(sun_appearance.center_offset, CENTER));

        let planet_appearance = CanvasAppearance::from_planet(
            &celestial_system,
            &planet,
            &viewport_away_from_sun,
            &inner_observer,
        );
        assert!(planet_appearance.is_some());
        let planet_appearance = planet_appearance.unwrap();
        assert!(vecs_equal(planet_appearance.center_offset, CENTER));

        let outer_observer = planet_position * 1.5;
        let sun_appearance = CanvasAppearance::from_central_body(
            &celestial_system,
            &viewport_to_sun,
            &outer_observer,
        );
        assert!(sun_appearance.is_some());
        let sun_appearance = sun_appearance.unwrap();
        assert!(vecs_equal(sun_appearance.center_offset, CENTER));

        let planet_appearance = CanvasAppearance::from_planet(
            &celestial_system,
            &planet,
            &viewport_to_sun,
            &outer_observer,
        );
        assert!(planet_appearance.is_some());
        let planet_appearance = planet_appearance.unwrap();
        assert!(vecs_equal(planet_appearance.center_offset, CENTER));
    }
}
