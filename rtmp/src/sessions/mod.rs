/*!
This module contains implemented session abstractions.

A session is a high level abstraction that makes it simple to create custom RTMP clients and
servers without having to worry about the exact flow of RTMP messages to perform specific
actions.  The session has it's own `ChunkSerializer` and `ChunkDeserializer` so consumers only
have to worry about bytes in and bytes/events out.

A single session represents a single peer in an RTMP connection, so if multiple connections are
being managed (in any direction) each connection should have its own, distinct, session instance.

It is also expected that a session has been created *after* handshaking has been completed.
*/

mod client;
mod server;

pub use self::client::ClientSession;
pub use self::client::ClientSessionConfig;
pub use self::client::ClientSessionError;
pub use self::client::ClientSessionErrorKind;
pub use self::client::ClientSessionEvent;
pub use self::client::ClientSessionResult;
pub use self::client::ClientState;
pub use self::client::PublishRequestType;

pub use self::server::ServerSession;
pub use self::server::ServerSessionConfig;
pub use self::server::ServerSessionError;
pub use self::server::ServerSessionErrorKind;
pub use self::server::ServerSessionEvent;
pub use self::server::ServerSessionResult;

use rml_amf0::Amf0Value;
use std::collections::HashMap;

/// Contains the metadata information a stream may advertise on publishing
#[derive(PartialEq, Debug, Clone)]
pub struct StreamMetadata {
    pub video_width: Option<u32>,
    pub video_height: Option<u32>,
    pub video_codec: Option<String>,
    pub video_frame_rate: Option<f32>,
    pub video_bitrate_kbps: Option<u32>,
    pub audio_codec: Option<String>,
    pub audio_bitrate_kbps: Option<u32>,
    pub audio_sample_rate: Option<u32>,
    pub audio_channels: Option<u32>,
    pub audio_is_stereo: Option<bool>,
    pub encoder: Option<String>,
}

impl StreamMetadata {
    /// Creates a new (and empty) metadata instance
    fn new() -> StreamMetadata {
        StreamMetadata {
            video_width: None,
            video_height: None,
            video_codec: None,
            video_frame_rate: None,
            video_bitrate_kbps: None,
            audio_codec: None,
            audio_bitrate_kbps: None,
            audio_sample_rate: None,
            audio_channels: None,
            audio_is_stereo: None,
            encoder: None,
        }
    }

    fn apply_metadata_values(&mut self, mut properties: HashMap<String, Amf0Value>) {
        for (key, value) in properties.drain() {
            match key.as_ref() {
                "width" => {
                    if let Some(x) = value.get_number() {
                        self.video_width = Some(x as u32)
                    }
                }

                "height" => {
                    if let Some(x) = value.get_number() {
                        self.video_height = Some(x as u32)
                    }
                }

                "videocodecid" => {
                    if let Some(x) = value.get_string() {
                        self.video_codec = Some(x)
                    }
                }

                "videodatarate" => {
                    if let Some(x) = value.get_number() {
                        self.video_bitrate_kbps = Some(x as u32)
                    }
                }

                "framerate" => {
                    if let Some(x) = value.get_number() {
                        self.video_frame_rate = Some(x as f32)
                    }
                }

                "audiocodecid" => {
                    if let Some(x) = value.get_string() {
                        self.audio_codec = Some(x)
                    }
                }

                "audiodatarate" => {
                    if let Some(x) = value.get_number() {
                        self.audio_bitrate_kbps = Some(x as u32)
                    }
                }

                "audiosamplerate" => {
                    if let Some(x) = value.get_number() {
                        self.audio_sample_rate = Some(x as u32)
                    }
                }

                "audiochannels" => {
                    if let Some(x) = value.get_number() {
                        self.audio_channels = Some(x as u32)
                    }
                }

                "stereo" => {
                    if let Some(x) = value.get_boolean() {
                        self.audio_is_stereo = Some(x)
                    }
                }

                "encoder" => {
                    if let Some(x) = value.get_string() {
                        self.encoder = Some(x)
                    }
                }

                _ => (),
            }
        }
    }
}
