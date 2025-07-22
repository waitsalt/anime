use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ThemeConfig {
    pub theme_type: ThemeType,
    pub theme_dark_default: String,
    pub theme_light_default: String,
    pub current_dark_theme_name: String,
    pub current_light_theme_name: String,
    pub theme_list: Vec<Theme>,
    pub theme_dark_name_list: Vec<String>,
    pub theme_light_name_list: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum ThemeType {
    Auto,
    Dark,
    Light,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
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
            theme_dark_default: "Catppuccin Mocha".to_string(),
            theme_light_default: "Catppuccin Latte".to_string(),
            current_dark_theme_name: "Catppuccin Mocha".to_string(),
            current_light_theme_name: "Catppuccin Latte".to_string(),
            theme_list: vec![
                Theme {
                    name: "Catppuccin Latte".to_string(),
                    theme_type: ThemeType::Light,
                    base: "#eff1f5".to_string(),
                    mantle: "#e6e9ef".to_string(),
                    crust: "#dce0e8".to_string(),
                    surface0: "#ccd0da".to_string(),
                    surface1: "#bcc0cc".to_string(),
                    surface2: "#acb0be".to_string(),
                    overlay0: "#9ca0b0".to_string(),
                    overlay1: "#8c8fa1".to_string(),
                    overlay2: "#7c7f93".to_string(),
                    text: "#4c4f69".to_string(),
                    subtext1: "#5c5f77".to_string(),
                    subtext0: "#6c6f85".to_string(),
                    rosewater: "#dc8a78".to_string(),
                    flamingo: "#dd7878".to_string(),
                    pink: "#ea76cb".to_string(),
                    mauve: "#8839ef".to_string(),
                    red: "#d20f39".to_string(),
                    maroon: "#e64553".to_string(),
                    peach: "#fe640b".to_string(),
                    yellow: "#df8e1d".to_string(),
                    green: "#40a02b".to_string(),
                    teal: "#179299".to_string(),
                    sky: "#04a5e5".to_string(),
                    sapphire: "#209fb5".to_string(),
                    blue: "#1e66f5".to_string(),
                    lavender: "#7287fd".to_string(),
                },
                Theme {
                    name: "Catppuccin Frappé".to_string(),
                    theme_type: ThemeType::Dark,
                    base: "#303446".to_string(),
                    mantle: "#292c3c".to_string(),
                    crust: "#232634".to_string(),
                    surface0: "#414559".to_string(),
                    surface1: "#51576d".to_string(),
                    surface2: "#626880".to_string(),
                    overlay0: "#737994".to_string(),
                    overlay1: "#838ba7".to_string(),
                    overlay2: "#949cbb".to_string(),
                    text: "#c6d0f5".to_string(),
                    subtext1: "#b5bfe2".to_string(),
                    subtext0: "#a5adce".to_string(),
                    rosewater: "#f2d5cf".to_string(),
                    flamingo: "#eebebe".to_string(),
                    pink: "#f4b8e4".to_string(),
                    mauve: "#ca9ee6".to_string(),
                    red: "#e78284".to_string(),
                    maroon: "#ea999c".to_string(),
                    peach: "#ef9f76".to_string(),
                    yellow: "#e5c890".to_string(),
                    green: "#a6d189".to_string(),
                    teal: "#81c8be".to_string(),
                    sky: "#99d1db".to_string(),
                    sapphire: "#85c1dc".to_string(),
                    blue: "#8caaee".to_string(),
                    lavender: "#babbf1".to_string(),
                },
                Theme {
                    name: "Catppuccin Macchiato".to_string(),
                    theme_type: ThemeType::Dark,
                    base: "#24273a".to_string(),
                    mantle: "#1e2030".to_string(),
                    crust: "#181926".to_string(),
                    surface0: "#363a4f".to_string(),
                    surface1: "#494d64".to_string(),
                    surface2: "#5b6078".to_string(),
                    overlay0: "#6e738d".to_string(),
                    overlay1: "#8087a2".to_string(),
                    overlay2: "#939ab7".to_string(),
                    text: "#cad3f5".to_string(),
                    subtext1: "#b8c0e0".to_string(),
                    subtext0: "#a5adcb".to_string(),
                    rosewater: "#f4dbd6".to_string(),
                    flamingo: "#f0c6c6".to_string(),
                    pink: "#f5bde6".to_string(),
                    mauve: "#c6a0f6".to_string(),
                    red: "#ed8796".to_string(),
                    maroon: "#ee99a0".to_string(),
                    peach: "#f5a97f".to_string(),
                    yellow: "#eed49f".to_string(),
                    green: "#a6da95".to_string(),
                    teal: "#8bd5ca".to_string(),
                    sky: "#91d7e3".to_string(),
                    sapphire: "#7dc4e4".to_string(),
                    blue: "#8aadf4".to_string(),
                    lavender: "#b7bdf8".to_string(),
                },
                Theme {
                    name: "Catppuccin Mocha".to_string(),
                    theme_type: ThemeType::Dark,
                    base: "#1e1e2e".to_string(),
                    mantle: "#181825".to_string(),
                    crust: "#11111b".to_string(),
                    surface0: "#313244".to_string(),
                    surface1: "#45475a".to_string(),
                    surface2: "#585b70".to_string(),
                    overlay0: "#6c7086".to_string(),
                    overlay1: "#7f849c".to_string(),
                    overlay2: "#9399b2".to_string(),
                    text: "#cdd6f4".to_string(),
                    subtext1: "#bac2de".to_string(),
                    subtext0: "#a6adc8".to_string(),
                    rosewater: "#f5e0dc".to_string(),
                    flamingo: "#f2cdcd".to_string(),
                    pink: "#f5c2e7".to_string(),
                    mauve: "#cba6f7".to_string(),
                    red: "#f38ba8".to_string(),
                    maroon: "#eba0ac".to_string(),
                    peach: "#fab387".to_string(),
                    yellow: "#f9e2af".to_string(),
                    green: "#a6e3a1".to_string(),
                    teal: "#94e2d5".to_string(),
                    sky: "#89dceb".to_string(),
                    sapphire: "#74c7ec".to_string(),
                    blue: "#89b4fa".to_string(),
                    lavender: "#b4befe".to_string(),
                },
            ],
            theme_dark_name_list: vec![
                "Catppuccin Frappé".to_string(),
                "Catppuccin Macchiato".to_string(),
                "Catppuccin Mocha".to_string(),
            ],
            theme_light_name_list: vec!["Catppuccin Latte".to_string()],
        }
    }
}

