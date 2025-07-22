import { createMemoryHistory, createRouter } from "vue-router";
import ExploreView from "../view/ExploreView.vue";
import SearchView from "../view/SearchView.vue";
import SettingView from "../view/SettingView.vue";

const routes = [
  { path: "/", redirect: "/explore" },
  { path: "/explore", name: "探索", component: ExploreView },
  { path: "/search", name: "搜索", component: SearchView },
  { path: "/setting", name: "设置", component: SettingView },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

export { router };
