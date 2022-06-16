use std::{
    error::Error,
    fs::read_dir,
    net::TcpStream,
    path::{Path, PathBuf},
};

use git2::Cred;
use ssh2::Session;

use crate::dirs::home_dir;

struct GitUrl {
    hostname: String,
    username: String,
}

impl TryFrom<String> for GitUrl {
    type Error = Box<dyn Error>;

    fn try_from(mut value: String) -> Result<Self, Self::Error> {
        value = value.replace("ssh://", "");

        let mut username = "root";
        let mut destination = value.clone();

        let mut parts = value.split("@");

        if parts.clone().count() >= 2 {
            username = parts.next().unwrap();
            destination = parts.next().unwrap().into();
        }

        let hostname = destination.split(":").next().unwrap().into();
        let username = username.into();

        Ok(GitUrl { username, hostname })
    }
}

fn get_shh_key(session: &Session, user: &str, hostname: &str) -> Result<PathBuf, Box<dyn Error>> {
    let ssh_path = Path::new("/home/{}");
    let home_path = home_dir();

    for entry in read_dir(ssh_path)? {
        let entry = entry?;
        if let Some(publickey) = entry.file_name().to_str() {
            if publickey.starts_with("id_") && publickey.ends_with(".pub") {
                let privatekey = home_path.join(".ssh").join(publickey.replace(".pub", ""));
                let publickey = home_path.join(".ssh").join(publickey);

                if privatekey.exists() && publickey.exists() {
                    if let Ok(()) = session.userauth_hostbased_file(
                        &user,
                        &publickey,
                        &privatekey,
                        None,
                        hostname,
                        None,
                    ) {
                        if session.authenticated() {
                            return Ok(privatekey);
                        }
                    }
                }
            }
        }
    }

    return Err("No keys found".into());
}

pub fn ssh_credentials(url: String) -> Result<Cred, Box<dyn Error>> {
    let url = GitUrl::try_from(url)?;

    let mut session = Session::new()?;

    let tcp = TcpStream::connect(&url.hostname).unwrap();
    session.set_tcp_stream(tcp);

    session.handshake()?;

    let key = get_shh_key(&session, &url.username, &url.hostname)?;

    Ok(Cred::ssh_key(&url.username, None, &key, None)?)
}
