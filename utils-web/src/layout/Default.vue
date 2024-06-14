<template>
  <header class="px-20">
    <nav class="flex justify-between px-6 pt-8 lg:px-8 items-center">
      <a href="/" class="font-bold indigo-violet-600">Utils</a>
      <div class="hidden lg:flex lg:gap-x-12 capitalize">
        <a :href="route.path" v-for="(route, index) in routes" :key="index">
          {{ route.name }}
        </a>
      </div>
      <div>
        <a href="#">
          <i class="ri-github-fill text-xl"></i>
        </a>
      </div>
    </nav>
  </header>
  <main class="">
    <slot></slot>
  </main>

  <footer class="bg-white rounded-lg shadow m-4">
    <div class="w-full max-w-screen-xl mx-auto p-4 md:py-8">
      <div class="sm:flex sm:items-center sm:justify-between">
        <a
          href="https://flowbite.com/"
          class="flex items-center mb-4 sm:mb-0 space-x-3 rtl:space-x-reverse"
        >
          <img src="https://flowbite.com/docs/images/logo.svg" class="h-8" alt="Flowbite Logo" />
          <span class="self-center text-2xl font-semibold whitespace-nowrap">Utils</span>
        </a>
        <ul class="flex flex-wrap items-center mb-6 text-sm font-medium text-gray-500 sm:mb-0">
          <li>
            <a href="#" class="hover:underline me-4 md:me-6">About</a>
          </li>
          <li>
            <a href="#" class="hover:underline me-4 md:me-6">Privacy Policy</a>
          </li>
          <li>
            <a href="#" class="hover:underline me-4 md:me-6">Licensing</a>
          </li>
          <li>
            <a href="#" class="hover:underline">Contact</a>
          </li>
        </ul>
      </div>
      <hr class="my-6 border-gray-200 sm:mx-auto lg:my-8" />
      <span class="block text-sm text-gray-500 sm:text-center dark:text-gray-400"
        >© 2023 <a href="https://flowbite.com/" class="hover:underline">Flowbite™</a>. All Rights
        Reserved.</span
      >
    </div>
  </footer>
</template>

<script setup lang="ts">
export interface Route {
  path: string
  name: string
}
const routes: Array<Route> = [
  {
    path: '/',
    name: 'home'
  },
  {
    path: '/signup',
    name: 'signup'
  },
  {
    path: '/user',
    name: 'Dashboard'
  }
]
</script>
