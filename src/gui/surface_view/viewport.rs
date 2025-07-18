use astro_coords::{direction::Direction, equatorial::Equatorial, spherical::Spherical, traits::*};
use astro_utils::planets::{planet_data::PlanetData, surface_normal::surface_normal_at_time};
use iced::Rectangle;
use uom::si::f64::{Angle, SolidAngle};

pub(super) struct Viewport {
    pub(super) center_direction: Direction,
    pub(super) top_direction: Direction,
    pub(super) px_per_distance: f32,
}

impl Viewport {
    pub(super) fn calculate(
        observer_normal: &Direction,
        local_view_direction: &Spherical,
        opening_angle: SolidAngle,
        rotation_axis: &Direction,
        bounds: Rectangle,
    ) -> Self {
        let view_direction = local_view_direction.to_direction();
        let center_direction = view_direction.active_rotation_to_new_z_axis(observer_normal);
        let ortho = match center_direction.cross_product(rotation_axis) {
            Ok(ortho) => ortho,
            Err(_) => match observer_normal.cross_product(rotation_axis) {
                Ok(ortho) => ortho,
                Err(_) => center_direction.some_orthogonal_vector(),
            },
        };
        let aspect_ration = bounds.width / bounds.height;
        // A = a * b = a^2 * aspect_ratio
        // a = sqrt(A / aspect_ratio)
        let vertical_angle = Angle {
            rad: (opening_angle.sr / aspect_ration as f64).sqrt(),
        };
        let top_direction = center_direction.rotated(vertical_angle / 2., &ortho);
        let viewport_height = (vertical_angle / 2.).rad.sin() * 2.; //Viewport is at unit distance
        let px_per_distance = bounds.height / viewport_height as f32;
        Self {
            center_direction,
            top_direction,
            px_per_distance,
        }
    }
}

pub(super) fn observer_normal(
    planet: &PlanetData,
    surface_position: Spherical,
    time_since_epoch: Time,
) -> Direction {
    let observer_equatorial_position =
        Equatorial::new(surface_position, planet.get_rotation_axis().clone());
    //TODO: Define Angle at Epoch
    let planet_angle_at_epoch = Angle::new::<degree>(0.0);
    surface_normal_at_time(
        observer_equatorial_position,
        planet_angle_at_epoch,
        time_since_epoch,
        planet.get_sideral_rotation_period(),
    )
}

#[cfg(test)]
mod tests {

    use uom::si::angle::degree;

    use super::*;

    const TEST_ACCURACY: f64 = 1e-5;
    const SOME_SOLID_ANGLE: SolidAngle = SolidAngle { sr: 1.0 };
    const SOME_SQUARE: Rectangle = Rectangle {
        x: 0.,
        y: 0.,
        width: 100.,
        height: 100.,
    };

    fn example_directions() -> Vec<Direction> {
        let ordinates = vec![-1., 0., 1., 12.];
        let mut directions = Vec::new();
        for x1 in ordinates.clone().iter() {
            for y1 in ordinates.clone().iter() {
                for z1 in ordinates.clone().iter() {
                    if let Ok(direction) = Direction::new(*x1, *y1, *z1) {
                        directions.push(direction);
                    }
                }
            }
        }
        directions
    }

    #[test]
    fn view_direction_z_does_not_influence_center_direction_and_makes_rotation_axis_irrelevant() {
        for observer_normal in example_directions().iter() {
            for rotation_axis in example_directions().iter() {
                let view_direction = Spherical::z_direction();
                let viewport = Viewport::calculate(
                    &observer_normal,
                    &view_direction,
                    SOME_SOLID_ANGLE,
                    &rotation_axis,
                    SOME_SQUARE,
                );
                assert!(viewport
                    .center_direction
                    .eq_within(&observer_normal, TEST_ACCURACY));
            }
        }
    }

