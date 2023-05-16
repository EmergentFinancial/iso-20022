// Copyright 2023 Emergent Financial, LLC - All Rights Reserved
//
//
// This software is licensed under the Emergent Financial Limited Public License Version 1.0
// (EF-LPLv1). You may use, copy, modify, and distribute this software under the terms and
// conditions of the EF-LPL. For more information, please refer to the full text of the license
// at https://github.com/emergentfinancial/ef-lpl.
//
//
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

// use iso_20022_dsig::dsig::{Signature, SignatureBuilder};
// use iso_20022_head::head_001_001_03::*;
// use iso_20022_nvlp::nvlp_001_001_01::*;
// use p256::ecdsa::{signature::Signer, Signature as P256Signature, SigningKey};
// use quick_xml::de::from_str;
// use quick_xml::se::to_string;

use super::XmlSignature;
use crate::documents::Dmkr;

pub use const_oid::db::rfc5912::SECP_256_R_1;
use elliptic_curve::sec1::{Coordinates, ToEncodedPoint};

use iso_20022_dsig::dsig::{
    CanonicalizationMethod, CanonicalizationMethodType, DigestMethod, DigestMethodType,
    DigestValue, DigestValueType, KeyInfo, Reference, ReferenceType, SignatureMethod,
    SignatureMethodType, SignedInfo, SignedInfoType, SignedInfoTypeBuilder, Transform,
    TransformType, TransformTypeEnum, Transforms, TransformsType, ID,
};
use iso_20022_dsig::ecdsa::{
    DomainParamsType, DomainParamsTypeEnum, EcPointType, EcdsaKeyValue, EcdsaKeyValueType,
    FieldElemType, HexBinary, NamedCurveType,
};
use iso_20022_dsig::xpath::XPath;
use sha2::{Digest, Sha256};

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    // ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EcdsaSignature {
    inner: iso_20022_dsig::dsig::Signature<
        Dmkr,
        EcdsaKeyValue<Dmkr, Dmkr, Dmkr, Dmkr, Dmkr, HexBinary, HexBinary>,
        Dmkr,
        Dmkr,
        Dmkr,
        Dmkr,
        Dmkr,
        Dmkr,
        Dmkr,
        Dmkr,
        Dmkr,
    >,
}

impl XmlSignature for EcdsaSignature {
    type PublicKey = p256::PublicKey;

    fn set_signed_info(
        self,
        uri: String,
        x_path_transformations: Vec<XPath>,
        digest_value: DigestValueType,
        public_key: Self::PublicKey,
    ) -> Self {
        let mut sig = self.inner;

        let transform = x_path_transformations
            .into_iter()
            .map(|path| Transform {
                value: TransformType {
                    // use filter2 for xpath transformations
                    algorithm: "http://www.w3.org/2002/06/xmldsig-filter2".into(),
                    value: vec![TransformTypeEnum {
                        x_path: Some(path),
                        any: None,
                    }],
                },
            })
            .collect();

        let id = ID {
            value: uuid::Uuid::new_v4().to_string(),
        };

        let ec_points = match public_key.to_encoded_point(false).coordinates() {
            Coordinates::Uncompressed { x, y } => EcPointType {
                x: FieldElemType {
                    value: HexBinary {
                        value: hex::encode(x),
                    },
                },
                y: FieldElemType {
                    value: HexBinary {
                        value: hex::encode(y),
                    },
                },
            },
            _ => EcPointType::default(),
        };

        sig.value.signed_info = SignedInfo {
            value: SignedInfoType {
                id: Some(id.clone()),
                reference: vec![Reference {
                    value: ReferenceType {
                        id: Some(id.clone()),
                        r#type: Some("application/xml".to_string()),
                        uri: Some(uri),
                        // Transforms specify how the document was processed
                        // prior to calculating the digest.
                        transforms: Some(Transforms {
                            value: TransformsType { transform },
                        }),
                        digest_method: DigestMethod {
                            value: DigestMethodType {
                                algorithm: super::DEFAULT_DIGEST_METHOD_ALGORITHM.into(),
                                value: vec![],
                            },
                        },
                        digest_value: DigestValue {
                            value: digest_value,
                        },
                    },
                }],
                canonicalization_method: CanonicalizationMethod {
                    value: CanonicalizationMethodType {
                        value: vec![],
                        algorithm: super::DEFAULT_CANONICALIZATION_METHOD_ALGORITHM.into(),
                    },
                },
                signature_method: SignatureMethod {
                    value: SignatureMethodType {
                        // Default bytes for SHA256 hash output
                        hmac_output_length: Some(iso_20022_dsig::dsig::HmacOutputLengthType {
                            value: super::SHA_256_HMAC_OUTPUT_LENGTH,
                        }),
                        algorithm: format!(
                            "{}#ecdsa-sha256",
                            super::DEFAULT_SIGNATURE_METHOD_ALGORITHM
                        ),
                        value: vec![EcdsaKeyValue {
                            value: EcdsaKeyValueType {
                                domain_parameters: Some(DomainParamsType {
                                    value: DomainParamsTypeEnum {
                                        named_curve: Some(NamedCurveType {
                                            urn: SECP_256_R_1.to_string(),
                                        }),
                                        explicit_params: None,
                                    },
                                }),
                                public_key: ec_points,
                                xmlns: iso_20022_dsig::ecdsa::namespace(),
                            },
                        }],
                    },
                },
            },
        };

        Self { inner: sig }
    }
}

impl signature::Signer<EcdsaSignature> for super::MessageSigner<EcdsaSignature> {
    fn try_sign(&self, msg: &[u8]) -> Result<EcdsaSignature, signature::Error> {
        // Hash the message
        let mut hasher = Sha256::new();
        hasher.update(msg);
        let hash = hasher.finalize();

        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p256() {
        // let sig = SignatureBuilder::default();

        // let sig = sig.build().ok();

        // println!("sig: {:#?}", sig);
        // println!("sig: {:?}", to_string(&sig));
    }
}
