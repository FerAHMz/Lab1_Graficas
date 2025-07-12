use raylib::prelude::*;
use std::convert::TryInto;

pub struct Framebuffer {
    width: u32,
    height: u32,
    color_buffer: Image,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(
            width.try_into().unwrap(),
            height.try_into().unwrap(),
            background_color,
        );
        Framebuffer {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(
            self.width.try_into().unwrap(),
            self.height.try_into().unwrap(),
            self.background_color,
        );
    }

    pub fn set_pixel(&mut self, x: u32, y: u32) {
        if x < self.width && y < self.height {
            Image::draw_pixel(&mut self.color_buffer, x as i32, y as i32, self.current_color);
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn render_to_file(&self, file_path: &str) {
        let img = self.color_buffer.clone();
        img.export_image(file_path);
    }
}

/// Rellena un polígono con color actual, y opcionalmente excluye un agujero interior.
pub fn fill_polygon(fb: &mut Framebuffer, points: &[Vector2], skip: Option<&[Vector2]>) {
    let mut edges = vec![];

    for i in 0..points.len() {
        let p1 = points[i];
        let p2 = points[(i + 1) % points.len()];
        if (p1.y - p2.y).abs() < f32::EPSILON {
            continue;
        }
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
                if x_start > x_end {
                    std::mem::swap(&mut x_start, &mut x_end);
                }

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

/// Determina si un punto está dentro de un polígono usando el método de ray casting.
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
