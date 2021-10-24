use yew::prelude::*;

#[function_component(Controls)]
pub fn controls() -> Html {
    let text = "I'm the controls!";
    html! {<div> {text} </div>}
}
