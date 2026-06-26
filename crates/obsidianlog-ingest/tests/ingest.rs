//! Integration tests for the ingest service.
//!
//! `#[ignore]`d until the server is implemented so CI stays green on the
//! scaffold.

#[test]
#[ignore = "TODO(impl): boot the server and POST a batch to /ingest"]
fn accepts_a_vector_batch() {
    // TODO(impl): start `serve` on an ephemeral port, POST a JSON batch, and
    // assert a 2xx response plus a chunk written to the (fake) backend.
}

#[test]
#[ignore = "TODO(impl): malformed payloads should be rejected, not panic"]
fn rejects_malformed_payload() {
    // TODO(impl): POST invalid JSON and assert a 4xx response.
}
