#[derive(Debug, Clone, Default)]
pub struct TransactionPreview {
    pub command: String,
    pub output: String,
}

impl TransactionPreview {
    pub fn new(command: impl Into<String>, output: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            output: output.into(),
        }
    }
}
