use std::error::Error;
use std::fmt::Display;

use tracing_error::SpanTrace;

pub struct SpanErr<T: Error + Display> {
    pub error: T,
    pub span: SpanTrace,
}

impl<T: Error + Display> SpanErr<T> {
    pub fn map<U: Error + Display>(self, f: impl FnOnce(T) -> U) -> SpanErr<U> {
        SpanErr {
            error: f(self.error),
            span: self.span,
        }
    }
}

impl<T: Error + Display> From<T> for SpanErr<T> {
    fn from(error: T) -> Self {
        Self {
            error,
            span: SpanTrace::capture(),
        }
    }
}
