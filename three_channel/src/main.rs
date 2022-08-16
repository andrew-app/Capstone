use image::{RgbImage, Rgb};
use std::fs;


fn main() {


    let mut r_ch: Vec<_> = fs::read_dir("./assets/700nm_1ch").unwrap().map(|r| r.unwrap()).collect();
    r_ch.sort_by_key(|dir| dir.path());

    
    let mut g_ch: Vec<_> = fs::read_dir("./assets/550nm_1ch").unwrap().map(|r| r.unwrap()).collect();
    g_ch.sort_by_key(|dir| dir.path());

    let mut b_ch: Vec<_> = fs::read_dir("./assets/450nm_1ch").unwrap().map(|r| r.unwrap()).collect();
    b_ch.sort_by_key(|dir| dir.path());
    
    for i in 0..35 {
        let r = image::open(r_ch[i].path()).unwrap_or_default();
        let g = image::open(g_ch[i].path()).unwrap_or_default();
        let b = image::open(b_ch[i].path()).unwrap_or_default();

        let r = r.as_bytes();
        let g = g.as_bytes();
        let b = b.as_bytes();

        let mut img = RgbImage::new(500, 500);
        let mut px = 0;
        for x in 0..500 {
            for y in 0..500 {
                img.put_pixel(y, x, Rgb([r[px],g[px],b[px]]));
                px += 1;
            }
        }

    
        let format = image::ColorType::Rgb8;
        
        let mut file = format!("./output/IMG_0{}.jpg", i);

        if i < 9 {
            file = format!("./output/IMG_00{}.jpg", i);
        }
        
        image::save_buffer(file, &img, 500, 500, format).unwrap_or_default();

    }
    
    
}
