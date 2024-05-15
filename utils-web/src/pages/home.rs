use crate::components::button::Button;
use crate::components::icon::Icon;
use crate::layouts::default::Layout;
use leptos::{component, view, IntoView};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Layout class="">
            <header class="h-[75vh] flex flex-col item-center justify-center">
                <h1 class="text-5xl text-center capitalize font-medium ">
                    One Stop toolbox for every developer
                </h1>
                <p class="leading-2 w-2/3 mx-auto mt-2 py-2 text-center">
                    Lorem ipsum dolor sit amet consectetur, adipisicing elit. Explicabo consectetur facilis, dicta, in magni incidunt esse neque, iste rem libero omnis asperiores earum? Eligendi magnam, ratione harum saepe fugiat similique?
                </p>
                <div class="flex items-center w-fit gap-2 mx-auto my-4 ">
                    <Button class="bg-[#101010] text-white w-fit flex items-center justify-center text-center">
                        "Download"
                        <Icon icon="ri-arrow-down-circle-fill" class="text-2xl block ml-2"/>
                    </Button>
                    <Button class="border-2  border-gray-500 items-center justify-center text-center  bg-gray-50 ">
                        Read the docs
                    </Button>

                </div>
            </header>
            <section class="flex justify-between items-center my-[100px]">

                <div class="w-1/2">
                    <h2 class="font-medium text-3xl leading-3 mb-2">Some heading</h2>
                    <p class="leading-2 my-4">
                        Lorem ipsum, dolor sit amet consectetur adipisicing elit. Temporibus labore beatae eum sed ad natus perspiciatis. Facere molestias incidunt molestiae maxime perferendis consequatur ullam tenetur! Mollitia porro numquam voluptatibus aut?
                    </p>
                     <Button class="border-2  border-gray-500 items-center justify-center text-center  bg-gray-50 ">
                    Get Started
                </Button>
                </div>
               <div>
               <img src="./placeholder.jpeg" alt="./utils-featrures"/>
               </div>
            </section>
        </Layout>
    }
}
