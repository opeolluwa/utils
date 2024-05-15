use leptos::{component, view, IntoView, ReadSignal};

/// the application footer
#[component]
pub fn AppFooter() -> impl IntoView {
    view! { <footer class="container bg-red-500">"Built With Love  lorem"</footer> }
}
