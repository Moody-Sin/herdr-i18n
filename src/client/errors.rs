use std::io;

use crate::protocol;
use crate::server::socket_paths::client_socket_path;

/// Errors that can occur during client operation.
#[derive(Debug)]
pub enum ClientError {
    /// Could not connect to the server's client socket.
    ConnectionFailed(io::Error),
    /// Server rejected our handshake.
    HandshakeRejected { version: u32, error: String },
    /// Server shut down.
    ServerShutdown { reason: Option<String> },
    /// Lost connection to the server.
    ConnectionLost(io::Error),
    /// Protocol error (framing, deserialization).
    Protocol(protocol::FramingError),
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::ConnectionFailed(err) => {
                write!(
                    f,
                    "{}",
                    crate::trf!("failed to connect to server: {error}", error = err)
                )?;
                let path = client_socket_path();
                write!(
                    f,
                    "\n{}",
                    crate::tr!("Is herdr server running? Start it with `herdr server`.")
                )?;
                write!(
                    f,
                    "\n{}",
                    crate::trf!("Socket path: {path}", path = path.display())
                )
            }
            ClientError::HandshakeRejected { version, error } => {
                write!(
                    f,
                    "{}",
                    crate::trf!(
                        "server rejected handshake (version {version}): {error}",
                        version = version,
                        error = error
                    )
                )
            }
            ClientError::ServerShutdown { reason } => {
                match reason.as_deref() {
                    Some("detached") => {
                        if let Ok(reattach_command) =
                            std::env::var(crate::remote::REATTACH_COMMAND_ENV_VAR)
                        {
                            write!(f, "{}", crate::tr!("detached from remote server"))?;
                            write!(
                                f,
                                "\n{}",
                                crate::trf!(
                                    "Run `{command}` to reattach",
                                    command = reattach_command
                                )
                            )?;
                        } else {
                            write!(f, "{}", crate::tr!("detached from server"))?;
                            write!(
                                f,
                                "\n{}",
                                crate::trf!(
                                    "Run `{command}` to reattach",
                                    command = crate::session::local_attach_command()
                                )
                            )?;
                        }
                    }
                    _ => {
                        write!(f, "{}", crate::tr!("server shut down"))?;
                        if let Some(reason) = reason {
                            write!(f, ": {reason}")?;
                        }
                    }
                }
                Ok(())
            }
            ClientError::ConnectionLost(err) => {
                if let Ok(reattach_command) = std::env::var(crate::remote::REATTACH_COMMAND_ENV_VAR)
                {
                    write!(
                        f,
                        "{}",
                        crate::trf!("lost connection to remote Herdr: {error}", error = err)
                    )?;
                    write!(
                        f,
                        "\n{}",
                        crate::tr!("If the remote server survived the SSH or network drop, its panes may still be running.")
                    )?;
                    write!(
                        f,
                        "\n{}",
                        crate::trf!("Run `{command}` to reattach", command = reattach_command)
                    )
                } else {
                    write!(
                        f,
                        "{}",
                        crate::trf!("lost connection to server: {error}", error = err)
                    )
                }
            }
            ClientError::Protocol(err) => {
                write!(f, "{}", crate::trf!("protocol error: {error}", error = err))
            }
        }
    }
}

impl std::error::Error for ClientError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ClientError::ConnectionFailed(err) => Some(err),
            ClientError::ConnectionLost(err) => Some(err),
            ClientError::Protocol(err) => Some(err),
            _ => None,
        }
    }
}

impl From<protocol::FramingError> for ClientError {
    fn from(err: protocol::FramingError) -> Self {
        match err {
            protocol::FramingError::UnexpectedEof => ClientError::ConnectionLost(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "server closed connection",
            )),
            protocol::FramingError::Io(err) => ClientError::ConnectionLost(err),
            err => ClientError::Protocol(err),
        }
    }
}
