pub mod barrage;
pub mod info;
pub mod sync;
pub mod video;

pub enum PluginType {
    Barrage,
    Video,
    Info,
    Sync,
}
