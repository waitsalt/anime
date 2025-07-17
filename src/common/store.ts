import { defineStore } from "pinia";
import { ref } from "vue";
import { type Theme, ThemeType } from "./theme";

interface AppServer {
  baseUrl: string;
  userAgent: string;
  accessToken: string | null;
  refreshToken: string | null;
}

interface AppTheme {
  themeType: ThemeType;
  themeTypeDefault: ThemeType;
  themeDarkDefault: string;
  themeLightDefault: string;

  currentTheme: string;
  themeList: Theme[];
}

const useAppStore = defineStore("app", () => {
  const appServer = ref<AppServer>({
    baseUrl: "https://api.bgm.tv",
    userAgent: "waitsalt/anime",
    accessToken: null,
    refreshToken: null,
  });
  const appTheme = ref<AppTheme>({
    themeType: ThemeType.Dark,
    themeTypeDefault: ThemeType.Dark,
    themeDarkDefault: "mocha",
    themeLightDefault: "latte",

    currentTheme: "mocha",
    themeList: [],
  });

  return { appServer, appTheme };
});

export { useAppStore };
