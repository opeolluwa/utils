use crate::components::button::Button;
use crate::components::input::Input as UserInput;
use crate::layouts::default::Layout;
use leptos::{component, view, IntoView};
use serde::{Deserialize, Serialize};

#[component]
pub fn Login() -> impl IntoView {
    view! {
        <Layout>
            <section class="flex items-center justify-center ">
                <div class="bg-[#101010] rounded-lg">
                    <h2 class="text-4xl font-bold leading-1 sm:text-left">
                        Welcome Back!
                    </h2>

                    <form action="" >
                        <div class="mt-8">
                            <label for="email-or-username" class="text-gray-400 mb-2 block">

                                Email or Username
                            </label>
                            <UserInput
                                name="email-or-username"
                                id="#email-or-username"
                                placeholder="email or username"
                            />
                            <Button class="bg-violet-700 text-white rounded-lg mt-8 w-full">
                                Submit
                            </Button>
                            <a href="/sign-up" class="text-gray-400 mt-2 flex justify-end text-sm">
                                "Don't have an account?"
                                <span class="text-violet-500 font-medium pl-1"></span>
                                Sign up
                            </a>
                        </div>
                    </form>
                </div>

            </section>
        </Layout>
    }
}
