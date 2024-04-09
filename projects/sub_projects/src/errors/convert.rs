use super::*;

impl From<ExampleErrorKind> for ExampleError {
    fn from(value: ExampleErrorKind) -> Self {
        Self {
            kind: Box::new(value),
        }
    }
}