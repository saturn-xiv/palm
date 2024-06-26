use std::io::Cursor;

use ab_glyph::{FontRef, PxScale};
use image::{imageops, ImageBuffer, ImageFormat, Rgb, RgbImage};
use imageproc::drawing::{draw_cross_mut, draw_text_mut, text_size};
use rand::{thread_rng, Rng};

use super::Result;

pub const FONT_DEJAVUSANS: &[u8] = include_bytes!("DejaVuSans.ttf");

pub struct Captcha {
    pub text: String,
    pub height: u32,
    pub gap: u32,
}

impl Captcha {
    pub fn png(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        let it = self.build()?;
        it.write_to(&mut Cursor::new(&mut buf), ImageFormat::Png)?;
        Ok(buf)
    }
    fn build(&self) -> Result<RgbImage> {
        let img_w = (self.height * (self.text.len() as u32)) + self.gap * 4;
        let img_h = self.height + self.gap * 4;
        let mut img: RgbImage = ImageBuffer::new(img_w, img_h);

        let (w, h) = {
            // https://www.fontsquirrel.com/fonts/dejavu-sans
            let font = FontRef::try_from_slice(FONT_DEJAVUSANS)?;

            let scale = PxScale {
                x: self.height as f32,
                y: self.height as f32,
            };
            draw_text_mut(
                &mut img,
                Rgb([0u8, 0u8, 255u8]),
                self.gap as i32,
                self.gap as i32,
                scale,
                &font,
                &self.text,
            );
            Self::snowflow(&mut img, img_w, img_h);
            text_size(scale, &font, &self.text)
        };

        let it = imageops::crop(&mut img, 0, 0, w + self.gap * 2, h + self.gap * 2);
        let it = it.to_image();
        Ok(it)
    }

    fn snowflow(img: &mut RgbImage, width: u32, height: u32) {
        let mut rng = thread_rng();

        for _ in 1..(width * height / 64) {
            let rgb = {
                Rgb([
                    rng.gen_range(0..255),
                    rng.gen_range(0..255),
                    rng.gen_range(0..255),
                ])
            };
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            draw_cross_mut(img, rgb, x as i32, y as i32);
        }
    }
}
