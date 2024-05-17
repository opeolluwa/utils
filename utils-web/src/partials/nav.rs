use crate::components::logo::Logo;
use leptos::{component, view, IntoView};
use leptos_remix_icon::Icon;
use serde::{Deserialize, Serialize};

/// the nav items is going ot be an array of these
#[derive(Debug, Serialize, Deserialize, Default)]
struct NavItem {
    /// the ro=oute path
    path: &'static str,
    /// the route naem
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

/// the nav items is going ot be an array of these
#[derive(Debug, Serialize, Deserialize, Default)]
struct NavItemWithIcon {
    /// the ro=oute path
    path: &'static str,
    /// the route name
    name: &'static str,
    /// the icon class
    icon: &'static str,
    /// alternate icon
    alternate_icon: &'static str,
}

impl NavItemWithIcon {
    pub fn from(
        // the route path
        path: &'static str,
        // the route name
        name: &'static str,
        // the icon class
        icon: &'static str,
        // alternate icon
        alternate_icon: &'static str,
    ) -> Self {
        Self {
            name,
            path,
            icon,
            alternate_icon,
        }
    }
}
/// the buttom nav component
#[component]
pub fn BottomNavigation(
    /// custom css rules or tailwind classes
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    type Nav = NavItemWithIcon;

    let routes: Vec<NavItemWithIcon> = vec![
        Nav::from("home", "home", "home-3-line", "home-3-solid"),
        Nav::from("search", "search", "search-2-line", "serach-2-fill"),
        Nav::from("store", "Store", "add-circle-line", "add-circle-fill"),
        Nav::from("settings", "settings", "settings-3-line", "settings-3-fill"),
        Nav::from("profile", "profile", "user-6-line", "user-6-fill"),
    ];
    view! {
        <ul class="flex items-center justify-between py-2 container gap-4 w-full border-t-[2px] border-t-gray-300">
            {routes
                .into_iter()
                .map(|route| {
                    view! {
                        <li class="capitalize  font-medium ">
                            <a
                                href=format!("/dashboard/{}", route.path)
                                class="flex flex-col items-center gap-col-4 text-gray-500 hover:text-violet-800 transition-all duration-300 hover:bg-violet-100 py-2 px-2 rounded-xl w-[50px] h-[50px] "
                            >
                                <Icon icon=route.icon class="bottom-nav-item text-2xl icon-line"/>
                                <span class="text-sm sr-only">{route.name}</span>
                            </a>
                        </li>
                    }
                })
                .collect::<Vec<_>>()}

        </ul>
    }
}
