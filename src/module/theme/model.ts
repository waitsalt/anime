import { notify } from "@kyvg/vue3-notification";
import { defineStore } from "pinia";
import { ref } from "vue";
import { configGet } from "../config/service";

enum ThemeType {
  Auto = "auto",
  Dark = "dark",
  Light = "light",
}

interface Theme {
  // 基础配置
  name: string; // 主题名称
  type: ThemeType; // 主题类型(auto/dark/light)

  // 基础背景色
  base: string; // 主要背景色(如页面主体背景)
  mantle: string; // 次要背景色(比base稍暗，用于卡片、侧边栏等)
  crust: string; // 第三层背景色(最深，用于悬浮元素或强调区域)

  // 表面色 - 用于内容容器
  surface0: string; // 基础表面色(卡片、输入框等)
  surface1: string; // 次级表面色(悬停状态、二级卡片)
  surface2: string; // 三级表面色(激活状态、强调卡片)

  // 覆盖层色 - 用于叠加效果
  overlay0: string; // 弱覆盖色(半透明遮罩、下拉阴影)
  overlay1: string; // 中等覆盖色(模态框背景)
  overlay2: string; // 强调覆盖色(工具提示、弹出框)

  // 文本色
  text: string; // 主要文本颜色(正文内容)
  subtext0: string; // 三级文本(禁用文字、占位符)
  subtext1: string; // 次级文本(副标题、辅助文字)

  // 基本颜色 - 用于强调、状态和装饰
  rosewater: string; // 柔和粉色(装饰性元素)
  flamingo: string; // 亮粉色(警告、特殊标记)
  pink: string; // 标准粉色(强调按钮、图标)
  mauve: string; // 淡紫色(导航、标签)
  red: string; // 错误状态、删除操作
  maroon: string; // 深红色(严重警告)
  peach: string; // 橙色(警告、通知)
  yellow: string; // 黄色(提醒、高亮)
  green: string; // 成功状态、确认操作
  teal: string; // 蓝绿色(完成状态)
  sky: string; // 天蓝色(链接、信息)
  sapphire: string; // 宝石蓝(主要按钮、重要操作)
  blue: string; // 标准蓝色(次要按钮)
  lavender: string; // 淡紫色(特殊状态、装饰)
}

interface ThemeConfig {
  themeType: ThemeType;
  themeDarkDefault: string;
  themeLightDefault: string;
  currentDarkThemeName: string;
  currentLightThemeName: string;
  themeList: Theme[];
  themeDarkNameList: string[];
  themeLightNameList: string[];
}

const themeConfigDefault: ThemeConfig = {
  themeType: ThemeType.Auto,
  themeDarkDefault: "Catppuccin Mocha",
  themeLightDefault: "Catppuccin Latte",
  currentDarkThemeName: "Catppuccin Mocha",
  currentLightThemeName: "Catppuccin Latte",
  themeDarkNameList: ["Catppuccin Mocha"],
  themeLightNameList: ["Catppuccin Latte"],
  themeList: [
    {
      name: "Catppuccin Latte",
      type: ThemeType.Light,
      base: "#eff1f5",
      mantle: "#e6e9ef",
      crust: "#dce0e8",
      surface0: "#ccd0da",
      surface1: "#bcc0cc",
      surface2: "#acb0be",
      overlay0: "#9ca0b0",
      overlay1: "#8c8fa1",
      overlay2: "#7c7f93",
      text: "#4c4f69",
      subtext1: "#5c5f77",
      subtext0: "#6c6f85",
      rosewater: "#dc8a78",
      flamingo: "#dd7878",
      pink: "#ea76cb",
      mauve: "#8839ef",
      red: "#d20f39",
      maroon: "#e64553",
      peach: "#fe640b",
      yellow: "#df8e1d",
      green: "#40a02b",
      teal: "#179299",
      sky: "#04a5e5",
      sapphire: "#209fb5",
      blue: "#1e66f5",
      lavender: "#7287fd",
    },
    {
      name: "Catppuccin Mocha",
      type: ThemeType.Dark,
      base: "#1e1e2e",
      mantle: "#181825",
      crust: "#11111b",
      surface0: "#313244",
      surface1: "#45475a",
      surface2: "#585b70",
      overlay0: "#6c7086",
      overlay1: "#7f849c",
      overlay2: "#9399b2",
      text: "#cdd6f4",
      subtext1: "#bac2de",
      subtext0: "#a6adc8",
      rosewater: "#f5e0dc",
      flamingo: "#f2cdcd",
      pink: "#f5c2e7",
      mauve: "#cba6f7",
      red: "#f38ba8",
      maroon: "#eba0ac",
      peach: "#fab387",
      yellow: "#f9e2af",
      green: "#a6e3a1",
      teal: "#94e2d5",
      sky: "#89dceb",
      sapphire: "#74c7ec",
      blue: "#89b4fa",
      lavender: "#b4befe",
    },
  ],
};

class ThemeConfigManager {
  private config: ThemeConfig;

  constructor(config: ThemeConfig) {
    this.config = config;
  }

  public getConfig(): ThemeConfig {
    return this.config;
  }

  public setConfig(config: ThemeConfig): void {
    this.config = config;

    // 获取当前使用的主题类型
    const currentThemeType = config.themeType;

    let currentThemeName: string;
    switch (currentThemeType) {
      case ThemeType.Auto:
        currentThemeName = config.currentDarkThemeName;
        break;
      case ThemeType.Dark:
        // 应用深色主题
        currentThemeName = config.currentDarkThemeName;
        break;
      case ThemeType.Light:
        // 应用浅色主题
        currentThemeName = config.currentLightThemeName;
        break;
    }

    // 根据主题名获取主题配置
    const theme = config.themeList.find((t) => t.name === currentThemeName);

    if (!theme) {
      notify({
        type: "error",
        title: "配置-主题",
        text: "无法找到选择的主题配置",
      });
      return;
    }

    if (config.themeType !== theme.type) {
      config.themeType = theme.type;
    }

    // 根据主题配置设置根元素的 CSS 变量
    const root = document.documentElement;

    // 设置 CSS 变量
    Object.entries(theme).forEach(([key, value]) => {
      if (key === "name" || key === "type") return;
      root.style.setProperty(`--ctp-${key}`, value);
    });

    // 设置文档主题属性
    root.setAttribute("data-theme", theme.type);
  }
}

const useThemeStore = defineStore("theme", () => {
  const themeConfigManager = ref<ThemeConfigManager>(
    new ThemeConfigManager(themeConfigDefault),
  );

  async function init() {
    try {
      const config = await configGet();
      themeConfigManager.value.setConfig(config.theme);
      console.log(config);
    } catch (error) {
      console.error(error);
    }
  }

  init();

  return { themeConfigManager };
});

export {
  type Theme,
  ThemeType,
  type ThemeConfig,
  ThemeConfigManager,
  useThemeStore,
  themeConfigDefault,
};
