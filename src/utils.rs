use time::{macros::offset, OffsetDateTime};

pub fn now_kst() -> OffsetDateTime {
    OffsetDateTime::now_utc().to_offset(offset!(+9))
}

pub fn is_valid_time(when: OffsetDateTime) -> bool {
    let h = when.hour();

    h >= 18 && h < 22
}
