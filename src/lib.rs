use std::f64;
use std::cell::Cell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// - listen to mouse down:
//   - listen to mouse move:
//     - draw
//   - listen to mouse up:
//     - stop drawing

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let canvas = document.create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    body.append_child(&canvas)?;

    body.style().set_property("background", "#000")?;

    canvas.set_width(640);
    canvas.set_height(480);
    canvas.style().set_property("border", "1px solid #777")?;
    canvas.style().set_property("border-radius", "0.25rem")?;

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let context = Rc::new(context);
    let pressed = Rc::new(Cell::new(false));

    context.set_stroke_style(&"#777".into());
    context.set_fill_style(&"#fff".into());

    // add canvas click listener
    {
        let context = context.clone();
        let listener = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let x = event.offset_x() as f64 - 75.0;
            let y = event.offset_y() as f64 - 75.0;

            context.begin_path();

            // Draw outer circle
            context.arc(x + 75.0, y + 75.0, 50.0, 0.0, f64::consts::PI * 2.0).unwrap();

            // Draw the mouth
            context.move_to(x + 110.0, y + 75.0);
            context.arc(x + 75.0, y + 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

            // Draw the left eye
            context.move_to(x + 65.0, y + 65.0);
            context.arc(x + 60.0, y + 65.0, 5.0, 0.0, f64::consts::PI * 2.0).unwrap();

            // Draw the right eye
            context.move_to(x + 95.0, y + 65.0);
            context.arc(x + 90.0, y + 65.0, 5.0, 0.0, f64::consts::PI * 2.0).unwrap();

            context.stroke();

            pressed.set(true);
        }) as Box<dyn FnMut(_)>);

        canvas.add_event_listener_with_callback("mousedown", listener.as_ref().unchecked_ref())?;
        listener.forget();
    }

    Ok(())
}
