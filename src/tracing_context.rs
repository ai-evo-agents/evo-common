//! Trace-context propagation helpers for distributed tracing.
//!
//! Provides inject/extract functions for two transport types:
//! * **HashMap** – for embedding trace context in Socket.IO event payloads.
//! * **HTTP HeaderMap** – for W3C `traceparent` propagation over HTTP.

use opentelemetry::propagation::{Extractor, Injector};
use opentelemetry::{Context, global};
use std::collections::HashMap;

// ─── HashMap carrier (Socket.IO) ─────────────────────────────────────────────

struct HashMapInjector<'a>(&'a mut HashMap<String, String>);

impl Injector for HashMapInjector<'_> {
    fn set(&mut self, key: &str, value: String) {
        self.0.insert(key.to_string(), value);
    }
}

struct HashMapExtractor<'a>(&'a HashMap<String, String>);

impl Extractor for HashMapExtractor<'_> {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(|v| v.as_str())
    }

    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(|k| k.as_str()).collect()
    }
}

/// Inject the current span's trace context into a `HashMap`.
///
/// Use this before emitting a Socket.IO event so the receiver can parent its
/// spans to the current trace.
pub fn inject_context(carrier: &mut HashMap<String, String>) {
    global::get_text_map_propagator(|propagator| {
        propagator.inject_context(&Context::current(), &mut HashMapInjector(carrier));
    });
}

/// Extract a parent trace context from a `HashMap`.
///
/// Use this when handling an incoming Socket.IO event to continue the trace
/// started by the sender.
pub fn extract_context(carrier: &HashMap<String, String>) -> Context {
    global::get_text_map_propagator(|propagator| propagator.extract(&HashMapExtractor(carrier)))
}