impl ThemeConfig {
    pub fn add_theme(&mut self, theme: Theme) -> Result<()> {
        match theme.theme_type {
            ThemeType::Auto => anyhow::bail!("主题类型不能是 auto"),
            ThemeType::Dark => {
                if self.theme_dark_name_list.contains(&theme.name) {
                    anyhow::bail!("该深色主题的名称已存在")
                } else {
                    self.theme_dark_name_list.push(theme.name.clone());
                    self.theme_list.push(theme);
                }
            }
            ThemeType::Light => {
                if self.theme_light_name_list.contains(&theme.name) {
                    anyhow::bail!("该浅色主题的名称已存在")
                } else {
                    self.theme_light_name_list.push(theme.name.clone());
                    self.theme_list.push(theme);
                }
            }
        }
        Ok(())
    }

    pub fn del_theme(&mut self, theme_name: String) -> Result<()> {
        // 1. 查找主题位置
        let index = self
            .theme_list
            .iter()
            .position(|theme| theme.name == theme_name)
            .ok_or_else(|| anyhow::anyhow!("未找到主题 '{}'", theme_name))?;

        // 2. 提前检查主题类型和数量约束
        let is_dark = self.theme_dark_name_list.contains(&theme_name);
        let is_light = self.theme_light_name_list.contains(&theme_name);

        match (is_dark, is_light) {
            (true, _)
                if self.theme_dark_name_list.len() > 1
                    && self.current_dark_theme_name != theme_name =>
            {
                self.theme_dark_name_list.retain(|n| n != &theme_name);
            }
            (_, true)
                if self.theme_light_name_list.len() > 1
                    && self.current_light_theme_name != theme_name =>
            {
                self.theme_light_name_list.retain(|n| n != &theme_name);
            }
            _ => anyhow::bail!("该主题类型的主题仅一个，或者，该主题正在使用中，不能删除"),
        }

        // 3. 最后删除主题（避免所有权问题）
        self.theme_list.remove(index);
        Ok(())
    }

    pub fn set_theme(&mut self, theme: Theme) -> Result<()> {
        // 1. 查找主题位置
        let index = self
            .theme_list
            .iter()
            .position(|theme| theme.name == theme.name)
            .ok_or_else(|| anyhow::anyhow!("未找到主题 '{}'", theme.name))?;

        self.theme_list.remove(index);

        // 2. 更新主题列表
        self.theme_list.push(theme);

        Ok(())
    }

