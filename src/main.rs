use yew::prelude::*;
use yewdux::prelude::*;


enum Action {
    SetDimension(i32),
}

#[derive(Clone)]
struct State {
    dimension: i32,
}

impl Reducer for State {
    type Action = Action;

    fn reduce(&mut self, action: Self::Action) -> Changed {
        match action {
            Action::SetDimension(dimension) => {
                self.dimension = dimension;
                true
            }
        }
    }
    fn new() -> Self {
        Self { dimension: 0}
    }
}

mod controls;
use controls::Controls;

mod grid;
use grid::Grid;


type AppDispatch = DispatchProps<ReducerStore<State>>;

struct App {
    dispatch: AppDispatch
}
impl Component for App {
    type Message = ();
    type Properties = AppDispatch;

    fn create(dispatch: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {dispatch}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, dispatch: Self::Properties) -> ShouldRender {
        self.dispatch.neq_assign(dispatch)
    }

    fn view(&self) -> Html {
        let increment = self.dispatch.callback(|_| Action::SetDimension(5));
        html! {
            <div class={"AppContainer"}>
            <button onClick=increment>{"me button"}</button>
              <Controls/>
              <Grid/>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<WithDispatch<App>>();
}
