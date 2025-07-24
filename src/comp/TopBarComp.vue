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
                class="topbarExploreItem commonClick"
                :class="{ commonClickActive: route.name == '探索' }"
            >
                <AiFillHome />
            </div>
            <div
                @click="router.push('/search')"
                class="topbarSearchItem commonClick"
                :class="{ commonClickActive: route.name == '搜索' }"
            >
                <HiSearch />
            </div>
            <div
                @click="router.push('/setting')"
                class="topbarSettingItem commonClick"
                :class="{ commonClickActive: route.name == '设置' }"
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
    padding: 10px;
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
</style>
