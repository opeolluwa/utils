use leptos::{component, view, IntoView};

#[component]
pub fn Logo(
    /// additional tailwind or custom css classes
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    view! {
        <div class="">
            <a href="/" class=format!("flex items-center {class}")>

                <span class="self-center text-2xl font-semibold whitespace-nowrap">Utils</span>
            </a>
        </div>
    }
}
