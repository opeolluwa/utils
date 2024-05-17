use leptos::{component, view, Children, IntoView};

/// the app card component
#[component]
pub fn View(
    /// additional tailwind or custom css classes
    #[prop(default = "")]
    class: &'static str,
    /// the element that goes into the button
    children: Children,
) -> impl IntoView {
    view! { <div class=format!("{class}")>{children()}</div> }
}
