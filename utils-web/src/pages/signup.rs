use crate::components::button::Button;
use crate::components::input::Input as UserInput;
use crate::layouts::default::Layout;
use leptos::{component, view, IntoView};
use serde::{Deserialize, Serialize};

#[component]
pub fn SignUp() -> impl IntoView {
    view! {
        <Layout class="py-20 flex items-center juistify-center">
            <div class="flex items-center justify-between">
                <div class="w-1/2 shadow-xl rounded-xl shadow-amber-600   shadow-[4px_20px_20px_27px_#FFEBB5] ">
                    <img src="./public/snapshot.png" alt="login page illustration"/>
                </div>

                <div class="bg-[#101010] rounded-lg">
                    <h2 class="text-4xl font-bold leading-1">Create an account to begin</h2>
                    <p class="leading-2 text-xl mb-4 my-1 text-gray-400">
                        Please povide your email or username
                    </p>
                    <form action="">
                        <div class="mt-8">
                            <label
                                for="email-or-username"
                                class="text-gray-400 font-medium text-xl mb-2 block"
                            >

                                Email or Username
                            </label>
                            <UserInput

                                name="email-or-username"
                                id="#email-or-username"

                                placeholder="email or username"
                            />
                        </div>

                        <div class="my-8">
                            <label
                                for="email-or-username"
                                class="text-gray-400 font-medium text-xl mb-2 block"
                            >

                                Email or Username
                            </label>
                            <UserInput

                                name="email-or-username"
                                id="#email-or-username"

                                placeholder="email or username"
                            />
                        </div>
                        <div>

                            <label
                                for="email-or-username"
                                class="text-gray-400 font-medium text-xl mb-2 block"
                            >

                                Email or Username
                            </label>
                            <UserInput

                                name="email-or-username"
                                id="#email-or-username"

                                placeholder="email or username"
                            />
                        </div>

                        <Button class="bg-violet-700 text-white rounded-lg mt-4 w-full">
                            Submit
                        </Button>
                        <a href="/sign-up" class="text-gray-400 mt-4 flex justify-end text-sm">
                            "Don't have an account?"
                            <span class="text-violet-500 font-medium ml-2"></span>
                            Sign up
                        </a>
                    </form>
                </div>

            </div>
        </Layout>
    }
}
