//! `change_data_capture` — observe database changes.

/// Sentinel for `change_data_capture`.
pub struct ChangeDataCapture;

cast::concept! {
    name: "change_data_capture",
    summary: "Observe a database's change stream — WAL, binlog, \
              changelog topic — and react. Trades polling cost for a \
              tighter coupling to the source database's \
              implementation; upgrades and replication topology \
              changes become integration concerns.",
    anchors: [cast_stdlib::integration::change_data_capture::ChangeDataCapture],
    tags: ["cast_stdlib", "integration"],
}
