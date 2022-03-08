use crc::{crc32, Hasher32};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use quad_rand as qrand;
use web_sys::{CanvasRenderingContext2d};
use numtoa::NumToA;
use std::str;
use std::f64::consts::PI;

use strings::*;

mod strings;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn set_random_values(context: &CanvasRenderingContext2d) {
    let fill_color = get_random_color();

    context.set_fill_style(&JsValue::from_str(fill_color));
    context.set_stroke_style(&JsValue::from_str(fill_color));

    let shadow_color = get_random_color();

    context.set_shadow_color(shadow_color);

    context.set_shadow_offset_x(qrand::gen_range(0., 2.));
    context.set_shadow_offset_y(qrand::gen_range(0., 2.));
    context.set_shadow_blur(qrand::gen_range(0., 2.));
}

fn draw_random_primitive(context: &CanvasRenderingContext2d, height: f64, width: f64) -> Result<(), JsValue> {
    set_random_values(context);
    context.begin_path();

    match qrand::gen_range(0u8, 5u8) {
        // text
        0 => {
            context.set_font("25px Arial Verdana Helvetica");
            context.fill_text(
                get_random_string(20),
                qrand::gen_range(0., width / 2.),
                qrand::gen_range(0., height),
            )?;
        }
        // arc
        1 => {
            context.arc(
                qrand::gen_range(0., width),
                qrand::gen_range(0., height),
                qrand::gen_range(0., width.min(height)),
                qrand::gen_range(0., 2.0 * PI),
                qrand::gen_range(0., 2.0 * PI),
            )?;
        }
        // bezier curve
        2 => {
            context.move_to(
                qrand::gen_range(0., width),
                qrand::gen_range(0., height),
            );
            context.bezier_curve_to(
                qrand::gen_range(0., width),
                qrand::gen_range(0., height),
                qrand::gen_range(0., width),
                qrand::gen_range(0., height),
                qrand::gen_range(0., width),
                qrand::gen_range(0., height),
            );
        }
        // quadratic curve
        3 => {
            context.move_to(
                qrand::gen_range(0., width),
                qrand::gen_range(0., height),
            );
            context.quadratic_curve_to(
                qrand::gen_range(0., width),
                qrand::gen_range(0., height),
                qrand::gen_range(0., width),
                qrand::gen_range(0., height),
            );
        }
        4 => {
            context.ellipse(
                qrand::gen_range(0., width),
                qrand::gen_range(0., height),
                qrand::gen_range(0., width / 2.0),
                qrand::gen_range(0., height / 2.0),
                qrand::gen_range(0., 2.0 * PI),
                qrand::gen_range(0., 2.0 * PI),
                qrand::gen_range(0., 2.0 * PI),
            )?;
        }
        _ => {}
    }

    context.stroke();
    Ok(())
}

#[wasm_bindgen]
pub fn make_fingerprint(seed: u32, rounds: u32, height: u32, width: u32) -> Result<u32, JsValue> {
    qrand::srand(seed as u64);

    let document = web_sys::window()
        .ok_or(JsValue::from_str("window unavailable"))?
        .document()
        .ok_or(JsValue::from_str("document unavailable"))?;

    let canvas = document.create_element("canvas")?;

    canvas.set_attribute("width", format_pixel_value(width))?;
    canvas.set_attribute("height", format_pixel_value(height))?;

    #[cfg(feature = "debug")]
        {
            let body = document.body().ok_or(JsValue::from_str("body unavailable"))?;
            body.append_child(&canvas);
        }
    #[cfg(not(feature = "debug"))]
        canvas.set_attribute("display", "none")?;

    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| JsValue::from_str("canvas html element unavailable"))?;

    let context = canvas
        .get_context("2d")?
        .ok_or(JsValue::from_str("2d canvas context unavailable"))?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .map_err(|_| JsValue::from_str("2d canvas rendering context unavailable"))?;

    let mut result = 0;

    for _ in 0..rounds {
        let mut digest = crc32::Digest::new_with_initial(crc32::IEEE, result);

        draw_random_primitive(&context, height as f64, width as f64)?;

        let data_url = canvas.to_data_url()?;
        digest.write(data_url.as_bytes());
        result = digest.sum32();
    }

    Ok(result)
}
