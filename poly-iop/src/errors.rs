//! Error module.

use ark_std::string::String;
use displaydoc::Display;

/// A `enum` specifying the possible failure modes of the PolyIOP.
#[derive(Display, Debug)]
pub enum PolyIOPErrors {
    /// Invalid Prover
    InvalidProver(String),
    /// Invalid Verifier
    InvalidVerifier(String),
    /// Invalid Proof
    InvalidProof(String),
    /// Invalid parameters
    InvalidParameters(String),
    /// Invalid Transcript
    InvalidTranscript(String),
    /// An error during (de)serialization
    SerializationError(ark_serialize::SerializationError),
}

impl From<ark_serialize::SerializationError> for PolyIOPErrors {
    fn from(e: ark_serialize::SerializationError) -> Self {
        Self::SerializationError(e)
    }
}