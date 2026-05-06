//! `cron_semantics` — recurring schedule specification.

/// Sentinel for `cron_semantics`.
pub struct CronSemantics;

cast::concept! {
    name: "cron_semantics",
    summary: "Recurring schedule specification. The cron expression is \
              fully resolved against a declared timezone, calendar, and \
              missed-fire policy; \"every Monday\" without those \
              decisions is a footgun in three timezones.",
    anchors: [cast_stdlib::workflow::cron_semantics::CronSemantics],
    tags: ["cast_stdlib", "workflow"],
}
