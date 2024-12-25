use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};

use crate::backend::greet;

#[component]
pub fn App() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (greet_msg, set_greet_msg) = signal(String::new());

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }
            
            let msg = match greet(&name).await {
                Ok(msg) => msg,
                Err(err) => err.to_string(),
            };
            set_greet_msg.set(msg);
        });
    };

    view! {
        <main class="container">
            <form class="row" on:submit=greet>
                <input
                    id="greet-input"
                    on:input=update_name
                />
                <button type="submit">"Greet"</button>
            </form>
            <p>{ move || greet_msg.get() }</p>
        </main>
    }
}
