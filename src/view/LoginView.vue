<script lang="ts" setup>
import { openUrl } from "@tauri-apps/plugin-opener";
import { invoke } from "@tauri-apps/api/core";
import { notify } from "@kyvg/vue3-notification";
import { ref } from "vue";

const bangumiAccessTokenGetUrl = "https://next.bgm.tv/demo/access-token/create";

const loginAccessToken = ref("");

const loginLoginClick = async () => {
    // 获取输入框里的值
    const accessToken = loginAccessToken.value;

    // 验证 access_token 的长度
    if (accessToken.length != 40) {
        notify({
            type: "error",
            title: "登录",
            text: "accessToken 错误",
        });

        return;
    }

    // 尝试验证 access_token 的有效性
    const a = await invoke("login", {
        accessToken: accessToken,
    });

    console.log(a);

    notify({
        type: "success",
        title: "登录",
        text: `登录成功`,
    });
};
</script>

<template>
    <div class="loginContainer">
        <div class="loginTitle">登录 Bangumi 账号</div>
        <div class="loginIcon">
            <img src="/src/asset/bg_musume.png" alt="bangumi 娘的图片" />
        </div>
        <div class="loginInfo">
            登录后，可以使用收藏、观看进度管理等额外功能
        </div>
        <!-- <div class="loginBtn">登录/注册</div> -->
        <div class="LoginAccessTokenItem">
            <div
                @click="openUrl(bangumiAccessTokenGetUrl)"
                class="loginAccessTokenGet"
            >
                获取
            </div>
            <input
                type="text"
                class="loginAccessTokenInput"
                placeholder="请填入你的accessToken"
                v-model="loginAccessToken"
            />
            <div class="loginBtn" @click="loginLoginClick()">登录</div>
        </div>
    </div>
</template>

<style lang="css" scoped>
.loginContainer {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
}

.loginTitle {
    font-size: 28px;
    font-weight: 700;
    margin-bottom: 30px;
    text-align: center;
}

.loginIcon {
    width: 300px;
    height: 80px;
    margin-bottom: 25px;
    overflow: hidden;
}

.loginIcon img {
    width: 100%;
    height: 100%;
    object-fit: cover;
}

.loginInfo {
    font-size: 15px;
    color: var(--ctp-subtext0);
    margin-bottom: 40px;
    text-align: center;
}

.LoginAccessTokenItem {
    display: flex;
    width: 100%;
    max-width: 400px;
    gap: 10px;
    align-items: center;
}

.loginAccessTokenGet {
    background-color: var(--ctp-surface0);
    padding: 12px 20px;
    border-radius: 5px;
    font-weight: 500;
    cursor: pointer;
    font-size: 14px;
}

.loginAccessTokenGet:hover {
    background-color: var(--ctp-surface1);
}

.loginAccessTokenInput {
    flex: 1;
    padding: 14px 16px;
    border-radius: 5px;
    border: 1px solid var(--ctp-surface1);
    background-color: var(--ctp-mantle);
    color: var(--ctp-text);
    font-size: 14px;
}

.loginAccessTokenInput:focus {
    outline: none;
    border-color: var(--ctp-blue);
}

.loginBtn {
    background-color: var(--ctp-surface0);
    padding: 12px 20px;
    border-radius: 5px;
    cursor: pointer;
    font-size: 15px;
    transition: all 0.2s ease;
    border: none;
    white-space: nowrap;
}

.loginBtn:hover {
    background-color: var(--ctp-surface1);
}
</style>
