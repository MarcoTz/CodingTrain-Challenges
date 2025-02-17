use ::image::RgbaImage;
use graphics::{
    image as img, Drawable, DrawingContext, EventHandler, Graphics, Runnable, Updatable,
    UpdateContext, WindowConfig,
};
use piston_window::{G2dTexture, TextureSettings};

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;
const IMAGE_FILE: &str = "./challenges/047_pixelsorting/image.png";

pub struct PixelSorting {
    image: RgbaImage,
    next_row: u32,
}

impl PixelSorting {
    pub fn new() -> PixelSorting {
        let image = image::open(IMAGE_FILE).unwrap().into_rgba8();
        PixelSorting { image, next_row: 0 }
    }
}

impl Drawable for PixelSorting {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();
        let texture = G2dTexture::from_image(
            &mut ctx.texture_context,
            &self.image,
            &TextureSettings::new(),
        )
        .unwrap();
        img(&texture, transform, gl);
    }
}

impl Updatable for PixelSorting {
    fn update(&mut self, _: &mut UpdateContext) {
        if self.next_row >= self.image.height() {
            return;
        }
        let width = self.image.width();
        let mut row_vals = vec![];
        for i in 0..width {
            let next_pix = self.image.get_pixel(i, self.next_row);
            row_vals.push(*next_pix);
        }
        row_vals.sort_by(|rgb1, rgb2| {
            (rgb1[0] as u32 + rgb1[1] as u32 + rgb1[2] as u32)
                .cmp(&(rgb2[0] as u32 + rgb2[1] as u32 + rgb2[2] as u32))
        });
        for (ind, new_val) in row_vals.iter().enumerate() {
            let next_pix = self.image.get_pixel_mut(ind as u32, self.next_row);
            *next_pix = *new_val;
        }
        self.next_row += 1;
    }
}

impl EventHandler for PixelSorting {}

impl Runnable for PixelSorting {
    fn config(&self) -> WindowConfig {
        WindowConfig {
            width: WIDTH,
            height: HEIGHT,
            title: "PixelSorting".to_owned(),
        }
    }
}
