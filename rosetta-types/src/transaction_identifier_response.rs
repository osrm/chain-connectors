/*
 * Rosetta
 *
 * Build Once. Integrate Your Blockchain Everywhere.
 *
 * The version of the OpenAPI document: 1.4.13
 *
 * Generated by: https://openapi-generator.tech
 */

/// `TransactionIdentifierResponse` : `TransactionIdentifierResponse` contains the
/// `transaction_identifier` of a transaction that was submitted to either `/construction/hash` or
/// `/construction/submit`.
#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TransactionIdentifierResponse {
    #[serde(rename = "transaction_identifier")]
    pub transaction_identifier: crate::TransactionIdentifier,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl TransactionIdentifierResponse {
    /// `TransactionIdentifierResponse` contains the `transaction_identifier` of a transaction that
    /// was submitted to either `/construction/hash` or `/construction/submit`.
    #[must_use]
    pub const fn new(transaction_identifier: crate::TransactionIdentifier) -> Self {
        Self { transaction_identifier, metadata: None }
    }
}
