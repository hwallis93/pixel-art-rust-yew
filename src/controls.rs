use yew::prelude::*;

pub struct Controls;
impl Component for Controls {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        html!{
            <span class={"ControlsContainer"}>
            {"Controls"}
            <span class={"ButtonsContainer"}>
            {"Buttons"}
              </span>
            </span>
        }
    }
}
