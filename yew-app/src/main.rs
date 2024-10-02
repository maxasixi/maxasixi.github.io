use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h3>{"調整中"}</h3>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}