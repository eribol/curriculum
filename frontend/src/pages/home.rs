use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use zoon::{Canvas, Element, RawEl, RawHtmlEl, El, *};

pub fn home() -> impl Element{
    RawHtmlEl::new("div").attr("class", "columns").child(canvas()).child(toolbox())
}

#[static_ref]
fn draw()-> &'static Mutable<bool>{
    Mutable::new(false)
}

fn canvas() -> impl Element{
    RawHtmlEl::new("div").attr("class","column is-10").child(
    Canvas::new()
        .width(1900)
        .height(800)
        .after_insert(get_context)
        .update_raw_el(|raw| raw.event_handler(|event: events::MouseDown| {
            canvas_context().use_ref(|ctx| {
                if let Some(ctx) = ctx.as_ref() {
                    ctx.move_to(event.offset_x() as f64, event.offset_y() as f64);
                    //ctx.line_to(150., 60.);
                }
            });
            draw().set(true);
        }))
        .update_raw_el(|raw| raw.event_handler(|event: events::MouseUp| draw().set(false)))
        .update_raw_el(|raw| raw.event_handler(|event: events::MouseMove| if draw().get(){
            ciz(event.offset_x() as f64, event.offset_y() as f64);
        }))
    )
    //<canvas id="paint"></canvas>
}

fn toolbox() -> impl Element{
    RawHtmlEl::new("div").attr("class","column is-2").child("Toolbox")
}

fn get_context(canvas: HtmlCanvasElement){
    let canvas = canvas.get_context("2d")
        .unwrap_throw()
        .unwrap_throw()
        .unchecked_into::<CanvasRenderingContext2d>();
    canvas_context().set(Some(SendWrapper::new(canvas)));
}

#[static_ref]
fn canvas_context()-> &'static Mutable<Option<SendWrapper<CanvasRenderingContext2d>>>{
    Mutable::new(None)
}

fn ciz(x: f64, y: f64){
    canvas_context().use_ref(|ctx| {
        if let Some(ctx) = ctx.as_ref() {
            //ctx.line_to(150., 60.);
            ctx.line_to(x, y);
            ctx.stroke();
        }
    })
}