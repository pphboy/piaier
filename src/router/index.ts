import { createRouter, createWebHistory } from "vue-router";
import ChatMain from "../views/ChatMain/Index.vue";
import Prompter from "../views/Prompter/Index.vue";
import KeyShut from "../views/KeyShut/Index.vue";
import PopWin from "../views/PopWin/Index.vue";

const route = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      name: "Main",
      redirect: "/Chat"
    },
    {
      path: "/Chat",
      name: "ChatMain",
      component: ChatMain,
    },
    {
      path: "/prompter",
      name: "Prompter",
      component: Prompter,
    },
    {
      path: "/keyShut",
      name: "KeyShut",
      component: KeyShut,
    },
    {
      path: "/popwin",
      name: "PopWin",
      component: PopWin,
    }
  ]
})

export default route