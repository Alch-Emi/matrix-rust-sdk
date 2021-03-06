// Copyright 2020 The Matrix.org Foundation C.I.C.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use cjson::Error as CjsonError;
use olm_rs::errors::{OlmGroupSessionError, OlmSessionError};
use serde_json::Error as SerdeError;
use thiserror::Error;

use super::store::CryptoStoreError;

pub type OlmResult<T> = Result<T, OlmError>;
pub type MegolmResult<T> = Result<T, MegolmError>;

/// Error representing a failure during a device to device cryptographic
/// operation.
#[derive(Error, Debug)]
pub enum OlmError {
    /// The event that should have been decrypted is malformed.
    #[error(transparent)]
    EventError(#[from] EventError),

    /// The received decrypted event couldn't be deserialized.
    #[error(transparent)]
    JsonError(#[from] SerdeError),

    /// The underlying Olm session operation returned an error.
    #[error("can't finish Olm Session operation {0}")]
    OlmSession(#[from] OlmSessionError),

    /// The underlying group session operation returned an error.
    #[error("can't finish Olm Session operation {0}")]
    OlmGroupSession(#[from] OlmGroupSessionError),

    /// The storage layer returned an error.
    #[error("failed to read or write to the crypto store {0}")]
    Store(#[from] CryptoStoreError),

    /// The session with a device has become corrupted.
    #[error("decryption failed likely because a Olm session was wedged")]
    SessionWedged,
}

/// Error representing a failure during a group encryption operation.
#[derive(Error, Debug)]
pub enum MegolmError {
    /// The event that should have been decrypted is malformed.
    #[error(transparent)]
    EventError(#[from] EventError),

    /// The received decrypted event couldn't be deserialized.
    #[error(transparent)]
    JsonError(#[from] SerdeError),

    /// Decryption failed because the session needed to decrypt the event is
    /// missing.
    #[error("decryption failed because the session to decrypt the message is missing")]
    MissingSession,

    /// The underlying group session operation returned an error.
    #[error("can't finish Olm group session operation {0}")]
    OlmGroupSession(#[from] OlmGroupSessionError),

    /// The storage layer returned an error.
    #[error(transparent)]
    Store(#[from] CryptoStoreError),
}

#[derive(Error, Debug)]
pub enum EventError {
    #[error("the Olm message has a unsupported type")]
    UnsupportedOlmType,

    #[error("the Encrypted message has been encrypted with a unsupported algorithm.")]
    UnsupportedAlgorithm,

    #[error("the provided JSON value isn't an object")]
    NotAnObject,

    #[error("the Encrypted message doesn't contain a ciphertext for our device")]
    MissingCiphertext,

    #[error("the Encrypted message is missing the signing key of the sender")]
    MissingSigningKey,

    #[error("the Encrypted message is missing the field {0}")]
    MissingField(String),

    #[error("the sender of the plaintext doesn't match the sender of the encrypted message.")]
    MissmatchedSender,

    #[error("the keys of the message don't match the keys in our database.")]
    MissmatchedKeys,
}

#[derive(Error, Debug)]
pub(crate) enum SignatureError {
    #[error("the provided JSON value isn't an object")]
    NotAnObject,

    #[error("the provided JSON object doesn't contain a signatures field")]
    NoSignatureFound,

    #[error("the provided JSON object can't be converted to a canonical representation")]
    CanonicalJsonError(CjsonError),

    #[error("the signature didn't match the provided key")]
    VerificationError,
}

impl From<CjsonError> for SignatureError {
    fn from(error: CjsonError) -> Self {
        Self::CanonicalJsonError(error)
    }
}
