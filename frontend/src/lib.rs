use zoon::{
    web_sys::{CanvasRenderingContext2d, HtmlCanvasElement},
    *,
};
use crate::canvas::WidthFlagSet;
use crate::canvas::HeightFlagSet;
use zoon::console::log;

// ------ ------
//    States
// ------ ------


fn root() -> impl Element {
    RawHtmlEl::new("div").attr("class", "columns").child(canvas()).child(toolbox())
}

#[wasm_bindgen(start)]
pub fn start() {
    start_app("main", root);
}
































/*
fn canvas() -> impl Element {
    Canvas::new()
        .width(1200)
        .height(800)
        .after_insert(set_canvas_context)
        //.after_remove(|_| remove_canvas_context())
        .update_raw_el(|raw_el| raw_el.event_handler(move |event: events::MouseDown| if_draw(event.offset_x(), event.offset_y())))
        .update_raw_el(|raw_el| raw_el.event_handler(move |event: events::MouseUp| if_draw(event.offset_x(), event.offset_y())))
        .update_raw_el(|raw_el| raw_el.event_handler(move |event: events::MouseMove| on_click_on_canvas(event.offset_x(), event.offset_y())))
}

fn set_canvas_context(canvas: HtmlCanvasElement) {
    let ctx = canvas
        .get_context("2d")
        .unwrap_throw()
        .unwrap_throw()
        .unchecked_into::<CanvasRenderingContext2d>();
    canvas_context().set(Some(SendWrapper::new(ctx)));
    //paint_canvas();
}

fn if_draw(x: i32, y: i32){
    if draw().get(){
        draw().update_mut(|a| *a = false)
    }
    else{
        canvas_context().use_ref(|ctx| {
            if let Some(ctx) = ctx.as_ref() {
                ctx.set_line_width(1.);
                ctx.set_fill_style(&JsValue::from("darkblue"));
                ctx.set_stroke_style(&JsValue::from("darkblue"));
                ctx.begin_path();
                ctx.move_to(x as f64, y as f64);
                ctx.arc(x as f64, y as f64, 0.5, 0.0, 2.0*3.14);
                ctx.fill();

            }
        });
        draw().update_mut(|a| *a = true)
    }
}

fn draw_line(x: i32, y: i32){
    canvas_context().use_ref(|ctx| {
        if let Some(ctx) = ctx.as_ref() {
            ctx.set_line_width(3.);
            ctx.set_fill_style(&JsValue::from("darkblue"));
            ctx.set_stroke_style(&JsValue::from("darkblue"));
            ctx.line_to(x as f64, y as f64);
            ctx.stroke();
            ctx.begin_path();
            ctx.move_to(x as f64, y as f64);
        }
    });
}

fn on_click_on_canvas(x: i32, y: i32){
    if draw().get(){
        draw_line(x,y)
    }
}

fn on_hovered_on_canvas() -> bool{
    true
}
fn toolbox() -> impl Element{
    RawHtmlEl::new("div").attr("class", "column is-2").child(
        RawHtmlEl::new("div").attr("class", "columns").child(tools())
    )
}

fn tools() -> impl Element {
    RawHtmlEl::new("div").attr("class", "column").child(line()).child(rectangle()).child(line()).child(rectangle())
}
fn line() -> impl Element {

    RawHtmlEl::new("div").attr("class", "column fa-solid fa-pen")
    //.s(Padding::all(10))
    //.s(RoundedCorners::new().bottom(30)) // because of iOS
    //.s(Background::new()
    //    .color_signal(hovered_signal.map_bool(|| NamedColor::Green5, || NamedColor::Green2)))
    //.on_hovered_change(move |is_hovered| hovered.set(is_hovered))
    //.label("Change color")
    //.on_press(toggle_color)
}

fn rectangle() -> impl Element {

    RawHtmlEl::new("div").attr("class", "column fa-solid fa-t")
    //.s(Padding::all(10))
    //.s(RoundedCorners::new().bottom(30)) // because of iOS
    //.s(Background::new()
    //    .color_signal(hovered_signal.map_bool(|| NamedColor::Green5, || NamedColor::Green2)))
    //.on_hovered_change(move |is_hovered| hovered.set(is_hovered))
    //.label("Change color")
    //.on_press(toggle_color)
}

fn line_button(cls: &str) -> impl Element {
    //let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    RawHtmlEl::new("div").attr("class", "column box pen")
        //.s(Padding::all(10))
        //.s(RoundedCorners::new().bottom(30)) // because of iOS
        //.s(Background::new()
        //    .color_signal(hovered_signal.map_bool(|| NamedColor::Green5, || NamedColor::Green2)))
        //.on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        //.label("Change color")
        //.on_press(toggle_color)
}


// ------ ------
//     Start
// ------ ------

 */
