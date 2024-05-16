use crate::components::logo::Logo;
use leptos::{component, view, IntoView};
/// the application footer
#[component]
pub fn AppFooter() -> impl IntoView {
    view! {
        <footer class="container pb-8">

            <footer>
                <div class="mx-auto w-full max-w-screen-xl p-4 py-6 lg:py-8">
                    <div class="md:flex md:justify-between">
                        <Logo class="hidden sm:block"/>
                        <div class="grid grid-cols-2 gap-8 sm:gap-6 sm:grid-cols-3">
                            <div>
                                <h2 class="mb-6 text-sm font-semibold text-gray-300 uppercase text-violet-400 ">
                                    Account
                                </h2>
                                <ul class="text-gray-400 font-medium">
                                    <li class="mb-4">
                                        <a href="/login" class="hover:underline">
                                            Login
                                        </a>
                                    </li>
                                    <li>
                                        <a href="/sign-up" class="hover:underline">
                                            Sign up
                                        </a>
                                    </li>
                                </ul>
                            </div>
                            <div>
                                <h2 class="mb-6 text-sm font-semibold text-gray-300 uppercase text-violet-400 ">
                                    Follow us
                                </h2>
                                <ul class="text-gray-400  font-medium">
                                    <li class="mb-4">
                                        <a
                                            href="https://github.com/themesberg/flowbite"
                                            class="hover:underline "
                                        >
                                            Github
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            href="https://discord.gg/4eeurUVvTy"
                                            class="hover:underline"
                                        >
                                            Discord
                                        </a>
                                    </li>
                                </ul>
                            </div>
                            <div>
                                <h2 class="mb-6 text-sm font-semibold text-gray-300 uppercase text-violet-400 ">
                                    Legal
                                </h2>
                                <ul class="text-gray-400  font-medium">
                                    <li class="mb-4">
                                        <a href="#" class="hover:underline">
                                            Privacy Policy
                                        </a>
                                    </li>
                                    <li>
                                        <a href="#" class="hover:underline">
                                            Terms & Conditions
                                        </a>
                                    </li>
                                </ul>
                            </div>
                        </div>
                    </div>
                </div>
            </footer>

        </footer>
    }
}
