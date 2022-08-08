use image::GenericImageView;

fn main() {

    let R = image::open("/home/andrewapp/Documents/Capstone/three_channel/assets/Image_050_6.jpg").unwrap();
    let G = image::open("/home/andrewapp/Documents/Capstone/three_channel/assets/Image_050_3.jpg").unwrap();
    let B = image::open("/home/andrewapp/Documents/Capstone/three_channel/assets/Image_050_1.jpg").unwrap();

    
    let img = [R,G,B];
    image::save_buffer("test.jpg", img, 500, 500, image::ColorType::Rgb8).unwrap()
}
