#![allow(non_snake_case)]   // Crate name isn't snake case

use yew::prelude::*;
use stylist::{css, {yew::{use_style, Global}}};

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
            display: grid;
            place-items: center;

            p {
                color: white;
            }
        "#
    );

    let globalstyle = css!(
        "background-color: black;"
    );

    html! {
        <>
            <Global css={globalstyle}/>
            <div class={style}>
                <button {onclick}>{ "+1" }</button>
                <p>{ *counter }</p>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}