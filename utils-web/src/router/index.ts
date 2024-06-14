import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import LoginView from '../views/LoginView.vue'
import SignupView from '../views/SignupView.vue'
import ResetView from '../views/ResetView.vue'
import DashboardView from '../views/DashboardView.vue'
import DocsView from "../views/AboutView.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView
    },
    {
      path: '/signup',
      name: 'signup',
      component: SignupView
    },
    {
      path: '/login',
      name: 'login',
      component: LoginView
    },
    {
      path: '/reset',
      name: 'reset',
      component: ResetView
    },
    {
      path: '/user',
      name: 'dashboard',
      component: DashboardView
    }
  ]
})

export default router
