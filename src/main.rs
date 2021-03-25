mod engine;

use engine::backend::implementations::mini_fb::MiniFb;
use engine::backend::window_backend::WindowBackend;
use engine::domain::shapes_3d::mesh::FillMesh;
use engine::domain::shapes_3d::triangle::FillTriangle;
use engine::domain::shapes_3d::point::Point;
use engine::domain::frame::Color;
use engine::domain::frame::Frame;

fn main() {
    let mut window = MiniFb::new("Hello World!", 640, 480);

    let mut x_step = 0.01;
    let mut x = 0.0;
    let mut rx = 0.0;
    let mut ry = 0.0;

    while window.is_open() {
        x += x_step;
        rx += 0.03;
        ry += 0.01;

        if x < -1.0 {
            x_step = 0.01;
        };
        if x > 1.0 {
            x_step = -0.01;
        };

        let (width, height) = window.get_dimensions();
        let mut frame = Frame::new(width, height);
        let some_cube = FillMesh::from_triangles(
            vec![
                // Back
                FillTriangle::new(
                    Point::new(1.0, 1.0, -1.0),
                    Point::new(1.0, -1.0, -1.0),
                    Point::new(-1.0, -1.0, -1.0),
                    Color { r: 0, g: 0, b: 255 }
                ),
                FillTriangle::new(
                    Point::new(1.0, 1.0, -1.0),
                    Point::new(-1.0, -1.0, -1.0),
                    Point::new(-1.0, 1.0, -1.0),
                    Color { r: 0, g: 0, b: 255 }
                ),
                // Front
                FillTriangle::new(
                    Point::new(1.0, 1.0, 1.0),
                    Point::new(1.0, -1.0, 1.0),
                    Point::new(-1.0, -1.0, 1.0),
                    Color { r: 0, g: 255, b: 0 }
                ),
                FillTriangle::new(
                    Point::new(1.0, 1.0, 1.0),
                    Point::new(-1.0, -1.0, 1.0),
                    Point::new(-1.0, 1.0, 1.0),
                    Color { r: 0, g: 255, b: 0 }
                ),
                // Left
                FillTriangle::new(
                    Point::new(1.0, 1.0, 1.0),
                    Point::new(1.0, -1.0, 1.0),
                    Point::new(1.0, 1.0, -1.0),
                    Color { r: 255, g: 0, b: 0 }
                ),
                FillTriangle::new(
                    Point::new(1.0, -1.0, 1.0),
                    Point::new(1.0, -1.0, -1.0),
                    Point::new(1.0, 1.0, -1.0),
                    Color { r: 255, g: 0, b: 0 }
                ),
                // Right
                FillTriangle::new(
                    Point::new(-1.0, 1.0, 1.0),
                    Point::new(-1.0, -1.0, 1.0),
                    Point::new(-1.0, 1.0, -1.0),
                    Color { r: 255, g: 255, b: 255 }
                ),
                FillTriangle::new(
                    Point::new(-1.0, -1.0, 1.0),
                    Point::new(-1.0, -1.0, -1.0),
                    Point::new(-1.0, 1.0, -1.0),
                    Color { r: 255, g: 255, b: 255 }
                )
            ]
        );

        let rotated = some_cube.rotate(rx, ry, 0.0);
        let translated = rotated.translate(x, 0.0, 25.0);
        let some_cube_as_2d = translated.project_to_screenspace(
            0.1,
            1000.0,
            90.0,
            width as f64,
            height as f64
        );

        for triangle in some_cube_as_2d {
            frame.draw_shape(&*triangle);
        }

        window.draw(frame);
    }
}
