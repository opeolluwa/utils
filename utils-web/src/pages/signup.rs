use crate::components::button::Button;
use crate::components::input::Input as UserInput;
use crate::layouts::default::Layout;
use leptos::{component, view, IntoView};
use serde::{Deserialize, Serialize};
use crate::lib;

#[component]
pub fn SignUp() -> impl IntoView {
    view! {
        <Layout class="py-20 flex items-center juistify-center">
            <div class="flex items-center justify-between">
                <div class="w-1/2 shadow-xl rounded-xl shadow-amber-600   shadow-[4px_20px_20px_27px_#FFEBB5] hidden  sm:block ">
                    <img src="./public/snapshot.png" alt="login page illustration"/>
                </div>

                <div class="bg-[#101010] rounded-lg">
                    <h2 class="text-4xl font-bold leading-1">Create an account to begin</h2>

                    <form action="">
                        <div class="mt-8">
                            <label
                                for=""
                                class="text-gray-400  text-xl mb-2 block"
                            >

                                Email
                            </label>
                            <UserInput
                                r#type="email"
                                name=""
                                id="#"
                                placeholder="email"
                            />
                        </div>

                        <div class="my-8">
                            <label
                                for=""
                                class="text-gray-400  text-xl mb-2 block"
                            >

                                Username
                            </label>
                            <UserInput
                                name=""
                                id="#"
                                placeholder="username"
                            />
                        </div>
                        <div>

                            <label
                                for=""
                                class="text-gray-400  text-xl mb-2 block"
                            >

                                Fullname
                            </label>
                            <UserInput
                                name=""
                                id="#"
                                placeholder="fullname"
                            />
                        </div>

                        <div class="my-8">

                            <label
                                for=""
                                class="text-gray-400  text-xl mb-2 block"
                            >
                                Security Question
                            </label>
                            <select class="form-select w-full px-4 py-3 rounded-lg w-full :placeholder:text-gray-400 text-black">
                            {
                                lib::security_questions::security_questions().map(|question|{
                                    view!{
                                        <option> {question} </option>
                                    }
                                })
                            }
                            </select>
                        </div>

                        <div class="my-8">
                            <label
                                for=""
                                class="text-gray-400  text-xl mb-2 block"
                            >
                                Security Answer
                            </label>
                            <UserInput
                                name=""
                                id="#"
                                placeholder="security answer"
                            />
                        </div>

                        <Button class="bg-violet-700 text-white rounded-lg w-full">Submit</Button>
                        <a href="/login" class="text-gray-400 mt-4 flex justify-end text-sm">
                            "Already have an account?"
                            <span class="text-violet-500 font-medium pl-1"></span>
                            Login
                        </a>
                    </form>
                </div>

            </div>
        </Layout>
    }
}
