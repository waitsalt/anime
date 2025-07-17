import { createApp } from "vue";
import App from "./App.vue";
import router from "./common/router";
import "./asset/common.css";

createApp(App).use(router).mount("#app");
