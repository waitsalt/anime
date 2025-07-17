use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ThemeConfig {
    pub theme_type: ThemeType,
    pub theme_type_default: ThemeType,
    pub theme_dark_default: String,
    pub theme_light_default: String,
    pub current_theme_name: String,
    pub theme_list: Vec<Theme>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ThemeType {
    Auto,
    Dark,
    Light,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Theme {
    // 基础配置
    pub name: String,          // 主题名称
    pub theme_type: ThemeType, // 主题类型(light/dark)

    // 基础背景色
    pub base: String,   // 主要背景色(如页面主体背景)
    pub mantle: String, // 次要背景色(比base稍暗，用于卡片、侧边栏等)
    pub crust: String,  // 第三层背景色(最深，用于悬浮元素或强调区域)

    // 表面色 - 用于内容容器
    pub surface0: String, // 基础表面色(卡片、输入框等)
    pub surface1: String, // 次级表面色(悬停状态、二级卡片)
    pub surface2: String, // 三级表面色(激活状态、强调卡片)

    // 覆盖层色 - 用于叠加效果
    pub overlay0: String, // 弱覆盖色(半透明遮罩、下拉阴影)
    pub overlay1: String, // 中等覆盖色(模态框背景)
    pub overlay2: String, // 强调覆盖色(工具提示、弹出框)

    // 文本色
    pub text: String,     // 主要文本颜色(正文内容)
    pub subtext0: String, // 三级文本(禁用文字、占位符)
    pub subtext1: String, // 次级文本(副标题、辅助文字)

    // 基本颜色 - 用于强调、状态和装饰
    pub rosewater: String, // 柔和粉色(装饰性元素)
    pub flamingo: String,  // 亮粉色(警告、特殊标记)
    pub pink: String,      // 标准粉色(强调按钮、图标)
    pub mauve: String,     // 淡紫色(导航、标签)
    pub red: String,       // 错误状态、删除操作
    pub maroon: String,    // 深红色(严重警告)
    pub peach: String,     // 橙色(警告、通知)
    pub yellow: String,    // 黄色(提醒、高亮)
    pub green: String,     // 成功状态、确认操作
    pub teal: String,      // 蓝绿色(完成状态)
    pub sky: String,       // 天蓝色(链接、信息)
    pub sapphire: String,  // 宝石蓝(主要按钮、重要操作)
    pub blue: String,      // 标准蓝色(次要按钮)
    pub lavender: String,  // 淡紫色(特殊状态、装饰)
}

impl Default for ThemeConfig {
    fn default() -> Self {
        ThemeConfig {
            theme_type: ThemeType::Auto,
            theme_type_default: ThemeType::Dark,
            theme_dark_default: "mocha".to_string(),
            theme_light_default: "lottra".to_string(),
            current_theme_name: "mocha".to_string(),
            theme_list: vec![],
        }
    }
}
