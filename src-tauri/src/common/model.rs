use std::{
    fs,
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use anyhow::Result;

pub struct AutoSave<T: serde::Serialize> {
    data: T,
    path: PathBuf,
}

impl<T: serde::Serialize> AutoSave<T> {
    pub fn new(data: T, path: impl Into<PathBuf>) -> Self {
        Self {
            data,
            path: path.into(),
        }
    }

    fn save(&self) -> Result<()> {
        let content = toml::to_string(&self.data)?;
        fs::write(&self.path, content)?;
        Ok(())
    }
}

impl<T: serde::Serialize> Deref for AutoSave<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: serde::Serialize> DerefMut for AutoSave<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // 每次通过 DerefMut 访问时自动保存
        self.save().expect("Failed to save");
        &mut self.data
    }
}
