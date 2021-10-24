use yew::prelude::*;
use yew::{function_component, html};

mod components;
use components::controls::Controls;
use components::grid::Grid;

#[function_component(HelloWorld)]
fn hello_world() -> Html {
    html! {
        <>
          <Grid/>
          <Controls/>
        </>
    }
}

fn main() {
    yew::start_app::<HelloWorld>();
}
