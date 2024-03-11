use leptos::*;
use leptos_meta::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let a = 100;
    
    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/blog-post-rust.css"/>
        // sets the document title
        <Title text="Blog-Post"/>
        // Body
        <h1>"Welcome to Blog-Post! "{a}</h1>
    }
}