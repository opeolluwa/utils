use crate::pages::authorize::{ResetAccount, SecurityQuestion};
use crate::pages::dashboard::{Home as Dashboard, StoreData, UpdateRecord, ViewRecord};
use crate::pages::docs::Docs;
use crate::pages::home::Home;
use crate::pages::login::Login;
use crate::pages::mfa::MultiFactorAuthorization;
use crate::pages::signup::SignUp;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/style/output.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=move || view! { <Home/> }/>
                <Route path="/login" view=move || view! { <Login/> }/>
                <Route path="/sign-up" view=move || view! { <SignUp/> }/>
                <Route path="/sescurity-question" view=move || view! { <SecurityQuestion/> }/>
                <Route path="/reset" view=move || view! { <ResetAccount/> }/>
                <Route path="/mfa" view=move || view! { <MultiFactorAuthorization/> }/>
                <Route path="/dashboard" view=move || view! { <Dashboard/> }/>
                <Route path="/docs" view=move || view! { <Docs/> }/>

            </Routes>
        </Router>
    }
}
