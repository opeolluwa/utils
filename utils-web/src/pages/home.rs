use leptos::{component, view, IntoView};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="container bg-red">
            <header>
                <div>
                    <h1 class="text-5xl text-center">Everything that goes here goes here</h1>
                    <p>
                        Lorem ipsum dolor sit amet consectetur, adipisicing elit. Explicabo consectetur facilis, dicta, in magni incidunt esse neque, iste rem libero omnis asperiores earum? Eligendi magnam, ratione harum saepe fugiat similique?
                    </p>
                </div>
                <div>
                    <img src="public/default-img.png" alt="utils web"/>
                </div>
            </header>

        </div>
    }
}
