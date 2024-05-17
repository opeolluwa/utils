use crate::partials::footer::AppFooter;
use crate::partials::nav::AppNavigation;
use leptos::{component, view, Children, IntoView};

/// the default application layout, with footer, viewbox, and navigation
#[component]
pub fn Layout(
    /// elements injected into the DOM node
    children: Children,
    /// tailwind/css classes
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    view! {
        <div class="layout__default">
            <AppNavigation/>
            <main class=format!("container my-8 px-4  {class}")>{children()}</main>
            <AppFooter/>
        </div>
    }
}
