use rand::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    sierpinski(&context, [(300.0, 0.0), (0.0, 600.0), (600.0, 600.0)], 5, (10, 200, 20));

    Ok(())
}

fn sierpinski(
    context: &web_sys::CanvasRenderingContext2d,
    points: [(f64, f64); 3],
    depth: u8,
    color: (u8, u8, u8))
{
    let [top, left, right] = points;
    draw_triangle(&context, [top, left, right], color);
    let depth = depth - 1;
    if (depth > 0) {
        let left_midpoint = midpoint(top, left);
        let right_midpoint = midpoint(top, right);
        let bottom_midpoint = midpoint(left, right);
        let next_color = random_color();
        sierpinski(&context, [top, left_midpoint, right_midpoint], depth, next_color);
        sierpinski(&context, [left_midpoint, left, bottom_midpoint], depth, next_color);
        sierpinski(&context, [right_midpoint, bottom_midpoint, right], depth, next_color);
    }
}

fn random_color() -> (u8, u8, u8){
    let mut rng = thread_rng();
    (rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255))
}

fn midpoint(point_a: (f64, f64), point_b: (f64, f64)) -> (f64, f64){
    ((point_a.0 + point_b.0) / 2., (point_a.1 + point_b.1) / 2.)
}

fn draw_triangle(context: &web_sys::CanvasRenderingContext2d, points: [(f64, f64); 3], color: (u8, u8, u8)){
    let color_str = format!("rgb({},{},{})", color.0, color.1, color.2);
    context.set_fill_style(&wasm_bindgen::JsValue::from_str(&color_str));
    let [top, left, right] = points;
    context.move_to(top.0, top.1);
    context.begin_path();
    context.line_to(top.0, top.1);
    context.line_to(left.0, left.1);
    context.line_to(right.0, right.1);
    context.close_path();
    context.stroke();
    context.fill();
}
