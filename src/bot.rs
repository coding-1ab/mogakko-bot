use std::cmp::Ordering;
use std::sync::Arc;

use serenity::all::{
    CommandOptionType, CommandType, CreateCommand, CreateCommandOption, CreateEmbed,
    CreateInteractionResponse, CreateInteractionResponseMessage, Interaction, ResolvedValue,
    UserId,
};
use serenity::builder::CreateEmbedFooter;
use serenity::http::Http;
use serenity::{
    all::{Channel, ChannelType, Ready, VoiceState},
    async_trait,
    prelude::*,
};
use time::{Date, Duration, Month, OffsetDateTime, Weekday};

use crate::db::LeaderboardRecord;
use crate::{db::Db, Config};

pub struct Handler {
    config: Arc<Config>,
    db: Db,
}

impl Handler {
    pub async fn new(config: Arc<Config>) -> anyhow::Result<Self> {
        Ok(Self {
            db: Db::new(config.clone()).await?,
            config,
        })
    }
}

#[async_trait]
impl EventHandler for Handler {
    // Crash recovery.
    async fn ready(&self, ctx: Context, _: Ready) {
        let channel = match ctx.http.get_channel(self.config.vc_id.into()).await {
            Ok(Channel::Guild(v)) => v,
            Ok(_) => {
                eprintln!("Specified channel is not from guild!");
                return;
            }
            Err(e) => {
                eprintln!("Invalid channel id! Error: {:?}", e);
                return;
            }
        };

        if ChannelType::Voice != channel.kind {
            eprintln!("Specified channel is not vc!");
            return;
        }

        channel
            .guild_id
            .create_command(
                ctx.http.clone(),
                CreateCommand::new("leaderboard")
                    .name_localized("ko", "순위표")
                    .description("모각코 순위표 출력")
                    .kind(CommandType::ChatInput),
            )
            .await
            .expect("Unable to create /leaderboard command!");

        channel
            .guild_id
            .create_command(
                ctx.http.clone(),
                CreateCommand::new("table")
                    .name_localized("ko", "점수판")
                    .description("모각코 이벤트 참여자 전체 출력")
                    .kind(CommandType::ChatInput),
            )
            .await
            .expect("Unable to create /table command!");

        channel
            .guild_id
            .create_command(
                ctx.http.clone(),
                CreateCommand::new("statistic")
                    .name_localized("ko", "참여통계")
                    .description("지정된 유저, 또는 자기 자신의 모각코 이벤트 참여 통계 표시")
                    .add_option(
                        CreateCommandOption::new(
                            CommandOptionType::User,
                            "target",
                            "Target to get statistics from",
                        )
                        .description_localized("ko", "통계를 가져올 유저"),
                    )
                    .kind(CommandType::ChatInput),
            )
            .await
            .expect("Unable to create /statistics command!");

        channel
            .guild_id
            .create_command(
                ctx.http.clone(),
                CreateCommand::new("statistic")
                    .name_localized("ko", "참여통계")
                    .kind(CommandType::User),
            )
            .await
            .expect("Unable to create statistics user command!");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let Interaction::Command(interaction) = interaction else {
            return;
        };

        let contents = match interaction.data.name.as_str() {
            "leaderboard" => Bot::leaderboard(ctx.http.clone()).await,
            "table" => Bot::table().await,
            "statistic" => {
                let id = interaction
                    .data
                    .target_id
                    .map(|v| v.get())
                    .or_else(|| {
                        interaction
                            .data
                            .options()
                            .iter()
                            .find(|v| v.name == "target")
                            .map(|v| {
                                let ResolvedValue::User(user, _) = v.value else {
                                    unreachable!()
                                };
                                user.id.get()
                            })
                    })
                    .unwrap_or(interaction.user.id.get());
                Bot::statistics(ctx.http.clone(), id).await
            }
            _ => return,
        };

        let builder = CreateInteractionResponse::Message(contents);
        interaction
            .create_response(ctx.http, builder)
            .await
            .unwrap();
    }

    async fn voice_state_update(&self, _: Context, old: Option<VoiceState>, new: VoiceState) {
        let user_id = new.user_id.get();
        let was_in_vc = old
            .and_then(|v| v.channel_id)
            .map(|v| v.get() == self.config.vc_id.get())
            .unwrap_or(false);
        let now_in_vc = new
            .channel_id
            .map(|v| v.get() == self.config.vc_id.get())
            .unwrap_or(false);

        if !was_in_vc && now_in_vc {
            Db::joins(user_id).await.unwrap();
        }

        if was_in_vc && !now_in_vc {
            Db::leaves(user_id).await.unwrap();
        }
    }
}

pub struct Bot {
    pub client: Client,
}

impl Bot {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        let config = Arc::new(config);

        let handler = Handler::new(config.clone()).await?;

        let client = Client::builder(
            &config.token,
            GatewayIntents::GUILDS
                | GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::MESSAGE_CONTENT
                | GatewayIntents::GUILD_MEMBERS
                | GatewayIntents::GUILD_VOICE_STATES,
        )
        .event_handler(handler)
        .await?;

