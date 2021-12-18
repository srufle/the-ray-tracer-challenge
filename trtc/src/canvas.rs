use crate::color::Color;

#[derive(Debug, Clone)]
pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Canvas {
        let size = width * height;
        let mut pixels = Vec::with_capacity(size as usize);
        for _ in 0..size {
            pixels.push(Color::new(0.0, 0.0, 0.0));
        }

        Canvas {
            width,
            height,
            pixels,
        }
    }
    // https://www.google.com/search?q=Jamis+Buck+rust&oq=Jamis+Buck+rust
    // https://github.com/ahamez/ray-tracer
    // https://github.com/magnusstrale/raytracer
    // https://iwearshorts.com/blog/find-the-xy-coordinates-of-an-index-value-in-an-array/
    // https://softwareengineering.stackexchange.com/questions/212808/treating-a-1d-data-structure-as-2d-grid
    pub fn write_pixel(&mut self, x: u32, y: u32, color: Color) {
        // let canvas = canvas.clone();
        // let mut pixels = canvas.pixels.clone();
        let index = self.calc_index(x, y);

        self.pixels[index] = color;
    }

    pub fn pixel_at(&self, x: u32, y: u32) -> Color {
        let index = self.calc_index(x, y);
        self.pixels[index]
    }
    pub fn calc_index(&self, x: u32, y: u32) -> usize {
        let ans = (x + (self.width * y)) as usize;
        ans
    }

    pub fn size(self) -> u32 {
        self.width * self.height
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_canvas_create() {
        let canvas = Canvas::new(10, 20);

        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);
        assert_eq!(canvas.pixels.len(), 200);
        let black = Color::black();
        for pixel in canvas.pixels {
            assert_eq!(pixel, black);
        }
    }

    #[test]
    fn test_canvas_write() {
        let mut canvas = Canvas::new(10, 20);
        let red = Color::red();
        canvas.write_pixel(2, 3, red);
        // let canvas = Canvas::write_pixel(canvas, 2, 3, red);

        let pixel = canvas.pixel_at(2, 3);
        assert_eq!(pixel, red);
    }
}
