use leptos::{component, view, Children, IntoView};

/// the app button component
#[component]
pub fn Button(
    /// additional tailwind or custom css classes
    class: &'static str,
    /// the element that goes into the button
    children: Children,
) -> impl IntoView {
    view! { <button class=format!("px-4 py-2 rounded {class}")>{children()}</button> }
}
