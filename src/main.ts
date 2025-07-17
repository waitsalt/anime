import { createApp } from "vue";
import App from "./App.vue";
import router from "./common/router";
import "./asset/common.css";
import Notifications from "@kyvg/vue3-notification";
import { createPinia } from "pinia";

createApp(App).use(router).use(createPinia()).use(Notifications).mount("#app");
