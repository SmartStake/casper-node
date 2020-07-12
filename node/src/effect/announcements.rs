//! Announcement effects.
//!
//! Announcements indicate new incoming data or events from various sources. See the top-level
//! module documentation for details.

use std::fmt::{self, Display, Formatter};

use crate::types::Deploy;

/// A networking layer announcement.
#[derive(Debug)]
#[must_use]
pub enum NetworkAnnouncement<I, P> {
    /// A payload message has been received from a peer.
    MessageReceived {
        /// The sender of the message
        sender: I,
        /// The message payload
        payload: P,
    },
}

impl<I, P> Display for NetworkAnnouncement<I, P>
where
    I: Display,
    P: Display,
{
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            NetworkAnnouncement::MessageReceived { sender, payload } => {
                write!(formatter, "received from {}: {}", sender, payload)
            }
        }
    }
}

/// An HTTP API server announcement.
#[derive(Debug)]
#[must_use]
pub enum ApiServerAnnouncement {
    /// A new deploy received.
    DeployReceived {
        /// The received deploy.
        deploy: Box<Deploy>,
    },
}

impl Display for ApiServerAnnouncement {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ApiServerAnnouncement::DeployReceived { deploy } => {
                write!(formatter, "api server received {}", deploy.id())
            }
        }
    }
}