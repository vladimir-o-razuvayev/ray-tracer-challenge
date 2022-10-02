use crate::color::*;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub(crate) struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            pixels: vec![BLACK; width * height],
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            self.pixels[x + y * self.width] = color;
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        match self.pixels.get(x + y * self.width) {
            Some(color) => color.clone(),
            None => BLACK,
        }
    }

    pub fn to_ppm(self) -> std::io::Result<()> {
        let mut file = File::create("foo.ppm")?;
        file.write_fmt(format_args!("P3\n{} {}\n255\n", self.width, self.height))?;
        file.write(
            self.pixels
                .iter()
                .map(|color| color.to_string())
                .collect::<String>()
                .as_bytes(),
        )
        .unwrap();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{canvas::Canvas, color::*};

    #[test]
    fn new_canvas_is_black() {
        let canvas = Canvas::new(2, 3);
        assert_eq!(canvas.pixels.len(), 6);
        canvas.pixels.iter().for_each(|c| assert_eq!(*c, BLACK));
    }
    #[test]
    fn write_pixel_to_canvas() {
        let mut canvas = Canvas::new(2, 3);
        canvas.write_pixel(1, 1, WHITE);
        assert_eq!(canvas.pixel_at(1, 1), WHITE);
    }
}
