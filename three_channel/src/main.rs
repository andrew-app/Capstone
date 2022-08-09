use image::{RgbImage, Rgb};
fn main() {

    let r = image::open("./assets/Image_050_6.jpg").unwrap_or_default();
    let g = image::open("./assets/Image_050_3.jpg").unwrap_or_default();
    let b = image::open("./assets/Image_050_1.jpg").unwrap_or_default();

    let r = r.as_bytes();
    let g = g.as_bytes();
    let b = b.as_bytes();

    

    let mut img = RgbImage::new(500, 500);
    let mut iter = 0;
    for x in 0..500 {
        for y in 0..500 {
            img.put_pixel(y, x, Rgb([r[iter],g[iter],b[iter]]));
            iter += 1;
        }
    }

    
    let format = image::ColorType::Rgb8;
    image::save_buffer("./output/Test_Img.jpg", &img, 500, 500, format).unwrap_or_default();
    
}
