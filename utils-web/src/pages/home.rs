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
//  https://dribbble.com/shots/17411928-Account-Creation
// https://dribbble.com/shots/18976667-Website-Sign-In-Page
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
            <header class="h-[75vh] flex py-20  text-center sm:text-left gap-8 items-center justify-center">
                <div>
                    <h1 class="text-3xl sm:text-5xl hidden sm:block  capitalize font-bold ">
                        Enhance your developer experience with Utils
                    </h1>

                    <h1 class="font-extrabold text-center capitalize text-4xl leading-tight mb-2 bg-gradient-to-r from-violet-800 to-[#FCB900]  via-red-500 from-25% inline-block text-transparent bg-clip-text sm:hidden">

                        // data-aos-mirror="true"
                        Enhance your developer experience with Utils
                    </h1>

                    <p class="leading-1  my-4 py-2 text-gray-500 text-xl ">
                        Transform your developer experience with a comprehensive suite designed to streamline workflows and boost productivity.
                    </p>
                    <div class="flex justify-center sm:justify-start sm:items-start items-center gap-2 mx-auto my-4 ">
                        <Button class="bg-violet-800 text-white w-fit flex items-center justify-center text-center px-8 rounded-full px-4 ">
                            "Download"
                            <Icon icon="arrow-down-circle-fill" class="text-2xl block ml-2"/>
                        </Button>
                        <Button class="items-center hidden sm:block justify-center text-center ">
                            Read the docs
                        </Button>
                    </div>

                </div>
                <div class="shadow-inner shadow-violet-600 rounded-xl hidden sm:block ">
                    <img src="./public/snapshot.png" alt=""/>
                </div>
            </header>
            <section class="flex flex-col justify-between items-center">

                <h2
                    class="font-extrabold text-center capitalize text-3xl leading-tight mb-2 block sm:hidden"
                >
                    Powerful features to enhance your developer exprience
                </h2>

                <h2
                    class="font-extrabold text-center capitalize text-5xl sm:text-5xl leading-1 mb-2 bg-gradient-to-r from-violet-800 to-[#FCB900]  via-red-500 from-25% inline-block text-transparent bg-clip-text hidden sm:block w-2/3"

                >
                    Powerful features to enhance your developer exprience
                </h2>

                <p class="text-gray-400 text-center text-xl mt-2 mb-8">
                    Utils is built by developer for  developers
                </p>
                <div class="sm:flex items-center  gap-4 mt-8">
                    {features
                        .into_iter()
                        .map(|feature| {
                            view! {
                                <Card class="py-4 sm:py-20 flex flex-col justify-start items-start px-10 gap-6  rounded sm:w-1/4 bg-[#101010] cursor-pointer">
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
