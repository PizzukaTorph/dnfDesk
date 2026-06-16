#[derive(Debug, Clone)]
pub struct TransactionPreview {
    pub installs: Vec<String>,
    pub removals: Vec<String>,
}