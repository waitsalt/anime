<script lang="ts" setup>
import { HiSearch } from "vue-icons-plus/hi";
import { AiFillHome } from "vue-icons-plus/ai";
import { IoSettingsSharp } from "vue-icons-plus/io";
import { useRoute, useRouter } from "vue-router";

const route = useRoute();
const router = useRouter();
</script>

<template>
    <div class="topbarContainer">
        <div class="topBarLeftItemList">
            <p>{{ route.name }}</p>
        </div>
        <div class="topBarRightItemList">
            <div
                @click="router.push('/')"
                class="topbarExploreItem topbarItem"
                :class="{ topbarItemActive: route.name == '探索' }"
            >
                <AiFillHome />
            </div>
            <div
                @click="router.push('/search')"
                class="topbarSearchItem topbarItem"
                :class="{ topbarItemActive: route.name == '搜索' }"
            >
                <HiSearch />
            </div>
            <div
                @click="router.push('/setting')"
                class="topbarSettingItem topbarItem"
                :class="{ topbarItemActive: route.name == '设置' }"
            >
                <IoSettingsSharp />
            </div>
        </div>
    </div>
</template>

<style scoped>
.topbarContainer {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    padding: 0 10px;
}

.topBarLeftItemList {
    font-size: 20px;
    margin-left: 10px;
}

.topBarRightItemList {
    display: flex;
    align-items: center;
    gap: 10px;
}

.topbarItem {
    padding: 5px;
    display: flex;
    align-items: center;
    font-size: 15px;
}

.topbarItem:hover {
    border-radius: 10px;
    background-color: var(--ctp-surface0);
}

.topbarItemActive {
    border-radius: 10px;
    color: var(--ctp-mauve);
}
</style>
