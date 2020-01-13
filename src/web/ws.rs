#![allow(dead_code)]

use std::time;

use actix::{ActorContext, AsyncContext};
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use crate::{ChannelId, GuildId};

#[actix_web::get("/ws/{guild}")]
async fn handler(
    guild: web::Path<GuildId>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse, actix_web::Error> {
    let guild = guild.into_inner();
    ws::start(
        Socket {
            last_ping: time::Instant::now(),
            guild,
            state: ClientState::Idle,
        },
        &req,
        payload,
    )
}

struct Socket {
    last_ping: time::Instant,
    guild: GuildId,
    state: ClientState,
}

impl actix::Actor for Socket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(time::Duration::from_secs(5), |this, ctx| {
            if this.last_ping.elapsed() > time::Duration::from_secs(10) {
                ctx.stop()
            }
        });
    }
}

impl actix::StreamHandler<Result<ws::Message, ws::ProtocolError>> for Socket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.last_ping = time::Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.last_ping = time::Instant::now();
            }
            Ok(ws::Message::Text(_)) => {
                ctx.text("Unexpected text message");
            }
            Ok(ws::Message::Binary(bytes)) => {
                let _fc: protocol::FromClient = match rmp_serde::decode::from_read_ref(&bytes) {
                    Ok(v) => v,
                    Err(_) => return ctx.stop(),
                };
                unimplemented!("Handle fc")
            }
            _ => {
                ctx.stop();
            }
        };
    }
}

enum ClientState {
    History(ChannelId, (i32, u32, u32)),
    Live(ChannelId),
    Idle,
}

mod protocol {
    use serde::{Deserialize, Serialize};

    use crate::{ChannelId, EmojiId, MessageId};

    #[derive(Deserialize)]
    pub(super) enum FromClient {
        SubLive(SubLive),
        OpenDate(OpenDate),
        SetChannel(SetChannel),
    }

    #[derive(Deserialize)]
    pub(super) struct SubLive {
        pub(super) channel: ChannelId,
    }

    #[derive(Deserialize)]
    pub(super) struct OpenDate {
        pub(super) date: (i32, u32, u32),
    }

    #[derive(Deserialize)]
    pub(super) struct SetChannel {
        pub(super) channel: ChannelId,
    }

    #[derive(Serialize)]
    pub(super) enum FromServer<'t> {
        Alert(Alert<'t>),
        LiveMessage(LiveMessage<'t>),
        EditMessage(EditMessage<'t>),
        SetReactions(SetReactions<'t>),
        DeleteMessage(MessageId),
    }

    #[derive(Serialize)]
    pub(super) struct Alert<'t> {
        pub(super) message: &'t str,
    }

    #[derive(Serialize)]
    pub(super) struct LiveMessage<'t> {
        pub(super) message_id: MessageId,
        pub(super) raw_text: &'t str,
    }

    #[derive(Serialize)]
    pub(super) struct EditMessage<'t> {
        pub(super) message_id: MessageId,
        pub(super) raw_text: &'t str,
    }

    #[derive(Serialize)]
    pub(super) struct SetReactions<'t> {
        pub(super) message_id: MessageId,
        pub(super) reactions: Vec<(Emoji<'t>, u32)>,
    }

    #[derive(Serialize)]
    pub(super) enum Emoji<'t> {
        Unicode(&'t str),
        Custom(EmojiId),
    }
}
