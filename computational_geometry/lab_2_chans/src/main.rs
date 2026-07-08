use macroquad::prelude::*;
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Point {
    x: f64,
    y: f64,
}

fn cross(o: Point, a: Point, b: Point) -> f64 {
    (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x)
}

fn dist2(a: Point, b: Point) -> f64 {
    (a.x - b.x).powi(2) + (a.y - b.y).powi(2)
}

fn graham_scan(mut pts: Vec<Point>) -> Vec<Point> {
    if pts.len() <= 1 {
        return pts;
    }
    pts.sort_by(|a, b| {
        a.y.partial_cmp(&b.y)
            .unwrap()
            .then(a.x.partial_cmp(&b.x).unwrap())
    });
    let start = pts[0];
    pts[1..].sort_by(|a, b| {
        let ca = cross(start, *a, *b);
        if ca.abs() < 1e-9 {
            dist2(start, *a).partial_cmp(&dist2(start, *b)).unwrap()
        } else if ca > 0.0 {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
    let mut hull = vec![start];
    for p in pts.into_iter().skip(1) {
        while hull.len() >= 2 && cross(hull[hull.len() - 2], hull[hull.len() - 1], p) <= 0.0 {
            hull.pop();
        }
        hull.push(p);
    }
    hull
}

fn jarvis_next(hulls: &[Vec<Point>], current: Point, visited: &[Point]) -> Option<Point> {
    let mut best = None;
    for hull in hulls {
        for &p in hull {
            if p == current || visited.contains(&p) {
                continue;
            }
            if let Some(b) = best {
                let c = cross(current, b, p);
                if c < 0.0 || (c == 0.0 && dist2(current, p) > dist2(current, b)) {
                    best = Some(p);
                }
            } else {
                best = Some(p);
            }
        }
    }
    best
}

const POINTS: [Point; 12] = [
    Point { x: 30.0, y: 40.0 },
    Point { x: 60.0, y: 20.0 },
    Point { x: 110.0, y: 30.0 },
    Point { x: 150.0, y: 10.0 },
    Point { x: 190.0, y: 50.0 },
    Point { x: 170.0, y: 100.0 },
    Point { x: 140.0, y: 140.0 },
    Point { x: 90.0, y: 150.0 },
    Point { x: 50.0, y: 120.0 },
    Point { x: 20.0, y: 90.0 },
    Point { x: 10.0, y: 60.0 },
    Point { x: 100.0, y: 80.0 },
];

enum Phase {
    Idle,
    Grouping,
    BuildingHulls,
    Jarvis,
    Done,
}

struct AlgorithmState {
    points: Vec<Point>,
    current_m: usize,
    iteration: usize,
    groups: Vec<Vec<Point>>,
    hulls: Vec<Vec<Point>>,
    current_hull_vertices: Vec<Point>,
    phase: Phase,
    step_counter: usize,
    finished: bool,
    final_hull: Vec<Point>,
    jarvis_steps: usize,
}

impl AlgorithmState {
    fn new(points: &[Point]) -> Self {
        AlgorithmState {
            points: points.to_vec(),
            current_m: 2,
            iteration: 0,
            groups: Vec::new(),
            hulls: Vec::new(),
            current_hull_vertices: Vec::new(),
            phase: Phase::Idle,
            step_counter: 0,
            finished: false,
            final_hull: Vec::new(),
            jarvis_steps: 0,
        }
    }

    fn step(&mut self) -> Option<String> {
        if self.finished {
            return None;
        }

        match self.phase {
            Phase::Idle => {
                self.iteration += 1;
                let m = self.current_m;
                self.groups = self.points.chunks(m).map(|ch| ch.to_vec()).collect();
                self.phase = Phase::Grouping;
                self.step_counter += 1;
                Some(format!("Iter {}, m={}", self.iteration, m))
            }
            Phase::Grouping => {
                self.hulls.clear();
                self.phase = Phase::BuildingHulls;
                self.step_counter += 1;
                Some(format!("Groups: {}", self.groups.len()))
            }
            Phase::BuildingHulls => {
                if self.hulls.len() < self.groups.len() {
                    let idx = self.hulls.len();
                    let h = graham_scan(self.groups[idx].clone());
                    self.hulls.push(h);
                    self.step_counter += 1;
                    Some(format!("Group {}: {} vertices", idx + 1, self.hulls.last().unwrap().len()))
                } else {
                    self.current_hull_vertices.clear();
                    self.jarvis_steps = 0;
                    let start = *self.points.iter()
                        .min_by(|a, b| a.y.partial_cmp(&b.y).unwrap().then(a.x.partial_cmp(&b.x).unwrap()))
                        .unwrap();
                    self.current_hull_vertices.push(start);
                    self.phase = Phase::Jarvis;
                    self.step_counter += 1;
                    Some(format!("Jarvis start ({:.0},{:.0})", start.x, start.y))
                }
            }
            Phase::Jarvis => {
                let current = *self.current_hull_vertices.last().unwrap();
                let next = jarvis_next(&self.hulls, current, &self.current_hull_vertices);

                match next {
                    Some(nxt) => {
                        if nxt == self.current_hull_vertices[0] && self.current_hull_vertices.len() > 2 {
                            self.current_hull_vertices.pop();
                            self.final_hull = self.current_hull_vertices.clone();
                            self.phase = Phase::Done;
                            self.finished = true;
                            self.step_counter += 1;
                            Some(format!("Closed! {} vertices", self.final_hull.len()))
                        } else {
                            self.current_hull_vertices.push(nxt);
                            self.jarvis_steps += 1;
                            self.step_counter += 1;
                            if self.jarvis_steps >= self.current_m {
                                self.current_m *= 4;
                                self.phase = Phase::Idle;
                                if self.current_m > self.points.len() {
                                    self.final_hull = graham_scan(self.points.clone());
                                    self.phase = Phase::Done;
                                    self.finished = true;
                                    self.step_counter += 1;
                                    Some("m > n → full Graham".to_string())
                                } else {
                                    self.step_counter += 1;
                                    Some(format!("m reached, increasing to {}", self.current_m))
                                }
                            } else {
                                Some(format!("Added ({:.0},{:.0})", nxt.x, nxt.y))
                            }
                        }
                    }
                    None => {
                        if self.current_hull_vertices.len() > 2 {
                            self.final_hull = self.current_hull_vertices.clone();
                            self.phase = Phase::Done;
                            self.finished = true;
                            self.step_counter += 1;
                            Some(format!("No more points, closed ({} vertices)", self.final_hull.len()))
                        } else {
                            self.current_m *= 4;
                            self.phase = Phase::Idle;
                            self.step_counter += 1;
                            Some(format!("Failed, m={}", self.current_m))
                        }
                    }
                }
            }
            Phase::Done => {
                self.finished = true;
                None
            }
        }
    }
}

#[macroquad::main("Convex Hull – Chan's Algorithm")]
async fn main() {
    let mut state = AlgorithmState::new(&POINTS);
    let mut messages: Vec<String> = Vec::new();
    let mut step_count = 0;

    let window_w = 800.0;
    let window_h = 600.0;
    let margin = 40.0;
    let mut min_x = POINTS[0].x;
    let mut max_x = POINTS[0].x;
    let mut min_y = POINTS[0].y;
    let mut max_y = POINTS[0].y;
    for p in POINTS.iter() {
        if p.x < min_x { min_x = p.x; }
        if p.x > max_x { max_x = p.x; }
        if p.y < min_y { min_y = p.y; }
        if p.y > max_y { max_y = p.y; }
    }
    let range_x = (max_x - min_x).max(1.0);
    let range_y = (max_y - min_y).max(1.0);
    let scale = ((window_w - 2.0 * margin) / range_x)
        .min((window_h - 2.0 * margin) / range_y);
    let offset_x = (min_x + max_x) / 2.0;
    let offset_y = (min_y + max_y) / 2.0;

    fn to_screen(p: Point, scale: f64, ox: f64, oy: f64) -> Vec2 {
        vec2(
            ((p.x - ox) * scale + 400.0) as f32,
            ((p.y - oy) * scale + 300.0) as f32,
        )
    }

    loop {
        clear_background(WHITE);

        if is_key_pressed(KeyCode::Space) {
            if let Some(msg) = state.step() {
                messages.push(msg);
                step_count += 1;
                if messages.len() > 12 {
                    messages.remove(0);
                }
                println!("Step {}: {}", step_count, messages.last().unwrap());
            }
        }

        for p in POINTS.iter() {
            let pos = to_screen(*p, scale, offset_x, offset_y);
            draw_circle(pos.x, pos.y, 4.0, GRAY);
        }

        for (i, group) in state.groups.iter().enumerate() {
            let color = Color::from_rgba(
                ((i * 30) % 255) as u8,
                ((i * 70) % 255) as u8,
                200,
                100,
            );
            for p in group {
                let pos = to_screen(*p, scale, offset_x, offset_y);
                draw_circle(pos.x, pos.y, 3.0, color);
            }
        }

        for (i, hull) in state.hulls.iter().enumerate() {
            let color = Color::from_rgba(
                ((i * 50) % 255) as u8,
                ((i * 120) % 255) as u8,
                50,
                200,
            );
            if hull.len() >= 3 {
                for j in 0..hull.len() {
                    let a = to_screen(hull[j], scale, offset_x, offset_y);
                    let b = to_screen(hull[(j + 1) % hull.len()], scale, offset_x, offset_y);
                    draw_line(a.x, a.y, b.x, b.y, 2.0, color);
                }
            }
        }

        for (i, &p) in state.current_hull_vertices.iter().enumerate() {
            let pos = to_screen(p, scale, offset_x, offset_y);
            draw_circle(pos.x, pos.y, 7.0, RED);
            if i > 0 {
                let prev_pos = to_screen(state.current_hull_vertices[i - 1], scale, offset_x, offset_y);
                draw_line(prev_pos.x, prev_pos.y, pos.x, pos.y, 2.0, RED);
            }
        }

        if state.finished && !state.final_hull.is_empty() {
            let hull = &state.final_hull;
            for i in 0..hull.len() {
                let a = to_screen(hull[i], scale, offset_x, offset_y);
                let b = to_screen(hull[(i + 1) % hull.len()], scale, offset_x, offset_y);
                draw_line(a.x, a.y, b.x, b.y, 3.0, ORANGE);
            }
            for &p in hull {
                let pos = to_screen(p, scale, offset_x, offset_y);
                draw_circle(pos.x, pos.y, 6.0, ORANGE);
            }
        }

        draw_text("SPACE = next step", 20.0, 30.0, 20.0, DARKGRAY);
        draw_text(&format!("Steps: {}", step_count), 20.0, 55.0, 20.0, DARKGRAY);
        draw_text(&format!("m = {}", state.current_m), 20.0, 80.0, 20.0, DARKGRAY);
        draw_text(&format!("Iteration: {}", state.iteration), 20.0, 105.0, 20.0, DARKGRAY);

        let mut y_pos = 135.0;
        for msg in messages.iter().rev().take(10) {
            draw_text(msg, 20.0, y_pos, 16.0, BLACK);
            y_pos += 20.0;
        }

        next_frame().await;
    }
}