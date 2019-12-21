use std::hash::{Hash, Hasher};

use derive_new::new;
use getset::{CopyGetters, Getters};
use serde::Serialize;
use webcord_schema::models::*;

#[derive(Serialize, CopyGetters, Getters, new)]
pub struct GuildInfo {
    #[get_copy = "pub"]
    id: GuildId,
    #[get = "pub"]
    name: String,
    #[get = "pub"]
    channels: Vec<ChannelInfo>,
}

impl Hash for GuildInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

#[derive(Serialize, CopyGetters, Getters, new)]
pub struct ChannelInfo {
    #[get_copy = "pub"]
    id: ChannelId,
    #[get = "pub"]
    name: String,
    #[get = "pub"]
    description: String,
}

impl Hash for ChannelInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}
