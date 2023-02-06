#![allow(non_snake_case)]   // Crate name isn't snake case

use yew::prelude::*;
use stylist::yew::use_style;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    let style = use_style!(
        r#"
            background-color: black;

            p {
                color: white;
            }
        "#
    );

    html! {
        <div class={style}>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}