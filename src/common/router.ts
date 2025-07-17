import { createMemoryHistory, createRouter } from "vue-router";
import LoginView from "../view/LoginView.vue";
import ExploreView from "../view/ExploreView.vue";
import SearchView from "../view/SearchView.vue";
import SettingView from "../view/SettingView.vue";

const routes = [
  { path: "/", redirect: "/explore" },
  { path: "/login", name: "登录", component: LoginView },
  { path: "/explore", name: "探索", component: ExploreView },
  { path: "/search", name: "搜索", component: SearchView },
  { path: "/setting", name: "设置", component: SettingView },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

export default router;
