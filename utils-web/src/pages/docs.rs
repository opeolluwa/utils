use crate::components::button::Button;
use crate::components::card::Card;
use crate::components::icon::Icon;
use crate::layouts::default::Layout;
use leptos::{component, view, IntoView};
use serde::{Deserialize, Serialize};

#[component]
pub fn Docs() -> impl IntoView {
    view! {
        <Layout>
            <div>
                Lorem ipsum dolor sit amet consectetur adipisicing elit. Explicabo, aspernatur. Maiores accusantium iusto consectetur, eveniet iste maxime placeat minima odit voluptas alias corrupti, tempore temporibus, molestiae ab aliquid neque sunt!
            </div>

            <div>
                Lorem ipsum dolor sit amet consectetur adipisicing elit. Necessitatibus libero porro aut, accusantium consequatur ab est quisquam, voluptas ad sint temporibus reiciendis non quia, minima sequi excepturi officia omnis praesentium?
            </div>

        </Layout>
    }
}