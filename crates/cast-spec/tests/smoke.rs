//! Tokeniser smoke test. Confirms that the grammar in `GRAMMAR.md`
//! tokenises as valid Rust — necessary because the macros expand to
//! nothing, so this is the compile-time check that complements the
//! extractor's runtime parse.
//!
//! Identifiers in this file are intentionally fictional (`sample::*`,
//! `sample_agent::*`). The test exercises the parser, not anchor
//! resolution; cast-extract will report these paths as unresolved when
//! it walks the test target, which is correct and harmless.

#[test]
fn all_macros_parse() {
    cast::compare! {
        intent_id   @ sample::types::IntentId:
            "stable identity, used for traceability across logs",
        intent_hash @ sample::types::IntentHash:
            "replay-guard key, bound to exact server-signed bytes",
    }

    cast::pipeline! {
        stages: {
            raw_facts               @ sample_agent::heartbeat::report,
            normalized_capabilities @ sample::stability::normalize,
            cluster_capabilities    @ sample::stability::aggregate,
            stability_assessment    @ sample::stability::assess,
            upgrade_opportunities   @ sample::stability::propose,
        },
        flow: [
            raw_facts               -> normalized_capabilities,
            normalized_capabilities -> cluster_capabilities,
            cluster_capabilities    -> stability_assessment,
            stability_assessment    -> upgrade_opportunities,
        ],
    }

    cast::tier! {
        axis:      stability,
        direction: increasing,
        of:        sample::stability::StabilityLevel,
        stages: {
            fragile           @ sample::stability::StabilityLevel::Fragile:
                "single point of failure exists",
            recoverable       @ sample::stability::StabilityLevel::Recoverable:
                "failure causes downtime, data survives",
            resilient         @ sample::stability::StabilityLevel::Resilient:
                "failure tolerated with degraded service",
            highly_available  @ sample::stability::StabilityLevel::HighlyAvailable:
                "failure tolerated transparently",
        },
    }

    cast::matrix! {
        columns: [derived, stored, scope, example],
        rows: {
            hotp_tan        @ sample::auth::tan::HotpTan:
                ["yes", "seed only",   "any login",    "RFC 4226"],
            static_tan_list @ sample::auth::tan::StaticTanList:
                ["no",  "finite bag",  "any login",    "recovery codes"],
            transaction_tan @ sample::auth::tan::TransactionTan:
                ["yes", "server-side", "bound to txn", "photoTAN"],
        },
    }

    cast::rule! {
        rule: "Unify at the task level, separate at the secret/lifecycle level",
        why:  "Mixing storage models for derived vs stored secrets produces \
               either silent replay or breakage",
        governs: [
            sample::auth::credential_router,
            sample::auth::vault::derived,
            sample::auth::vault::stored,
        ],
    }

    cast::anti_pattern! {
        avoid:      "if (x && y && z) then do_thing",
        why:        "spaghetti decisions are unmaintainable and unauditable",
        instead:    "inputs -> evaluation -> ranked options -> chosen action",
        instead_at: sample::decision::evaluate,
        governs:    [sample::decision],
    }

    cast::gut_check! {
        question: "Can the code be regenerated later?",
        yes:      "it is a key (TOTP/HOTP)",
        yes_at:   sample::auth::vault::derived,
        no:       "it is a stored artifact (recovery code, TAN)",
        no_at:    sample::auth::vault::stored,
    }

    cast::continues_in! {
        target:  sample::reconciler::apply::dispatch,
        concept: "intent_envelope_consumption",
        why:     "the envelope created here is consumed by the reconciler",
    }

    cast::io::continues_in! {
        target:  "samples/external/keystore_manager.kt",
        lang:    kotlin,
        concept: "intent_envelope_consumption",
        why:     "external side stores the envelope and signs it",
    }
}
