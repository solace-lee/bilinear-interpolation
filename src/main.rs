extern crate serde_json;
use bilinear_interpolation::init_data::init_json::ImageData;

fn main() {
    let image_data = ImageData::new("./json/RT_fmt.json").expect("出现错误");
    let scaleW = image_data.dst_width as f32 / image_data.width as f32;
    let scaleH = image_data.dst_height as f32 / image_data.height as f32;

    print!("缩放比例：宽{:#}，高{:#}", scaleW, scaleH)
}
