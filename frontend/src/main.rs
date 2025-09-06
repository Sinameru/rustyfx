use yew::prelude::*;

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>{"ta tuto bem bunitinha!"}</div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
