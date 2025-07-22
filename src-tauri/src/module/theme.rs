use crate::common::theme::ThemeConfig;

pub fn theme_load(theme_config: &ThemeConfig) -> anyhow::Result<()> {
    theme_config.validate()?;
    Ok(())
}