        Ok(Self { client })
    }

    pub async fn start(&mut self) -> serenity::Result<()> {
        self.client.start().await
    }

    pub async fn leaderboard(client: Arc<Http>) -> CreateInteractionResponseMessage {
        let mut leaderboard: Vec<LeaderboardRecord> = Db::leaderboard().await.unwrap();

        leaderboard.sort_unstable_by(|a, b| {
            let cmp1 = b.days.cmp(&a.days);

            if let Ordering::Equal = cmp1 {
                b.total_duration.cmp(&a.total_duration)
            } else {
                cmp1
            }
        });

        let mut embeds = Vec::new();
        for (idx, record) in leaderboard.into_iter().take(10).enumerate() {
            let user = client.get_user(UserId::new(record.user)).await.unwrap();
            let place = match idx {
                0 => "one",
                1 => "two",
                2 => "three",
                3 => "four",
                4 => "five",
                5 => "six",
                6 => "seven",
                7 => "eight",
                8 => "nine",
                9 => "keycap_ten",
                _ => unreachable!(),
            };

            let color: (u8, u8, u8) = match idx {
                0 => (243, 250, 117),
                1 => (219, 219, 219),
                2 => (135, 62, 35),
                _ => (37, 150, 190),
            };

            let mut title = format!(":{}:등", place);
            if idx == 0 {
                title.push_str("     👑");
            }

            let duration_message = Self::pretty_duration(record.total_duration);

            let mut embed = CreateEmbed::new()
                .title(title)
                .color(color)
                .thumbnail(user.avatar_url().unwrap_or(user.default_avatar_url()))
                .description(format!("<@{}>", record.user))
                .field("출석 일수", record.days.to_string(), true)
                .field("총 개발 시간", duration_message, true);

            let footer_emoji = match idx {
                0 => "🥇",
                1 => "🥈",
                2 => "🥉",
                _ => "",
            };
            if !footer_emoji.is_empty() {
                embed = embed.footer(CreateEmbedFooter::new(footer_emoji));
            }

            embeds.push(embed);
        }

        let message = CreateInteractionResponseMessage::new()
            .add_embeds(embeds)
            .ephemeral(true);

        message
    }

    pub async fn table() -> CreateInteractionResponseMessage {
        let mut leaderboard = Db::leaderboard().await.unwrap();
        leaderboard.sort_unstable_by(|a, b| {
            let cmp1 = b.days.cmp(&a.days);

            if let Ordering::Equal = cmp1 {
                b.total_duration.cmp(&a.total_duration)
            } else {
                cmp1
            }
        });

        let mut line = String::new();
        for (idx, record) in leaderboard.into_iter().enumerate() {
            line.push_str(&format!("{}등: <@{}>\n", idx + 1, record.user));
        }

        CreateInteractionResponseMessage::new()
            .ephemeral(true)
            .content(line)
    }

    pub async fn statistics(client: Arc<Http>, target: u64) -> CreateInteractionResponseMessage {
        let statistics = Db::user_statistics(target).await.unwrap();

        let now = OffsetDateTime::now_utc().date();

        let start = Date::from_calendar_date(now.year(), now.month(), 1).unwrap();
        let end = if let Month::December = now.month() {
            Date::from_calendar_date(now.year() + 1, Month::January, 1)
        } else {
            Date::from_calendar_date(now.year(), now.month().next(), 1)
        }
        .unwrap()
        .previous_day()
        .unwrap();

        let top_left_offset = start.weekday().number_days_from_sunday();
        let bottom_right_offset: u8 = match end.weekday() {
            Weekday::Monday => 5,
            Weekday::Tuesday => 4,
            Weekday::Wednesday => 3,
            Weekday::Thursday => 2,
            Weekday::Friday => 1,
            Weekday::Saturday => 0,
            Weekday::Sunday => 6,
        };

        let user = client.get_user(UserId::new(target)).await.unwrap();

        const BLACK_SQUARE: char = '⬛';
        const WHITE_SQUARE: char = '⬜';
        const RED_SQUARE: char = '🟥';
        const GREEN_SQUARE: char = '🟩';

        let mut description = String::new();
        for _ in 0..top_left_offset {
            description.push(BLACK_SQUARE);
        }

        let days_in_month = end.day();

        let mut cursor = start.clone();
        for day in 1..=days_in_month {
            cursor = cursor.replace_day(day).unwrap();
            let slot = day + top_left_offset - 1;

            let emoji = if now < cursor {
                WHITE_SQUARE
            } else if statistics.calendar.contains(&cursor) {
                GREEN_SQUARE
            } else {
                RED_SQUARE
            };

            description.push(emoji);
            if (slot + 1) % 7 == 0 {
                description.push('\n');
            }
        }

        for _ in 0..bottom_right_offset {
            description.push(BLACK_SQUARE);
        }

        description.push('\n');
        description.push_str("참여 일 수: ");
        description.push_str(statistics.days.to_string().as_str());
        description.push('\n');
        description.push_str("누적 참여 시간: ");
        description.push_str(Self::pretty_duration(statistics.total_duration).as_str());

        let embed = CreateEmbed::new()
            .title(format!("{}님의 모각코 참여 통계", user.name))
            .thumbnail(user.avatar_url().unwrap_or(user.default_avatar_url()))
            .description(description);

        CreateInteractionResponseMessage::new()
            .ephemeral(true)
            .embed(embed)
    }

    fn pretty_duration(duration: Duration) -> String {
        let days = duration.whole_days();
        let hours = duration.whole_hours() % 24;
        let minutes = duration.whole_minutes() % 60;

        let mut duration_message = String::new();
        if days != 0 {
            duration_message.push_str(&days.to_string());
            duration_message.push_str("일 ")
        }

        if hours != 0 {
            duration_message.push_str(&hours.to_string());
            duration_message.push_str("시간 ");
        }

        if minutes != 0 {
            duration_message.push_str(&minutes.to_string());
            duration_message.push_str("분 ");
        }

        duration_message
    }
}
