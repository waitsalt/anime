<script lang="ts" setup>
import { HiSearch } from "vue-icons-plus/hi";
import { IoSettingsSharp } from "vue-icons-plus/io";
import { BiSolidUserCheck } from "vue-icons-plus/bi";
</script>

<template>
    <div class="topbarContainer">
        <div class="topBarLeftItemList">
            <p>{{ $route.name }}</p>
        </div>
        <div class="topBarRightItemList">
            <div
                @click="$router.push('/search')"
                class="topbarSearchItem topbarItem"
                :class="{ topbarItemActive: $route.name == '搜索' }"
            >
                <HiSearch />
            </div>
            <div
                @click="$router.push('/setting')"
                class="topbarSettingItem topbarItem"
                :class="{ topbarItemActive: $route.name == '设置' }"
            >
                <IoSettingsSharp />
            </div>
            <div
                @click="$router.push('/login')"
                class="topbarLoginItem topbarItem"
                :class="{ topbarItemActive: $route.name == '登录' }"
            >
                <BiSolidUserCheck />
                <p>登录</p>
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
    border-radius: 50px;
    background-color: var(--ctp-surface0);
}

.topbarItemActive {
    border-radius: 50px;
    color: var(--ctp-mauve);
}
</style>
