use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <div class="flex flex-col justify-center items-center">
            <h1 class="flex justify-center text-center bg-orange-500 opacity-70">"Hello World!"</h1>
            <button
                class="flex justify-center text-center bg-purple-500 rounded opacity-70 transition ease-in-out delay-200 hover:bg-blue-400"
                on:click=move |_| {
                    set_count.update(|n| *n += 1);
                }
                class:red=move || count() % 2 == 1
                class:green=move || count() % 2 == 0
            >
                "Click me: "
                {count}
            </button>
        </div>
    }
}
