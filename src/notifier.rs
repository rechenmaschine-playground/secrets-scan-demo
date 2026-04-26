// Sends launch-status pings to the team Slack channel.
// (Demo file — the webhook below is fake and used to exercise gitleaks.)

pub const LAUNCH_NOTIFY_WEBHOOK: &str =
    "https://hooks.slack.com/services/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tU";

pub fn notify(_message: &str) {
    // ...post to LAUNCH_NOTIFY_WEBHOOK
}
