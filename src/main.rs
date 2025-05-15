mod flags;

use std::env;
use std::process::exit;
use image::{GenericImageView, DynamicImage, Rgba, RgbaImage};
use crate::flags::*;

fn apply_flag_overlay_to_image_with_blend(img: DynamicImage, flag_data: Vec<u8>, blend_factor: f32, ) -> RgbaImage {
    let (width, _height) = img.dimensions();

    let mut img_rgba = img.to_rgba8();

    for (x, y, pixel) in img_rgba.enumerate_pixels_mut() {
        let index = (y * width + x) as usize * 4;

        let orig_r = pixel[0] as f32;
        let orig_g = pixel[1] as f32;
        let orig_b = pixel[2] as f32;
        let orig_a = pixel[3] as f32;

        let flag_r = flag_data[index] as f32;
        let flag_g = flag_data[index + 1] as f32;
        let flag_b = flag_data[index + 2] as f32;
        let flag_a = flag_data[index + 3] as f32;

        let blended_r = (orig_r * (1.0 - blend_factor) + flag_r * blend_factor).min(255.0) as u8;
        let blended_g = (orig_g * (1.0 - blend_factor) + flag_g * blend_factor).min(255.0) as u8;
        let blended_b = (orig_b * (1.0 - blend_factor) + flag_b * blend_factor).min(255.0) as u8;
        let blended_a = (orig_a * (1.0 - blend_factor) + flag_a * blend_factor).min(255.0) as u8;

        *pixel = Rgba([blended_r, blended_g, blended_b, blended_a]);
    }

    img_rgba
}

fn save_image(image: RgbaImage, output_path: &str) {
    if let Err(e) = image.save(output_path) {
        eprintln!("Erreur lors de la sauvegarde de l'image : {}", e);
    } else {
        println!("Image sauvegardée avec succès à : {}", output_path);
    }
}

fn main() {

    let img_path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("use: ./trans_filter <img path>");
        exit(1);
    });

    let img = match image::open(img_path) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Erreur lors de la lecture de l'image : {}", e);
            return;
        }
    };
    
    let (width, height) = img.dimensions();
    for x in ALL_FLAGS{
        let flag_data = create_pride_flag_overlay(&x, width, height);

        let result_image = apply_flag_overlay_to_image_with_blend(img.clone(), flag_data, 0.5);
        save_image(result_image, &*format!("img/output_with_flag_{:?}.png", x)); 
    }

}


