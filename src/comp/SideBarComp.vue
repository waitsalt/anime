<script lang="ts" setup>
import { HiSearch } from "vue-icons-plus/hi";
</script>

<template>
    <div class="sideBarContainer">
        <div class="sideBarTop">
            <div class="sideBarSearch sideBarItem">
                <div class="sideBarLogo">
                    <HiSearch />
                </div>
            </div>
        </div>
        <div class="sideBarmiddle">
            <div class="sideBarExplore"></div>
            <div class="sideBarMine"></div>
            <div class="sideBarCache"></div>
        </div>
        <div class="sideBarBtm">
            <div class="sideBarSetting"></div>
        </div>
    </div>
</template>

<style scoped>
.sideBarContainer {
    width: 100%;
    height: 100%;
}

.sideBarItem {
    width: 50px;
    height: 50px;
    display: flex;
    justify-content: center; /* 水平居中 */
    align-items: center; /* 垂直居中 */
}
</style>
