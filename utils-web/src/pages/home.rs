use crate::components::button::Button;
use crate::components::icon::Icon;
use crate::layouts::default::Layout;
use leptos::{component, view, IntoView};
// https://dribbble.com/shots/21248762-Crox-Encryption-Solutions-for-Developers
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Layout class="text-white">
            <header class="h-[60vh] sm:h-[75vh] sm:flex py-20  text-center sm:text-left gap-8 items-center justify-center">
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
                        <Button class=" items-center justify-center text-center ">
                            Read the docs
                        </Button>
                    </div>

                </div>
                <div class="shadow-inner shadow-violet-600 rounded-xl hidden sm:block">
                    <img
                        src="https://ik.imagekit.io/nethbooks/cdn/Screenshot%202024-05-15%20at%2009.47.41_tZQ7Yh4mj.png?updatedAt=1715763698464"
                        alt=""
                    />
                </div>
            </header>
            <section class="sm:flex justify-between items-center">

                <h2
                    class="font-extrabold text-center capitalize text-3xl sm:text-5xl leading-1 mb-2 bg-gradient-to-r from-violet-800 to-[#FCB900]  via-red-500 from-25% inline-block text-transparent bg-clip-text"
                    data-aos="fade-up"
                    data-aos-offset="200"
                    data-aos-delay="50"
                    data-aos-duration="1000"
                    data-aos-easing="ease-in-out"
                    data-aos-mirror="true"
                >
                    Everything about the feature that is featured is said here
                </h2>

            </section>
        </Layout>
    }
}
