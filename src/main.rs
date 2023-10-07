extern crate serde_json;
use bilinear_interpolation::init_data::init_json::ImageData;

fn main() {
    let image_data = init_json::ImageData::new("../json/image.json").expect("出现错误");
}
