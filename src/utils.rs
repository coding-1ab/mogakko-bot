use time::{OffsetDateTime, UtcOffset};

pub fn now_kst() -> OffsetDateTime {
    OffsetDateTime::now_utc().to_offset(offset!(+9))
}

pub fn is_in_target_range(when: OffsetDateTime) -> bool {
    let h = when.hour();

    h >= 18 && h < 22
}
