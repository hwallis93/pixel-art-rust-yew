use yew::prelude::*;

#[function_component(Grid)]
pub fn grid() -> Html {
    let text = "I'm the grid!";
    html! { <div>{text}</div> }
}
