use yew::prelude::*;
use yew::{function_component, html};

#[function_component(Grid)]
fn grid() -> Html {
    let sum = 2 + 2;
    let text = format!("I'm the grid {}", sum);
    html! { <div>{text}</div> }
}

#[function_component(HelloWorld)]
fn hello_world() -> Html {
    html! { <div><p>{"Hello henry"}</p> <Grid/> </div> }
}

fn main() {
    yew::start_app::<HelloWorld>();
}
