use leptos::{component, view, Children, IntoView};

/// the app card component
#[component]
pub fn Card(
    /// additional tailwind or custom css classes
    #[prop(default = "")]
    class: &'static str,
    /// the element that goes into the button
    children: Children,
) -> impl IntoView {
    view! { <div class=format!("px-4 py-2 rounded {class}")>{children()}</div> }
}
