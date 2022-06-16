use std::{env, path::PathBuf};

pub fn home_dir() -> PathBuf {
    let path = env::var("HOME")
        .or(env::var("USER").and_then(|user| Ok(format!("/home/{user}"))))
        .expect("No HOME or USER env vars found");
    let path = PathBuf::from(path);

    if path.exists() {
        panic!("Home directory ({path:?}) does not exist")
    }

    path
}

pub fn config_dir() -> PathBuf {
    let path = env::var("XDG_CONFIG_HOME")
        .or(env::var("HOME").and_then(|mut home| {
            home.push_str("/.config");
            Ok(home)
        }))
        .or(env::var("USER").and_then(|user| Ok(format!("/home/{user}/.config/dim"))))
        .expect("No XDG_CONFIG_HOME, HOME or USER env vars found");

    let path = PathBuf::from(path);

    if path.exists() {
        panic!("Home directory ({path:?}) does not exist")
    }

    path
}
