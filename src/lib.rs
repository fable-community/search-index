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
pub fn search(a: String, b: String) -> f32 {
    console_error_panic_hook::set_once();

    // let w: u32 = 350 * u32::max(left, 0) / 100;
    // let d: u32 = 350 * u32::max(damage, 0) / 100;

    // let img = image::RgbaImage::from_fn(350, 15, |x, _| {
    //     if (0..w).contains(&x) {
    //         WHITE
    //     } else if damage > 0 && (w..w + d + 1).contains(&x) {
    //         RED
    //     } else {
    //         GREY
    //     }
    // });

    // let mut buf = std::io::Cursor::new(Vec::new());

    // img.write_to(&mut buf, image::ImageOutputFormat::Png)
    //     .unwrap();

    // js_sys::Uint8Array::from(buf.get_ref().clone().as_ref())

    0.0
}
