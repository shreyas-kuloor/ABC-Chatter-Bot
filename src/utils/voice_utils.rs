use serenity::{model::prelude::GuildId, prelude::Context};

pub struct TrackEndNotifier {
    pub guild_id: GuildId,
    pub ctx: Context,
}