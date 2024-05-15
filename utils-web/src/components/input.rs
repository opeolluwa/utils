use leptos::{component, view, IntoView};

/// app input component
#[component]
pub fn Input(
    /// additional tailwind or custom css classes
    #[prop(default = "")]
    class: &'static str,
    /// input type
    #[prop(default = "text")]
    r#type: &'static str,
    /// placeholder
    #[prop(default = "")]
    placeholder: &'static str,
    //// the id
    #[prop(default = "")] id: &'static str,
    #[prop(default = "")] name: &'static str,
) -> impl IntoView {
    view! {
        <input
            type=r#type
            name=name
            id=id
            class=format!(
                "form-input px-4 py-3 rounded-lg w-full :placeholder:text-gray-400 text-black {class}",
            )

            placeholder=placeholder
        />
    }
}
