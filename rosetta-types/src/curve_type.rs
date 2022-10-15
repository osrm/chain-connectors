/*
 * Rosetta
 *
 * Build Once. Integrate Your Blockchain Everywhere.
 *
 * The version of the OpenAPI document: 1.4.13
 *
 * Generated by: https://openapi-generator.tech
 */

/// CurveType : CurveType is the type of cryptographic curve associated with a PublicKey.  * secp256k1: SEC compressed - `33 bytes` (https://secg.org/sec1-v2.pdf#subsubsection.2.3.3) * secp256r1: SEC compressed - `33 bytes` (https://secg.org/sec1-v2.pdf#subsubsection.2.3.3) * edwards25519: `y (255-bits) || x-sign-bit (1-bit)` - `32 bytes` (https://ed25519.cr.yp.to/ed25519-20110926.pdf) * tweedle: 1st pk : Fq.t (32 bytes) || 2nd pk : Fq.t (32 bytes) (https://github.com/CodaProtocol/coda/blob/develop/rfcs/0038-rosetta-construction-api.md#marshal-keys) * pallas: `x (255 bits) || y-parity-bit (1-bit) - 32 bytes` (https://github.com/zcash/pasta)

/// CurveType is the type of cryptographic curve associated with a PublicKey.  * secp256k1: SEC compressed - `33 bytes` (https://secg.org/sec1-v2.pdf#subsubsection.2.3.3) * secp256r1: SEC compressed - `33 bytes` (https://secg.org/sec1-v2.pdf#subsubsection.2.3.3) * edwards25519: `y (255-bits) || x-sign-bit (1-bit)` - `32 bytes` (https://ed25519.cr.yp.to/ed25519-20110926.pdf) * tweedle: 1st pk : Fq.t (32 bytes) || 2nd pk : Fq.t (32 bytes) (https://github.com/CodaProtocol/coda/blob/develop/rfcs/0038-rosetta-construction-api.md#marshal-keys) * pallas: `x (255 bits) || y-parity-bit (1-bit) - 32 bytes` (https://github.com/zcash/pasta)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CurveType {
    #[serde(rename = "secp256k1")]
    Secp256k1,
    #[serde(rename = "secp256r1")]
    Secp256r1,
    #[serde(rename = "edwards25519")]
    Edwards25519,
    #[serde(rename = "tweedle")]
    Tweedle,
    #[serde(rename = "pallas")]
    Pallas,
    #[serde(rename = "schnorrkel")]
    Schnorrkel,
}

impl ToString for CurveType {
    fn to_string(&self) -> String {
        match self {
            Self::Secp256k1 => String::from("secp256k1"),
            Self::Secp256r1 => String::from("secp256r1"),
            Self::Edwards25519 => String::from("edwards25519"),
            Self::Tweedle => String::from("tweedle"),
            Self::Pallas => String::from("pallas"),
            Self::Schnorrkel => String::from("schnorrkel"),
        }
    }
}

impl Default for CurveType {
    fn default() -> CurveType {
        Self::Secp256k1
    }
}
