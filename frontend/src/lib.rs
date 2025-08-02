use leptos::*;
use reqwasm::http::Request;
use serde::Deserialize;

#[derive(Deserialize)]
struct Message {
    msg: String,
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let message = create_resource(cx, || (), |_| async {
        let response = Request::get("http://localhost:3000/api/hello")
            .send()
            .await
            .unwrap()
            .json::<Message>()
            .await
            .unwrap();
        response.msg
    });

    view! { cx,
        <h1>"Frontend in Rust (Leptos)"</h1>
        <Suspense fallback=move || view! { cx, <p>"Loading..."</p> }>
            {move || message.read(cx).map(|msg| view! { cx, <p>{msg}</p> })}
        </Suspense>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

