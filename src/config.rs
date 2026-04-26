// Test file: this should be flagged by gitleaks as a fake AWS key.
// The string below matches the AKIA[A-Z0-9]{16} default rule. It is
// not a real credential — it is the AWS-docs example value.

pub const AWS_ACCESS_KEY_ID: &str = "AKIAIOSFODNN7EXAMPLE";
pub const AWS_SECRET_ACCESS_KEY: &str = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY";
