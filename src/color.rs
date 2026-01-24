use std::sync::OnceLock;

static NO_COLOR: OnceLock<bool> = OnceLock::new();

/// Initialize the color configuration based on CLI flag and environment.
/// Should be called once at startup.
pub fn init(cli_no_color: bool) {
    let no_color = cli_no_color || should_disable_color_from_env();
    NO_COLOR.set(no_color).ok();
}

/// Returns true if colors should be disabled.
pub fn no_color() -> bool {
    *NO_COLOR.get().unwrap_or(&false)
}

/// Returns true if colors are enabled.
pub fn color_enabled() -> bool {
    !no_color()
}

/// Check environment variables to determine if color should be disabled.
fn should_disable_color_from_env() -> bool {
    // NO_COLOR standard: https://no-color.org/
    // If NO_COLOR exists (with any value), disable color
    if std::env::var("NO_COLOR").is_ok() {
        return true;
    }

    // Generic CI detection
    if std::env::var("CI").is_ok_and(|v| v == "true") {
        return true;
    }

    // GitHub actions
    if std::env::var("GITHUB_ACTIONS").is_ok_and(|v| v == "true") {
        return true;
    }

    // GitLab CI
    if std::env::var("GITLAB_CI").is_ok() {
        return true;
    }

    // Travis CI
    if std::env::var("TRAVIS").is_ok_and(|v| v == "true") {
        return true;
    }

    // Jenkins
    if std::env::var("JENKINS_URL").is_ok() {
        return true;
    }

    // Check if output is not a TTY (piped)
    // This is a common convention for disabling colors
    if std::env::var("TERM").is_ok_and(|v| v == "dumb") {
        return true;
    }

    false
}
