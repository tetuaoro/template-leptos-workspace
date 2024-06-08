use leptos::*;
use leptos_router::*;

#[component]
pub fn TailwindPage() -> impl IntoView {
    let (is_true, _set_is_true) = create_signal(true);
    let (count, set_count) = create_signal(0);
    let pair_or_not = move || count.get() % 2 == 1;

    view! {
        <main class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"Welcome to Leptos with Tailwind"</h2>
            <p
                class="px-10 pb-10 text-left"
                // let (is_true, _set_is_true) = create_signal(true);
                class=("text-red-400", is_true)
                // let pair_or_not = move || count.get() % 2 == 1;
                class=("text-red-400/20", pair_or_not)
                class=("dark:text-blue-600", pair_or_not)
            >

                "Tailwind will scan your Rust files for Tailwind class names and compile them into a CSS file."
            </p>
            <button
                class="bg-amber-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg"
                on:click=move |_| set_count.update(|count| *count += 1)
            >
                "Something's here | "
                {move || {
                    if count.get() == 0 { "Click me!".to_string() } else { count.get().to_string() }
                }}

                " | Some more text"
            </button>
            <div>
                <A href="/">"Back home"</A>
            </div>
        </main>
    }
}
