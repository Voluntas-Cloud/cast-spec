//! `classifier_dispatch` — categorize an input then route to the per-category handler.

/// Sentinel for `classifier_dispatch`.
pub struct ClassifierDispatch;

cast::concept! {
    name: "classifier_dispatch",
    summary: "A two-step pipeline: a `classify` function maps each \
              input to one of N kinds, then a per-kind handler \
              processes it. Keeping the classifier separate from \
              the handlers makes the decision auditable (you can \
              ask 'why did this input land in branch X?' as a \
              standalone question) and keeps each handler focused \
              on one kind. The classifier is the policy; handlers \
              are the mechanism.",
    anchors: [cast_stdlib::architecture::classifier_dispatch::ClassifierDispatch],
    tags: ["cast_stdlib", "architecture"],
}
