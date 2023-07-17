use std::borrow::Cow;
use std::env;

const CONF_COMMAND_LINE_ARG: &str = "--conf";
const CONF_ENV_VAR: &str = "APP_CONF";
const DEFAULT_CONF_PATH: &str = "/etc/app/app.conf";

fn get_conf_path_from_command_line_args() -> Option<String> {
    let arg_position = env::args().position(|arg| arg == CONF_COMMAND_LINE_ARG)?;
    env::args().nth(arg_position + 1)
}

fn get_conf_path_from_env_var() -> Option<String> {
    env::var(CONF_ENV_VAR).ok()
}

fn detect_config_path() -> Cow<'static, str> {
    get_conf_path_from_command_line_args()
        .or_else(get_conf_path_from_env_var)
        .map(Cow::Owned)
        .unwrap_or(Cow::Borrowed(DEFAULT_CONF_PATH))
}

fn main() {
    println!("Config path: {}", detect_config_path());
}
