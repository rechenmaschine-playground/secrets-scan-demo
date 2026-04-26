// Test file: a generic high-entropy "API key" assignment. This pattern
// is caught by gitleaks' generic-api-key rule but generally not by
// GitHub's native push protection (which only blocks high-confidence
// partner-validated patterns).

pub const API_KEY: &str = "kx9P3vN7tQ2wJ5mZ8rL4eY1hC6bF0aDuG3sB9XnVkA";
