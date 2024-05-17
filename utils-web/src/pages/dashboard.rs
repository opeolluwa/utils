use crate::components::card::Card;
use crate::components::logo::Logo;
use crate::components::view::View;
use crate::layouts::dashboard::Layout;
use crate::lib::data::Data;
use leptos::{component, view, IntoView};
use leptos_remix_icon::Icon;

#[component]
pub fn Home() -> impl IntoView {
    let gate1 = Data {
        key: String::from("How to use gate"),
        value: String::from("important date goes here or waht ever"),
        date_added: String::from("Fri, 17 May 2024 16:17:31 +0100"),
        last_updated_at: String::from("Fri, 17 May 2024 16:17:31 +0100"),
    };

    let gate2 = gate1.clone();
    let gate3 = gate1.clone();
    let gate4 = gate1.clone();
    let gate5 = gate1.clone();
 let gate6 = gate1.clone();
  let gate7 = gate1.clone();
    let gates: Vec<Data> = vec![gate1, gate2, gate3, gate4, gate5, gate6, gate7];

    view! {
        <Layout>
            <header class="flex items-center justify-between">
                <Logo/>
                <div class="relative">
                    <Icon icon="notification-4-line" class="text-2xl text-gray-500"/>
                    <span
                        class="w-2 h-2  inline-block bg-red-500 rounded-full sups"
                        style="position:absolute; top:3px; right:3px; padding-top: -12px; padding-left:-20px"
                    ></span>

                </div>
            </header>
            <h1 class="text font-semiold mt-8">"Hello 👋"</h1>
            <h1 class="text-xl font-bold mb-8">Opeolluwa!</h1>

            <View class="my-4">
                <View class="flex gap-2  font-semibold ">
                    <a class="cursor-pointer">Recent</a>
                    <a class="text-gray-500 cursor-pointer">Starred</a>
                </View>

                {gates
                    .into_iter()
                    .map(|entry| {
                        view! {
                            <Card class="bg-violet-50 my-4 first:mt-1 last:mb-1 rounded-xl py-8 cursor-pointer first:bg-yellow-50">
                                <View class="text-sm text-gray-400">{entry.date_added}</View>
                                <View class="mt-6 font-bold capitalize">{entry.key}</View>
                                <View>{entry.value}</View>
                            </Card>
                        }
                    })
                    .collect::<Vec<_>>()}

            </View>
        </Layout>
    }
}

#[component]
pub fn ViewRecord() -> impl IntoView {
    view! { <div>dashboard</div> }
}

#[component]
pub fn UpdateRecord() -> impl IntoView {
    view! { <div>dashboard</div> }
}

#[component]
pub fn StoreData() -> impl IntoView {
    view! { <div>dashboard</div> }
}
