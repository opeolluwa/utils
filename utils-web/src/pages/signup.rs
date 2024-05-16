use crate::components::button::Button;
use crate::components::icon::Icon;
use crate::components::input::Input as UserInput;
use crate::layouts::default::Layout;
use leptos::{component, view, IntoView};
use serde::{Deserialize, Serialize};
use crate::lib;

#[component]
pub fn SignUp() -> impl IntoView {
    view! {
        <Layout>
            <section class="grid grid-col-2 items-center justify-center">

                <div class="bg-[#101010] rounded-lg">
                    <h2 class="text-4xl font-bold leading-1">Create an account to begin</h2>

                    <form action="">
                        <div class="mt-8">
                            <label for="" class="text-gray-400   mb-2 block">
                                Email
                            </label>
                            <UserInput r#type="email" name="" id="#" placeholder="email"/>
                        </div>

                        <div class="my-8">
                            <label for="" class="text-gray-400   mb-2 block">
                                Username
                            </label>
                            <UserInput name="" id="#" placeholder="username"/>
                        </div>
                        <div>

                            <label for="" class="text-gray-400   mb-2 block">
                                Full name
                            </label>
                            <UserInput name="" id="#" placeholder="fullname"/>
                        </div>

                        <div class="my-8">
                            <label for="" class="text-gray-400   mb-2 block">
                                Security Question
                                <Icon icon="information-fill" class="pl-2"/>
                            </label>
                            <select class="form-select w-full px-4 py-3 rounded-lg w-full :placeholder:text-gray-400 text-black">

                                {lib::security_questions::security_questions()
                                    .map(|question| {
                                        view! { <option>{question}</option> }
                                    })}

                            </select>
                        </div>

                        <div class="my-8">
                            <label for="" class="text-gray-400   mb-2 block">
                                Security Answer
                            </label>
                            <UserInput name="" id="#" placeholder="security answer"/>
                        </div>

                        <Button class="bg-violet-700 mt-6 text-white rounded-lg w-full">
                            Submit
                        </Button>
                        <a href="/login" class="text-gray-400 mt-4 flex justify-end text-sm">
                            "Already have an account?"
                            <span class="text-violet-500 font-medium pl-1"></span>
                            Login
                        </a>
                    </form>
                </div>

            </section>
        </Layout>
    }
}