    pub fn select_theme(&mut self, theme_name: String) -> Result<()> {
        // 1. 查找主题位置
        let _ = self
            .theme_list
            .iter()
            .position(|theme| theme.name == theme_name)
            .ok_or_else(|| anyhow::anyhow!("未找到主题 '{}'", theme_name))?;

        // 2. 提前检查主题类型和数量约束
        let is_dark = self.theme_dark_name_list.contains(&theme_name);
        let is_light = self.theme_light_name_list.contains(&theme_name);

        match (is_dark, is_light) {
            (true, _) => {
                self.current_dark_theme_name = theme_name;
                self.theme_type = ThemeType::Dark;
            }
            (_, true) => {
                self.current_light_theme_name = theme_name;
                self.theme_type = ThemeType::Light;
            }
            _ => anyhow::bail!("该主题类型错误"),
        }
        Ok(())
    }

    /// 验证主题配置的完整性
    pub fn validate(&self) -> Result<(), anyhow::Error> {
        // 1. 检查当前主题是否存在且类型匹配
        self.validate_current_theme(
            &self.current_dark_theme_name,
            ThemeType::Dark,
            "current_dark_theme_name",
        )?;

        self.validate_current_theme(
            &self.current_light_theme_name,
            ThemeType::Light,
            "current_light_theme_name",
        )?;

        // 2. 检查默认主题是否存在
        self.validate_theme_exists(&self.theme_dark_default, "theme_dark_default")?;
        self.validate_theme_exists(&self.theme_light_default, "theme_light_default")?;

        // 3. 验证主题列表和名称列表的一致性
        self.validate_theme_lists()?;

        // 4. 验证名称列表中的主题都存在
        self.validate_name_list(&self.theme_dark_name_list, "theme_dark_name_list")?;
        self.validate_name_list(&self.theme_light_name_list, "theme_light_name_list")?;

        // 5. 确保名称列表没有重叠
        self.validate_no_overlap()?;

        // 6. 确保所有主题都有正确的类型
        self.validate_theme_types()?;

        Ok(())
    }

    fn validate_current_theme(
        &self,
        theme_name: &str,
        expected_type: ThemeType,
        field_name: &str,
    ) -> Result<(), anyhow::Error> {
        let theme = self
            .theme_list
            .iter()
            .find(|t| t.name == theme_name)
            .ok_or_else(|| anyhow::anyhow!("{} '{}' 不存在于主题列表中", field_name, theme_name))?;

        if theme.theme_type != expected_type {
            anyhow::bail!(
                "{} '{}' 的主题类型应该是 {:?}，但实际是 {:?}",
                field_name,
                theme_name,
                expected_type,
                theme.theme_type
            );
        }

        Ok(())
    }

    fn validate_theme_exists(
        &self,
        theme_name: &str,
        field_name: &str,
    ) -> Result<(), anyhow::Error> {
        if !self.theme_list.iter().any(|t| t.name == theme_name) {
            anyhow::bail!("{} '{}' 不存在于主题列表中", field_name, theme_name);
        }
        Ok(())
    }

    fn validate_theme_lists(&self) -> Result<(), anyhow::Error> {
        // 检查主题列表中的每个主题都在对应的名称列表中
        for theme in &self.theme_list {
            match theme.theme_type {
                ThemeType::Dark => {
                    if !self.theme_dark_name_list.contains(&theme.name) {
                        anyhow::bail!("深色主题 '{}' 不在 theme_dark_name_list 中", theme.name);
                    }
                }
                ThemeType::Light => {
                    if !self.theme_light_name_list.contains(&theme.name) {
                        anyhow::bail!("浅色主题 '{}' 不在 theme_light_name_list 中", theme.name);
                    }
                }
                ThemeType::Auto => {
                    anyhow::bail!("主题 '{}' 的类型不能是 Auto", theme.name);
                }
            }
        }
        Ok(())
    }

    fn validate_name_list(
        &self,
        name_list: &[String],
        field_name: &str,
    ) -> Result<(), anyhow::Error> {
        for name in name_list {
            if !self.theme_list.iter().any(|t| &t.name == name) {
                anyhow::bail!("{} 中的主题 '{}' 不存在于主题列表中", field_name, name);
            }
        }
        Ok(())
    }

    fn validate_no_overlap(&self) -> Result<(), anyhow::Error> {
        for name in &self.theme_dark_name_list {
            if self.theme_light_name_list.contains(name) {
                anyhow::bail!("主题 '{}' 同时存在于深色和浅色列表中", name);
            }
        }
        Ok(())
    }

    fn validate_theme_types(&self) -> Result<(), anyhow::Error> {
        for theme in &self.theme_list {
            if theme.theme_type == ThemeType::Auto {
                anyhow::bail!("主题 '{}' 的类型不能是 Auto", theme.name);
            }
        }
        Ok(())
    }
}
