//! `onboarding_guide` — help new contributor/operator start.

/// Sentinel for `onboarding_guide`.
pub struct OnboardingGuide;

cast::concept! {
    name: "onboarding_guide",
    summary: "Help a new contributor or operator get started. The \
              health of an onboarding guide is a load-bearing signal \
              about whether the team can grow without bottlenecking on \
              one person's memory.",
    anchors: [cast_stdlib::docs::onboarding_guide::OnboardingGuide],
    tags: ["cast_stdlib", "docs"],
}
