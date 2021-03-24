mod engine;

use engine::backend::implementations::mini_fb::MiniFb;
use engine::backend::window_backend::WindowBackend;
use engine::domain::shapes_3d::mesh::Mesh;
use engine::domain::shapes_3d::triangle::Triangle;
use engine::domain::shapes_3d::point::Point;
use engine::domain::frame::Color;
use engine::domain::frame::Frame;

fn main() {
    let mut window = MiniFb::new("Hello World!", 640, 480);

    let mut rx = 0.0;
    let mut ry = 0.0;
    while window.is_open() {
        rx += 0.08;
        ry += 0.03;
        
        let (width, height) = window.get_dimensions();

        let mut frame = Frame::new(width, height);
        let some_cube = Mesh {
            triangles: vec![
                // Back
                Triangle {
                    p1: Point::new(1.0, 1.0, -1.0),
                    p2: Point::new(1.0, -1.0, -1.0),
                    p3: Point::new(-1.0, -1.0, -1.0),
                    color: Color { r: 0, g: 0, b: 255 }
                },
                Triangle {
                    p1: Point::new(1.0, 1.0, -1.0),
                    p2: Point::new(-1.0, -1.0, -1.0),
                    p3: Point::new(-1.0, 1.0, -1.0),
                    color: Color { r: 0, g: 0, b: 255 }
                },
                // Front
                Triangle {
                    p1: Point::new(1.0, 1.0, 1.0),
                    p2: Point::new(1.0, -1.0, 1.0),
                    p3: Point::new(-1.0, -1.0, 1.0),
                    color: Color { r: 0, g: 255, b: 0 }
                },
                Triangle {
                    p1: Point::new(1.0, 1.0, 1.0),
                    p2: Point::new(-1.0, -1.0, 1.0),
                    p3: Point::new(-1.0, 1.0, 1.0),
                    color: Color { r: 0, g: 255, b: 0 }
                },
                // Left
                Triangle {
                    p1: Point::new(1.0, 1.0, 1.0),
                    p2: Point::new(1.0, -1.0, 1.0),
                    p3: Point::new(1.0, 1.0, -1.0),
                    color: Color { r: 255, g: 0, b: 0 }
                },
                Triangle {
                    p1: Point::new(1.0, -1.0, 1.0),
                    p2: Point::new(1.0, -1.0, -1.0),
                    p3: Point::new(1.0, 1.0, -1.0),
                    color: Color { r: 255, g: 0, b: 0 }
                },
                // Right
                Triangle {
                    p1: Point::new(-1.0, 1.0, 1.0),
                    p2: Point::new(-1.0, -1.0, 1.0),
                    p3: Point::new(-1.0, 1.0, -1.0),
                    color: Color { r: 255, g: 255, b: 255 }
                },
                Triangle {
                    p1: Point::new(-1.0, -1.0, 1.0),
                    p2: Point::new(-1.0, -1.0, -1.0),
                    p3: Point::new(-1.0, 1.0, -1.0),
                    color: Color { r: 255, g: 255, b: 255 }
                },
            ]
        };

        let rotated = some_cube.rotate(rx, ry, 0.0);
        let translated = rotated.translate(0.0, 0.0, 25.0);
        let some_cube_as_triangles = translated.project_to_screenspace(
            0.1,
            1000.0,
            90.0,
            width as f64,
            height as f64
        );

        for triangle in some_cube_as_triangles {
            frame.draw_shape(&triangle);
        }

        window.draw(frame);
    }
}
