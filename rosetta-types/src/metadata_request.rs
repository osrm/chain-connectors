/*
 * Rosetta
 *
 * Build Once. Integrate Your Blockchain Everywhere.
 *
 * The version of the OpenAPI document: 1.4.13
 *
 * Generated by: https://openapi-generator.tech
 */

/// A `MetadataRequest` is utilized in any request where the only argument is optional metadata.
#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MetadataRequest {
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl MetadataRequest {
    /// A `MetadataRequest` is utilized in any request where the only argument is optional metadata.
    #[must_use]
    pub const fn new() -> Self {
        Self { metadata: None }
    }
}
