<script lang="ts" setup>
import { HiSearch } from "vue-icons-plus/hi";
import { useRoute, useRouter } from "vue-router";

const route = useRoute();
const router = useRouter();
</script>

<template>
    <div class="sideBarContainer">
        <div class="sideBarTop">
            <div
                @click="router.push('/search')"
                class="commonClick sideBarSearch sideBarItem"
                :class="{ commonClickActive: route.name == '搜索' }"
            >
                <HiSearch />
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
    padding: 5px;
    width: 100%;
    height: 100%;
}

.sideBarTop {
    /* margin-top: 10px; */
}
</style>
