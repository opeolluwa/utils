use leptos::{component, view, IntoView, ReadSignal};

/// the app button component
#[component]
pub fn Button(
    #[prop(default = "")]
    /// additional tailwind or custom css classes
    class: &'static str,
    /// the text content of the button
    context: &'static str, // the text content
) -> impl IntoView {
    view! { <button class=format!("px-4 py-2 rounded {class}")>{context}</button> }
}
