use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
// }

// macro_rules! console_log {
//     ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
// }

const WHITE: image::Rgba<u8> = image::Rgba([255, 255, 255, 255]);
const GREY: image::Rgba<u8> = image::Rgba([163, 163, 163, 255]);
// const BLANK: image::Rgba<u8> = image::Rgba([0, 0, 0, 0]);

#[wasm_bindgen]
pub fn probability(win: u32) -> js_sys::Uint8Array {
    console_error_panic_hook::set_once();


    let w: u32 = 350 * u32::max(win, 0) / 100;

    let img = image::RgbaImage::from_fn(350, 15, |x, _| {
        if (0..w).contains(&x) {
            WHITE
        } else {
            GREY
        }
    });

    let mut buf = std::io::Cursor::new(Vec::new());

    img.write_to(&mut buf, image::ImageOutputFormat::Png)
        .unwrap();

    js_sys::Uint8Array::from(buf.get_ref().clone().as_ref())
}
