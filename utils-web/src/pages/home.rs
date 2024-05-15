use crate::components::button::Button;
use crate::components::card::Card;
use crate::components::icon::Icon;
use crate::layouts::default::Layout;
use leptos::{component, view, IntoView};
use serde::{Deserialize, Serialize};

/// the application features section
#[derive(Serialize, Deserialize, Debug, Default)]
struct Feature {
    /// the heading
    title: &'static str,
    /// feature icon
    icon: &'static str,
    /// the featuresb text
    text: &'static str,
}
/// call the rom methods to create the feature
impl Feature {
    pub fn from(icon: &'static str, title: &'static str, text: &'static str) -> Self {
        Self { title, icon, text }
    }
}

// https://dribbble.com/shots/21248762-Crox-Encryption-Solutions-for-Developers
#[component]
pub fn Home() -> impl IntoView {
    let features: Vec<Feature> = vec![
        Feature::from(
            "database-2-line",
            "Persistence",
            "Distributed storage for important information",
        ),
        Feature::from(
            "train-line",
            "file generator",
            "Ships with configurabele project files generator",
        ),
        Feature::from(
            "lock-password-fill",
            "secure",
            "Built with only memory-safe and fault tollerant APIs",
        ),
        Feature::from(
            "flashlight-line",
            "fast",
            "Built from ground up with Rust and Web assembly for speed",
        ),
    ];
    view! {
        <Layout class="text-white">
            <header class="h-[60vh] sm:h-[75vh] sm:flex py-20  text-center sm:text-left gap-8 items-center justify-center">
                <div>
                    <h1 class="text-3xl sm:text-5xl  capitalize font-bold ">
                        Enhance your developer experience with Utils
                    // Super Charge your developer Journey with utils !
                    </h1>
                    <p class="leading-1  my-4 py-2 text-gray-500 text-xl ">
                        Transform your developer experience with a comprehensive suite of Utilities designed to streamline workflows, boost productivity, and elevate the quality of your projects.
                    </p>
                    <div class="flex  gap-2 mx-auto my-4 ">
                        <Button class="bg-violet-800 text-white w-fit flex items-center justify-center text-center rounded-full px-4 ">
                            "Download"
                            <Icon icon="arrow-down-circle-fill" class="text-2xl block ml-2"/>
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
            <section class="flex flex-col justify-between items-center">

                <h2 class="font-extrabold text-center capitalize text-3xl sm:text-5xl leading-1 mb-2 bg-gradient-to-r from-violet-800 to-[#FCB900]  via-red-500 from-25% inline-block text-transparent bg-clip-text">
                    // data-aos="fade-up"
                    // data-aos-offset="200"
                    // data-aos-delay="50"
                    // data-aos-duration="1000"
                    // data-aos-easing="ease-in-out"
                    // data-aos-mirror="true"
                    Powerful features to enhance your developer exprience
                </h2>
                <p class="text-gray-400 text-xl mt-2 mb-8">
                    Utils is built by developer for  developers
                </p>
                <div class="flex gap-8 items-center mt-8">
                    {features
                        .into_iter()
                        .map(|feature| {
                            view! {
                                <Card class="py-20 flex flex-col justify-start items-start gap-4 px-20 rounded w-1/4 bg-[#101010] cursor-pointer">
                                    <Icon icon=feature.icon class="text-4xl"/>
                                    <h2 class="text-2xl capitalize font-medium">{feature.title}</h2>
                                    <p class="text-gray-500 leading-1 my-1 text-xl :first-letter:capitalize">
                                        {feature.text}
                                    </p>
                                </Card>
                            }
                        })
                        .collect::<Vec<_>>()}

                </div>
            </section>
        </Layout>
    }
}
