use serenity::all::{ActivityData, ActivityType, Context};
use time::{macros::offset, Duration, OffsetDateTime};

pub fn now_kst() -> OffsetDateTime {
    OffsetDateTime::now_utc().to_offset(offset!(+9))
}

pub fn is_valid_time(when: OffsetDateTime) -> bool {
    let h = when.hour();

    h >= 18 && h < 22
}

pub fn pretty_duration(duration: Duration) -> String {
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

    if duration_message.is_empty() {
        duration_message.push_str("0분")
    }

    duration_message
}

pub fn change_status(ctx: &Context, users: usize) {
    ctx.set_activity(Some(ActivityData {
        name: "Mogakko".to_owned(),
        kind: ActivityType::Custom,
        state: Some(if users != 0 {
            format!("{}명과 모여서 각자 코딩 중...", users)
        } else {
            String::from("모각코 준비중...        ")
        }),
        url: None,
    }));
}
