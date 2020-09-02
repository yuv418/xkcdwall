extern crate xkcd_get;
extern crate image;
extern crate reqwest;

use xkcd_get::Comic;
use reqwest::StatusCode;

pub fn xkcd_over_background(wallpaper: &mut image::RgbImage, xkcd_image: image::RgbImage) {
    println!("[note] Writing XKCD to center of wallpaper...");

    let start_x = (wallpaper.dimensions().0 - xkcd_image.dimensions().0) / 2;
    let end_x = wallpaper.dimensions().0 - start_x - 1;

    let start_y = (wallpaper.dimensions().1 - xkcd_image.dimensions().1) / 2;
    let end_y = wallpaper.dimensions().1 - start_y - 1;

        // println!("Are the dimensions the same? {}", xkcd_portion.to_image().dimensions() == xkcd_image.dimensions());

    for (x, y, pix) in wallpaper.enumerate_pixels_mut() {
        if x > start_x && x < end_x {
            if y > start_y && y < end_y {
                let xkcd_x = x - start_x;
                let xkcd_y = y - start_y;

                *pix = *xkcd_image.get_pixel(xkcd_x, xkcd_y);
            }
        }
    }
}

pub fn xkcd_image(number: u32) -> Option<image::DynamicImage> {

    let xkcd_data = Comic::get(number).unwrap();

    if number > 1083 {
        println!("[note] Your XKCD is at least #1084, which means that there may be a 2x (hi-res) version of this comic. Trying... ");

        let xkcd_2x_url = xkcd_data.img.split(".png").collect::<Vec<&str>>()[0].to_string() + "_2x.png";
        let xkcd_2x_response = reqwest::blocking::get(&xkcd_2x_url).expect("Something went wrong when requesting the XKCD image.");

        if xkcd_2x_response.status() == StatusCode::OK {
            // Request a 1x image instead.
            println!("[note] There was a 2x version of your XKCD. Using that instead.");

            let xkcd_image = response_to_image(xkcd_2x_response).expect("Expected an image from the response");

            return Some(xkcd_image);
        }
    }

    let xkcd_image = response_to_image(request_image(&xkcd_data.img)).expect("Expected an image from the response");


    Some(xkcd_image)

}

fn request_image(url: &str) -> reqwest::blocking::Response {
    reqwest::blocking::get(url).expect("Something went wrong when requesting the XKCD image.")
}

fn response_to_image(response: reqwest::blocking::Response) -> Result<image::DynamicImage, Box<dyn std::error::Error>> {
    let bytes = response.bytes()?;
    let bytes_image = image::load_from_memory(&bytes)?;

    Ok(bytes_image)
}
