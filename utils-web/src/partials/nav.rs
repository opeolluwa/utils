use crate::components::icon::Icon;
use crate::components::logo::Logo;
use leptos::{component, view, IntoView};
use serde::{Deserialize, Serialize};

/// the nav items is going ot be an array of these
#[derive(Debug, Serialize, Deserialize, Default)]
struct NavItem {
    path: &'static str,
    name: &'static str,
}

impl NavItem {
    pub fn from(name: &'static str, path: &'static str) -> Self {
        Self { path, name }
    }
}

#[component]
pub fn AppNavigation() -> impl IntoView {
    let routes: Vec<NavItem> = vec![
        NavItem::from("home", "/"),
        NavItem::from("docs", "/docs"),
        NavItem::from("download", "/download"),
        NavItem::from("login", "/login"),
    ];

    view! {
        <nav class="container flex justify-between items-center pt-8">
            <Logo/>
            <Icon icon="menu-5-line" class="text-2xl block sm:hidden"/>
            <ul class="hidden sm:flex gap-4 items-center">
                {routes
                    .into_iter()
                    .map(|route| {
                        view! {
                            <li class="capitalize text-gray-300 :active:text-black :hover:text-black font-medium :first:text-white">
                                <a href=route.path>{route.name}</a>
                            </li>
                        }
                    })
                    .collect::<Vec<_>>()}

            </ul>
            <div class="flex gap-2 hidden sm:block">
                <a href="https://github.com/opeolluwa/utils">
                    <Icon icon="github-fill" class="text-2xl"/>
                </a>
                <a href="https://github.com/opeolluwa/utils" class="hidden">
                    <Icon icon="discord-fill" class="text-2xl"/>

                </a>
            </div>
        </nav>
    }
}
