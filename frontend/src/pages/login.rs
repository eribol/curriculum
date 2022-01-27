use zoon::{Canvas, Element, RawEl, RawHtmlEl, El, Button, UpdateRawEl, Placeholder, TextInput, *};
use zoon::text_input::{InputTypeNumber, InputTypePassword, InputTypeText};
use crate::{get_user, User};
use crate::router::router;

pub fn home() -> impl Element{
    RawHtmlEl::new("div").attr("class", "columns").child(login_form())
}



fn login_form() -> impl Element{
    RawHtmlEl::new("div").attr("class", "column is-12").style("margin-bottom", "1.5rem")
        .child(
            RawHtmlEl::new("p")
                .attr("class", "title is-4")
                .child(
                    RawHtmlEl::new("p")
                        .child("Login")
                        .style("color","black")
                )
        )
        .child(
            RawHtmlEl::new("div").attr("class","field is-grouped")
                .child(
                    RawHtmlEl::new("p").attr("class","control is-expanded")
                        .child(
                            TextInput::new().update_raw_el(|raw| raw.class_signal("input", always(true)))
                                .on_change(set_user_id)
                                .label_hidden("Id")
                                .placeholder(Placeholder::new("Id"))
                                .input_type(InputTypeNumber::default())
                        )
                )
        )
        .child(
            RawHtmlEl::new("div").attr("class","field is-grouped")
                .child(
                    RawHtmlEl::new("p").attr("class","control is-expanded")
                        .child(
                            TextInput::new().update_raw_el(|raw| raw.class_signal("input", always(true)))
                                .on_change(set_user_name)
                                .label_hidden("Name")
                                .placeholder(Placeholder::new("Name"))
                                .input_type(InputTypeText::default())
                        )
                )
        )
        .child(
            RawHtmlEl::new("div")
                .attr("class","field is-grouped")
                .child(
                    RawHtmlEl::new("p").attr("class","control is-expanded")
                        .child(
                            Button::new().update_raw_el(|raw_html_el|
                                {
                                    raw_html_el.class_signal("button", always(true))
                                })
                                .on_press(login)
                                .label("Login")
                        )
                )
        )
}
#[static_ref]
fn user_id() -> &'static Mutable<i32> {
    Mutable::new(0)
}
fn set_user_id(id: String) {
    user_id().set_neq(id.parse::<i32>().unwrap());
    self::println!("{:?}", user_id());
}

#[static_ref]
fn user_name() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

fn set_user_name(name: String) {
    user_name().set_neq(name);
    self::println!("{:?}", user_name());
}

fn login() {
    let user = User{
        id:user_id().get().clone(),
        first_name: user_name().get_cloned()
    };
    get_user().set(Some(user));
    router().go("/")
}