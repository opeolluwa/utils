use crate::components::button::Button;
use crate::components::icon::Icon;
use crate::layouts::default::Layout;
use leptos::{component, view, IntoView};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Layout class="text-white">
            <header class="h-[60vh] sm:h-[75vh] flex my-[200px] gap-8 item-center justify-center">
                <div>
                    <h1 class="text-3xl sm:text-5xl  capitalize font-bold ">
                        Everything that goes here goes here
                    </h1>
                    <p class="leading-1  my-4 py-2 text-gray-500 text-xl ">
                        Lorem ipsum dolor sit amet consectetur, adipisicing elit. Explicabo consectetur facilis, dicta, in magni incidunt esse neque,
                    </p>
                    <div class="flex  gap-2 mx-auto my-4 ">
                        <Button class="bg-violet-800 text-white w-fit flex items-center justify-center text-center rounded-full px-4 ">
                            "Download"
                            <Icon icon="ri-arrow-down-circle-fill" class="text-2xl block ml-2"/>
                        </Button>
                        <Button class="border-2  bg-[#999] border-gray-500 items-center justify-center text-center ">
                            Read the docs
                        </Button>
                    </div>

                </div>
                <div>
                    Lorem ipsum dolor, sit amet consectetur adipisicing elit. Amet nobis repellat nesciunt deserunt blanditiis nihil ut expedita fuga adipisci. Quas, in? Expedita hic earum soluta! Harum, ullam dolorum. Officiis, ipsa.
                    hehehe
                </div>
            </header>
            <section class="sm:flex justify-between items-center sm:my-[100px]">

                <div class="">
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
