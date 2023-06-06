use futures::{future, StreamExt};
use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use tauri_sys::{tauri::invoke,event::{listen, Event}};

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

// fn listen_to<T : DeserializeOwned + 'static>(event_name : String,action : impl Fn(Event<T>) + 'static) {
//     spawn_local(async move {
//         if let Ok(events) = listen(&event_name).await {
//             events.for_each(|event| {
//                 action(event);
//                 future::ready(())
//             }).await;
//         }
//     })
// }

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, String::new());
    let (greet_msg, set_greet_msg) = create_signal(cx, String::new());

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            if name.get().is_empty() {
                return;
            }

            let args = &GreetArgs { name: &name.get() };
            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            let new_msg = match invoke::<_,(String, Vec<String>)>("current_shift",args).await {
                Ok(m) => m,
                Err(err) => {log!("{:#?}",err); ("".to_string(),vec![])}
            };
            set_greet_msg.set(new_msg.0);
        });
    };

    view! { cx,
        <main class="container">
            <form class="row" on:submit=greet>
                <input
                    id="greet-input"
                    placeholder="Enter a name..."
                    on:input=update_name
                />
                <button type="submit">"Greet"</button>
            </form>

            <p><b>{ move || greet_msg.get() }</b></p>
        </main>
    }
}