    #[test]
    fn tilting_view() {
        let observer_normal = Direction::X;
        let rotation_axis = Direction::Z;
        let west_view = Spherical::x_direction();
        let south_view = Spherical::y_direction();
        let east_view = -Spherical::x_direction();
        let north_view = -Spherical::y_direction();
        let westward_viewport = Viewport::calculate(
            &observer_normal,
            &west_view,
            SOME_SOLID_ANGLE,
            &rotation_axis,
            SOME_SQUARE,
        );
        let southward_viewport = Viewport::calculate(
            &observer_normal,
            &south_view,
            SOME_SOLID_ANGLE,
            &rotation_axis,
            SOME_SQUARE,
        );
        let eastward_viewport = Viewport::calculate(
            &observer_normal,
            &east_view,
            SOME_SOLID_ANGLE,
            &rotation_axis,
            SOME_SQUARE,
        );
        let northward_viewport = Viewport::calculate(
            &observer_normal,
            &north_view,
            SOME_SOLID_ANGLE,
            &rotation_axis,
            SOME_SQUARE,
        );
        assert!(westward_viewport
            .center_direction
            .eq_within(&-&Direction::Y, TEST_ACCURACY));
        assert!(southward_viewport
            .center_direction
            .eq_within(&-&Direction::Z, TEST_ACCURACY));
        assert!(eastward_viewport
            .center_direction
            .eq_within(&Direction::Y, TEST_ACCURACY));
        assert!(northward_viewport
            .center_direction
            .eq_within(&Direction::Z, TEST_ACCURACY));
    }

    #[test]
    fn top_direction_aligns_with_rotation_axis() {
        for observer_normal in example_directions().iter() {
            for rotation_axis in example_directions().iter() {
                for view_direction in example_directions().iter() {
                    let view_direction = view_direction.to_spherical();
                    let viewport = Viewport::calculate(
                        &observer_normal,
                        &view_direction,
                        SOME_SOLID_ANGLE,
                        &rotation_axis,
                        SOME_SQUARE,
                    );

                    let ortho = rotation_axis.cross_product(&viewport.center_direction);
                    if ortho.is_err() {
                        continue;
                    }
                    let ortho = ortho.unwrap();
                    let overlap = ortho.dot_product(&viewport.top_direction);

                    assert!(overlap.abs() < TEST_ACCURACY,
                        "center_direction: {}\ntop_direction: {}\nrotation_axis: {}\northo: {}\noverlap: {}", 
                        viewport.center_direction,
                        viewport.top_direction,
                        rotation_axis,
                        ortho,
                        overlap
                    );
                }
            }
        }
    }

    #[test]
    fn opening_angle_zero() {
        let observer_normal = Direction::X;
        let rotation_axis = Direction::Z;
        let view_direction = Spherical::z_direction();
        let viewport = Viewport::calculate(
            &observer_normal,
            &view_direction,
            SOLID_Angle::new::<degree>(0.),
            &rotation_axis,
            SOME_SQUARE,
        );
        let expected_top_direction = viewport.center_direction;
        assert!(viewport
            .top_direction
            .eq_within(&expected_top_direction, TEST_ACCURACY));
    }

    #[test]
    fn opening_angle_90_degrees() {
        let observer_normal = Direction::X;
        let rotation_axis = Direction::Z;
        let view_direction = Spherical::z_direction();
        let opening_angle = Angle::new::<degree>(90.0);
        let opening_solid_angle = opening_angle * opening_angle;
        let viewport = Viewport::calculate(
            &observer_normal,
            &view_direction,
            opening_solid_angle,
            &rotation_axis,
            SOME_SQUARE,
        );

        let expected_top_direction = Direction::new(1., 0., 1.).unwrap();
        assert!(viewport
            .top_direction
            .eq_within(&expected_top_direction, TEST_ACCURACY));
    }

    #[test]
    fn opening_angle_180_degrees() {
        let observer_normal = Direction::X;
        let rotation_axis = Direction::Z;
        let view_direction = Spherical::z_direction();
        let opening_angle = Angle::new::<degree>(180.0);
        let opening_solid_angle = opening_angle * opening_angle;
        let viewport = Viewport::calculate(
            &observer_normal,
            &view_direction,
            opening_solid_angle,
            &rotation_axis,
            SOME_SQUARE,
        );

        let expected_top_direction = rotation_axis;
        assert!(viewport
            .top_direction
            .eq_within(&expected_top_direction, TEST_ACCURACY));
    }
}
