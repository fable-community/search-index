use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
// }

// macro_rules! console_log {
//     ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
// }

#[wasm_bindgen]
pub fn probability(p1: u32, _p2: u32) -> js_sys::Uint8Array {
    console_error_panic_hook::set_once();

    let t: u32 = 350 / (100 / p1);

    let img = image::RgbImage::from_fn(350, 15, |x, _y| {
        if x > t {
            image::Rgb([163, 163, 163])
        } else {
            image::Rgb([255, 255, 255])
        }
    });

    let mut buf = std::io::Cursor::new(Vec::new());

    img.write_to(&mut buf, image::ImageOutputFormat::Jpeg(80))
        .unwrap();

    js_sys::Uint8Array::from(buf.get_ref().clone().as_ref())
}
