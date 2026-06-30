// LOCAL-PATCH(warp-cloud-agent-removal): central toggle for removing official
// Warp cloud/account/agent behavior from this fork.
pub(crate) fn is_warp_cloud_agent_removal_enabled() -> bool {
    true
}

pub(crate) fn coerce_official_warp_privacy_setting(value: bool) -> bool {
    if is_warp_cloud_agent_removal_enabled() {
        false
    } else {
        value
    }
}
