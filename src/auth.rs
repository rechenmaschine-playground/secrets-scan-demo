// v0.3.0 validation: a fake JWT should be caught by gitleaks.
// Token is non-functional (random base64 segments).

pub const SERVICE_TOKEN: &str =
    "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ0ZXN0LXVzci0wMDA3IiwiaWF0IjoxNzMyOTkzNDIyfQ.qZ9wB4nT2vL5xY1aC6kE3sD7uF0gH8jM4pR";
