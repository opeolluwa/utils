use leptos::{component, view, IntoView};

/// the app icon component, built on remx icon
#[component]
pub fn Icon(
    #[prop(default = "")]
    /// the remix icon identifier
    icon: &'static str,
    /// tailwind classes
    #[prop(default = "")]
    /// the remix icon identifier
    class: &'static str,
) -> impl IntoView {
    view! { <i class=format!("{icon} {class}")></i> }
}
