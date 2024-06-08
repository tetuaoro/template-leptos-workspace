use crate::error_template::{AppError, ErrorTemplate};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptonic::prelude::*;
use tailwind::*;

pub mod error_template;
mod tailwind;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <Root default_theme=LeptonicTheme::default()>
                <Box style="display: flex; flex-direction: column; align-items: center; padding: 1em; min-height: 100%; min-width: 100%">
                    <main>
                        <Routes>
                            <Route path="" view=HomePage/>
                            <Route path="/tailwind" view=TailwindPage/>
                        </Routes>
                    </main>
                </Box>
            </Root>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <H1>"Welcome to Leptos!"</H1>
        <Button on_click>"Click Me: " {count} ></Button>
        <div>
            <A href="/tailwind">"Go to tailwind page"</A>
        </div>
    }
}
