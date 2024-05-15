use leptos::{component, view, IntoView};
use serde::{Deserialize, Serialize};

/// the nav otems is going ot be an array of these
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
        NavItem::from("login", "/login"),
    ];

    view! {
        <nav class="container flex justify-between items-center mt-8">
            <img src="app-icon.png" alt="Utils web"/>
            <ul class="flex gap-4 items-center ">
                {routes
                    .into_iter()
                    .map(|route| {
                        view! {
                            <li class="capitalize">
                                <a href=route.path>{route.name}</a>
                            </li>
                        }
                    })
                    .collect::<Vec<_>>()}

            </ul>
        </nav>
    }
}
