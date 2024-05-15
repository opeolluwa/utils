use crate::components::button::Button;
use crate::layouts::default::Layout;
use leptos::{component, view, IntoView};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Layout>

            <header class="">
                <div>
                    <h1 class="text-5xl text-center">Everything that goes here goes here</h1>
                    <p>
                        Lorem ipsum dolor sit amet consectetur, adipisicing elit. Explicabo consectetur facilis, dicta, in magni incidunt esse neque, iste rem libero omnis asperiores earum? Eligendi magnam, ratione harum saepe fugiat similique?
                    </p>
                </div>
                <div>
                    <img src="public/app-icon.png" alt="utils web"/>
                    <Button context="click me" class="bg-green-500 text-white"/>
                    <Button context="click me"/>

                </div>
            </header>

        </Layout>
    }
}
