use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

const POLYGON: [Point; 5] = [
    Point { x: 100.0, y: 100.0 },
    Point { x: 250.0, y: 80.0 },
    Point { x: 320.0, y: 180.0 },
    Point { x: 220.0, y: 320.0 },
    Point { x: 80.0, y: 250.0 },
];

const POINT_INSIDE: Point = Point { x: 180.0, y: 180.0 };
const POINT_OUTSIDE: Point = Point { x: 350.0, y: 250.0 };

fn ray_casting(point: Point, polygon: &[Point]) -> bool {
    let mut intersections = 0;
    let n = polygon.len();

    for i in 0..n {
        let a = polygon[i];
        let b = polygon[(i + 1) % n];

        if (a.y > point.y) != (b.y > point.y) {
            let x_intersection =
                a.x + (point.y - a.y) * (b.x - a.x) / (b.y - a.y);

            if point.x < x_intersection {
                intersections += 1;
            }
        }
    }

    intersections % 2 == 1
}

fn to_screen(p: Point, offset_x: f32, offset_y: f32, scale: f32) -> Vec2 {
    vec2(
        (p.x as f32 - offset_x) * scale,
        (p.y as f32 - offset_y) * scale,
    )
}

fn draw_polygon(vertices: &[Vec2], color: Color) {
    if vertices.len() < 3 {
        return;
    }
    let v0 = vertices[0];
    for i in 1..vertices.len() - 1 {
        let v1 = vertices[i];
        let v2 = vertices[i + 1];
        draw_triangle(v0, v1, v2, color);
    }
}

#[macroquad::main("Ray Casting Demo")]
async fn main() {
    let mut min_x = POLYGON[0].x;
    let mut max_x = POLYGON[0].x;
    let mut min_y = POLYGON[0].y;
    let mut max_y = POLYGON[0].y;

    for p in POLYGON.iter() {
        if p.x < min_x { min_x = p.x; }
        if p.x > max_x { max_x = p.x; }
        if p.y < min_y { min_y = p.y; }
        if p.y > max_y { max_y = p.y; }
    }

    let margin = 50.0;
    let world_width = (max_x - min_x) + 2.0 * margin;
    let world_height = (max_y - min_y) + 2.0 * margin;

    let target_size = 600.0;
    let scale = (target_size / world_width.max(world_height) as f32) * 0.7;

    let offset_x = (min_x - margin) as f32;
    let offset_y = (min_y - margin) as f32;

    let screen_poly: Vec<Vec2> = POLYGON
        .iter()
        .map(|p| to_screen(*p, offset_x, offset_y, scale))
        .collect();

    // Результаты проверки точек
    let inside_result = ray_casting(POINT_INSIDE, &POLYGON);
    let outside_result = ray_casting(POINT_OUTSIDE, &POLYGON);

    loop {
        clear_background(WHITE);

        draw_polygon(
            &screen_poly,
            Color::new(0.2, 0.6, 1.0, 0.3),
        );

        for i in 0..screen_poly.len() {
            let a = screen_poly[i];
            let b = screen_poly[(i + 1) % screen_poly.len()];
            draw_line(a.x, a.y, b.x, b.y, 2.0, BLUE);
        }

        let inside_screen = to_screen(POINT_INSIDE, offset_x, offset_y, scale);
        draw_circle(inside_screen.x, inside_screen.y, 8.0, GREEN);
        draw_circle_lines(inside_screen.x, inside_screen.y, 8.0, 2.0, Color::new(0.0, 0.5, 0.0, 1.0)); // тёмно-зелёный
        draw_text(
            &format!("Inside ({:.1}, {:.1})", POINT_INSIDE.x, POINT_INSIDE.y),
            inside_screen.x + 12.0,
            inside_screen.y - 6.0,
            18.0,
            Color::new(0.0, 0.5, 0.0, 1.0),
        );

        let outside_screen = to_screen(POINT_OUTSIDE, offset_x, offset_y, scale);
        draw_circle(outside_screen.x, outside_screen.y, 8.0, RED);
        draw_circle_lines(outside_screen.x, outside_screen.y, 8.0, 2.0, Color::new(0.5, 0.0, 0.0, 1.0)); // тёмно-красный
        draw_text(
            &format!("Outside ({:.1}, {:.1})", POINT_OUTSIDE.x, POINT_OUTSIDE.y),
            outside_screen.x + 12.0,
            outside_screen.y - 6.0,
            18.0,
            Color::new(0.5, 0.0, 0.0, 1.0),
        );

        draw_text(
            &format!("Inside point result: {}", inside_result),
            20.0,
            40.0,
            24.0,
            Color::new(0.0, 0.5, 0.0, 1.0),
        );
        draw_text(
            &format!("Outside point result: {}", outside_result),
            20.0,
            70.0,
            24.0,
            Color::new(0.5, 0.0, 0.0, 1.0),
        );

        next_frame().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_inside() {
        assert!(ray_casting(POINT_INSIDE, &POLYGON));
    }

    #[test]
    fn point_outside() {
        assert!(!ray_casting(POINT_OUTSIDE, &POLYGON));
    }
}