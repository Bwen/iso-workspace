import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/Home.vue'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home
  },
  {
    path: '/iso-639',
    name: 'ISO-639',
    component: () => import(/* webpackChunkName: "iso-639" */ '../views/iso-639.vue')
  },
  {
    path: '/iso-3166-1',
    name: 'ISO-3166-1',
    component: () => import(/* webpackChunkName: "iso-3166-1" */ '../views/iso-3166-1.vue')
  },
  {
    path: '/iso-3166-2',
    name: 'ISO-3166-2',
    component: () => import(/* webpackChunkName: "iso-3166-2" */ '../views/iso-3166-2.vue')
  },
  {
    path: '/iso-4217',
    name: 'ISO-4217',
    component: () => import(/* webpackChunkName: "iso-4217" */ '../views/iso-4217.vue')
  }
]

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes
})

export default router
