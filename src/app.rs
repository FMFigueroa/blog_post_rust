use leptos::*;
use leptos_meta::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    
    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/blog-post-rust.css"/>
        // sets the document title
        <Title text="Blog-Post"/>
        // Body
        <main class="my-0 mx-auto max-w-3xl text-center">
        <h2 class="pt-10 text-sky-500 text-4xl font-bold">"Welcome to Leptos with Tailwind"</h2>
        <p class="px-10 pb-10 text-left">"Tailwind will scan your Rust files for Tailwind class names and compile them into a CSS file."</p>
        </main>
        
    }
}