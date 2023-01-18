use yew::prelude::*;
use yew::ServerRenderer;

#[function_component]
fn App() -> Html {
    html! {<div>{"Hello, World!"}</div>}
}

// we use `flavor = "current_thread"` so this snippet can be tested in CI,
// where tests are run in a WASM environment. You likely want to use
// the (default) `multi_thread` favor as:
// #[tokio::main]
#[tokio::main(flavor = "current_thread")]
async fn no_main() {
    let renderer = ServerRenderer::<App>::new();

    let rendered = renderer.render().await;

    // Prints: <div>Hello, World!</div>
    println!("{}", rendered);
}
