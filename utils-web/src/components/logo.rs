use leptos::{view, component, IntoView};

#[component]
pub fn Logo() -> impl IntoView {
    view! {
        <div class="mb-6 md:mb-0">
            <a href="https://flowbite.com/" class="flex items-center">

                <span class="self-center text-2xl font-semibold whitespace-nowrap">Utils</span>
            </a>
        </div>
    }
}