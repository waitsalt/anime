import { computed, ref, watchEffect } from "vue";

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

const latte: Theme = {
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
};

const frappe: Theme = {
  name: "Catppuccin Frappé",
  type: ThemeType.Dark,

  base: "#303446",
  mantle: "#292c3c",
  crust: "#232634",

  surface0: "#414559",
  surface1: "#51576d",
  surface2: "#626880",

  overlay0: "#737994",
  overlay1: "#838ba7",
  overlay2: "#949cbb",

  text: "#c6d0f5",
  subtext1: "#b5bfe2",
  subtext0: "#a5adce",

  rosewater: "#f2d5cf",
  flamingo: "#eebebe",
  pink: "#f4b8e4",
  mauve: "#ca9ee6",
  red: "#e78284",
  maroon: "#ea999c",
  peach: "#ef9f76",
  yellow: "#e5c890",
  green: "#a6d189",
  teal: "#81c8be",
  sky: "#99d1db",
  sapphire: "#85c1dc",
  blue: "#8caaee",
  lavender: "#babbf1",
};

const macchiato: Theme = {
  name: "Catppuccin Macchiato",
  type: ThemeType.Dark,

  base: "#24273a",
  mantle: "#1e2030",
  crust: "#181926",

  surface0: "#363a4f",
  surface1: "#494d64",
  surface2: "#5b6078",

  overlay0: "#6e738d",
  overlay1: "#8087a2",
  overlay2: "#939ab7",

  text: "#cad3f5",
  subtext1: "#b8c0e0",
  subtext0: "#a5adcb",

  rosewater: "#f4dbd6",
  flamingo: "#f0c6c6",
  pink: "#f5bde6",
  mauve: "#c6a0f6",
  red: "#ed8796",
  maroon: "#ee99a0",
  peach: "#f5a97f",
  yellow: "#eed49f",
  green: "#a6da95",
  teal: "#8bd5ca",
  sky: "#91d7e3",
  sapphire: "#7dc4e4",
  blue: "#8aadf4",
  lavender: "#b7bdf8",
};

const mocha: Theme = {
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
};

const themeList = {
  latte: latte,
  frappe: frappe,
  macchiato: macchiato,
  mocha: mocha,
};

type ThemeName = keyof typeof themeList;

const currentThemeName = ref<ThemeName>("mocha");
const currentTheme = computed(() => themeList[currentThemeName.value]);

function setCurrentTheme(name: ThemeName) {
  currentThemeName.value = name;
}

// 应用主题到 :root
function applyTheme() {
  const root = document.documentElement;
  const theme = currentTheme.value;

  // 设置 CSS 变量
  Object.entries(theme).forEach(([key, value]) => {
    if (key === "name" || key === "type") return;
    root.style.setProperty(`--ctp-${key}`, value);
  });

  // 设置文档主题属性
  root.setAttribute("data-theme", theme.type);
}

watchEffect(() => {
  applyTheme();
});

export { type Theme, type ThemeName, ThemeType, currentTheme, setCurrentTheme };
