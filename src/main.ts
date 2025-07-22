import { createApp } from "vue";
import App from "./App.vue";
import { router } from "./common/router";
import "./asset/common.css";
import { createPinia } from "pinia";

createApp(App).use(router).use(createPinia()).mount("#app");
