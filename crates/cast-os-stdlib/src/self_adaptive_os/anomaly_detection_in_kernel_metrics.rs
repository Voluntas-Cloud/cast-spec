//! `anomaly_detection_in_kernel_metrics` — detect abnormal OS behavior.

/// Sentinel for `anomaly_detection_in_kernel_metrics`.
pub struct AnomalyDetectionInKernelMetrics;

cast::concept! {
    name: "anomaly_detection_in_kernel_metrics",
    summary: "detect abnormal OS behavior.",
    anchors: [cast_os_stdlib::self_adaptive_os::anomaly_detection_in_kernel_metrics::AnomalyDetectionInKernelMetrics],
    tags: ["cast_os_stdlib", "self_adaptive_os"],
}
