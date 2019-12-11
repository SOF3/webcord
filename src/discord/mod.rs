use serenity::prelude::*;

use crate::Secrets;

pub struct Bridge {}

impl Bridge {
    pub(crate) fn try_new(secrets: &Secrets) -> serenity::Result<Self> {
        let mut client = serenity::Client::new(secrets.discord().token(), Handler)?;
        client.start_autosharded()?;
        Ok(Self {})
    }
}

struct Handler;

impl EventHandler for Handler {}
