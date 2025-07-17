<script setup lang="ts">
import CurrentPathComp from "./comp/CurrentPathComp.vue";
import SideBarComp from "./comp/SideBarComp.vue";
import TopBarComp from "./comp/TopBarComp.vue";
import { setCurrentTheme } from "./common/theme";

setCurrentTheme("mocha");
</script>

<template>
    <CurrentPathComp />
    <main class="appContainer">
        <div class="leftAppContainer">
            <SideBarComp />
        </div>
        <div class="rightAppContainer">
            <div class="topAppItem">
                <TopBarComp />
            </div>
            <div class="contentAppItem">
                <RouterView />
            </div>
        </div>
    </main>
</template>

<style scoped>
.appContainer {
    width: 100%;
    height: 100%;
    overflow: hidden;
    display: flex;
    color: var(--ctp-text);
    background-color: var(--ctp-base);
}

.leftAppContainer {
    width: 50px;
    height: 100%;
    background-color: var(--ctp-mantle);
    border-right: 1px solid var(--ctp-surface0);
}

.rightAppContainer {
    flex: 1;
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
    align-items: normal;
}

.topAppItem {
    height: 50px;
    display: flex;
    align-items: center;
    background-color: var(--ctp-crust);
    border-bottom: 1px solid var(--ctp-surface0);
}

.contentAppItem {
    flex: 1;
    overflow: auto;
    background-color: var(--ctp-base);
}
</style>
