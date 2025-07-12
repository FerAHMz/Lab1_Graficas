mod framebuffer;
mod line;

use framebuffer::Framebuffer;
use line::line;
use raylib::prelude::*;

fn main() {
    let width = 800;
    let height = 600;
    let mut fb = Framebuffer::new(width, height, Color::RAYWHITE);
    fb.clear();

    let poly = vec![
        (321, 335), (288, 286), (339, 251), (374, 302),
    ];

    let points: Vec<Vector2> = poly.iter()
        .map(|&(x, y)| Vector2::new(x as f32, (height - y) as f32))
        .collect();

    fb.set_current_color(Color::BLUE);
    fill_polygon(&mut fb, &points, None);

    fb.set_current_color(Color::WHITE);
    for i in 0..points.len() {
        line(&mut fb, points[i], points[(i + 1) % points.len()]);
    }

    fb.render_to_file("out.bmp");
}


// --- funciones auxiliares abajo ---
fn fill_polygon(fb: &mut Framebuffer, points: &[Vector2], skip: Option<&[Vector2]>) {
    let mut edges = vec![];

    for i in 0..points.len() {
        let p1 = points[i];
        let p2 = points[(i + 1) % points.len()];
        if (p1.y - p2.y).abs() < f32::EPSILON { continue; }
        let (p1, p2) = if p1.y < p2.y { (p1, p2) } else { (p2, p1) };
        let inv_slope = (p2.x - p1.x) / (p2.y - p1.y);
        edges.push((p1.y, p2.y, p1.x, inv_slope));
    }

    let y_min = points.iter().map(|p| p.y).fold(f32::INFINITY, f32::min) as i32;
    let y_max = points.iter().map(|p| p.y).fold(f32::NEG_INFINITY, f32::max) as i32;

    for y in y_min..=y_max {
        let y_f = y as f32;
        let mut intersections = vec![];
        for (y0, y1, x0, inv_slope) in &edges {
            if y_f >= *y0 && y_f < *y1 {
                intersections.push(x0 + inv_slope * (y_f - y0));
            }
        }
        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for chunk in intersections.chunks(2) {
            if chunk.len() == 2 {
                let mut x_start = chunk[0] as i32;
                let mut x_end = chunk[1] as i32;
                if x_start > x_end { std::mem::swap(&mut x_start, &mut x_end); }

                for x in x_start..=x_end {
                    let mut inside_hole = false;
                    if let Some(hole) = skip {
                        inside_hole = point_in_polygon(Vector2::new(x as f32, y_f), hole);
                    }
                    if !inside_hole {
                        fb.set_pixel(x as u32, y as u32);
                    }
                }
            }
        }
    }
}

fn point_in_polygon(p: Vector2, polygon: &[Vector2]) -> bool {
    let mut inside = false;
    let mut j = polygon.len() - 1;
    for i in 0..polygon.len() {
        let pi = polygon[i];
        let pj = polygon[j];
        if ((pi.y > p.y) != (pj.y > p.y)) &&
           (p.x < (pj.x - pi.x) * (p.y - pi.y) / (pj.y - pi.y) + pi.x) {
            inside = !inside;
        }
        j = i;
    }
    inside
}
