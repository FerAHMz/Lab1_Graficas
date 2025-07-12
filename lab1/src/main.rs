mod framebuffer;
mod line;

use framebuffer::{Framebuffer, fill_polygon};
use line::line;
use raylib::prelude::*;

fn main() {
    let width = 800;
    let height = 600;
    let mut fb = Framebuffer::new(width, height, Color::RAYWHITE);
    fb.clear();

    let poly = vec![
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383),
    ];

    let points: Vec<Vector2> = poly.iter()
        .map(|&(x, y)| Vector2::new(x as f32, (height - y) as f32))
        .collect();

    fb.set_current_color(Color::YELLOW);
    fill_polygon(&mut fb, &points, None);

    fb.set_current_color(Color::WHITE);
    for i in 0..points.len() {
        line(&mut fb, points[i], points[(i + 1) % points.len()]);
    }

    fb.render_to_file("out.bmp");
}
