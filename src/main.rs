extern crate xkcd_get;
extern crate image;
extern crate reqwest;
extern crate xkcdwall;
extern crate rand;
extern crate wallpaper;
extern crate wallpaper_rs;

use image::imageops;
use tempfile::tempdir;
use std::env;
use xkcd_get::Comic;
use rand::Rng;
use std::process;
use wallpaper_rs::{Desktop, DesktopEnvt};

fn main() {

    println!("xkcdwall: Superimpose an XKCD over your wallpaper.\n");

    let args: Vec<String> = env::args().collect();
    let mut xkcd_num = 1;

    let mut imgbuf: image::RgbImage = image::ImageBuffer::new(1920, 1080);

    for (i, arg) in args.iter().enumerate() {
        // println!("{}", arg);
        
        match arg.as_str() {
            "--random" => {
                xkcd_num = rand::thread_rng().gen_range(1, Comic::latest().unwrap().num + 1);
                println!("[note] Chose random XKCD {}", xkcd_num);
            },
            "--num" => {
                if args.get(i+1) == None {
                    println!("Please provide an XKCD number!");
                    process::exit(1);
                }
                xkcd_num = args[i + 1].parse().expect("Please input a number!");
            },
            "--solidcolor" => {
                let r = args[i+1].parse().unwrap();
                let g = args[i+2].parse().unwrap();
                let b = args[i+3].parse().unwrap();

                let color = image::Rgb([r, g, b]);

                for pix in imgbuf.pixels_mut() {
                    *pix = color
                }
            },
            "--from_image" => {
                imgbuf = image::open(&args[i+1]).expect("Something went wrong opening your image").to_rgb();
            }
            _ => {
                 // println!("Please specify if you want a random XKCD, or a specific XKCD.");
                 // process::exit(1);
            }
        }
    }

    run(imgbuf, xkcd_num)

}


fn run(mut imgbuf: image::RgbImage, xkcd_num: u32) {
    let temp_wall_dir = tempdir().expect("Failed to create temp dir");

    println!("[note] Tempdir is: {}", temp_wall_dir.path().display());

    let mut xkcd_image = xkcdwall::xkcd_image(xkcd_num).unwrap().to_rgb();

    // step 1, check if image will fit.

    if ((imgbuf.dimensions().0 as f32) * 0.6 < xkcd_image.dimensions().0 as f32) || imgbuf.dimensions().1 < xkcd_image.dimensions().1 {
        println!("[note] The XKCD must be resized to fit in the background. Resizing...");

        let mut new_width = imgbuf.dimensions().0 as f32 * 0.6;
        let mut new_height = (new_width as f32 / xkcd_image.dimensions().0 as f32) * xkcd_image.dimensions().1 as f32;

        if (new_height as u32) > imgbuf.dimensions().1 {
            // Not acceptable, scale by height as well.

            println!("[note] Resizing height of XKCD to fit wallpaper...");

            let old_height = new_height;

            new_height = new_height * 0.6;
            new_width = (new_height/old_height) * new_width;
        }

        xkcd_image = imageops::resize(&xkcd_image, new_width as u32, new_height as u32, imageops::FilterType::CatmullRom);

        println!("[note] The XKCD was resized.")
    }

    println!("[note] The image buffer is (now) large enough to hold the xkcd.");
    xkcdwall::xkcd_over_background(&mut imgbuf, xkcd_image);

    let temp_wall_path = temp_wall_dir.path().join("wall.png");
    let temp_wall_path_str = temp_wall_path.to_str().expect("Expected a string for the path, but something happened.");
    imgbuf.save(&temp_wall_path_str).unwrap();

    let wall_setter = DesktopEnvt::new().expect("Tried to set your wallpaper, but it didn't work!");
    wall_setter.set_wallpaper(&temp_wall_path_str).unwrap();
}
