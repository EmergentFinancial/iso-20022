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

use super::{XmlPublicKey, XmlSignature};
use crate::documents::Dmkr;

use elliptic_curve::{
    pkcs8::{self, spki::AssociatedAlgorithmIdentifier, EncodePublicKey},
    point::{PointCompaction, PointCompression},
    sec1::{Coordinates, FromEncodedPoint, ModulusSize, ToEncodedPoint},
    AffinePoint, Curve, FieldBytesSize, PrimeCurve,
};

use base64::{engine::general_purpose, Engine};
use iso_20022_dsig::dsig::{
    Base64Binary, CanonicalizationMethod, CanonicalizationMethodType, DigestMethod,
    DigestMethodType, DigestValue, DigestValueType, KeyInfo, KeyInfoType, KeyInfoTypeEnum,
    Reference, ReferenceType, SignatureMethod, SignatureMethodType, SignatureValue,
    SignatureValueType, SignedInfo, SignedInfoType, Transform, TransformType, TransformTypeEnum,
    Transforms, TransformsType, X509Data, X509DataType, X509DataTypeEnum, ID,
};
use iso_20022_dsig::ecdsa::{
    DomainParamsType, DomainParamsTypeEnum, EcPointType, EcdsaKeyValue, EcdsaKeyValueType,
    FieldElemType, HexBinary, NamedCurveType,
};
use iso_20022_dsig::xpath::XPath;
use p256::{ecdsa::Signature, pkcs8::AssociatedOid, NistP256};
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
#[serde(transparent)]
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

#[derive(Default, Debug, Clone)]
pub enum EcdsaPublicKey {
    P256(p256::PublicKey),
    #[default]
    Unknown,
}

impl XmlPublicKey for EcdsaPublicKey {
    type PublicKey = Self;

    fn to_ec_point(&self) -> EcPointType<HexBinary, HexBinary> {
        match self {
            Self::P256(key) => match key.to_encoded_point(false).coordinates() {
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
            },
            _ => {
                unimplemented!()
            }
        }
    }

    fn to_der_bytes(self) -> Vec<u8> {
        match self {
            Self::P256(key) => {
                let public_key_bytes = key.to_encoded_point(false);
                if let Ok(subject_public_key) =
                    der::asn1::BitStringRef::new(0, public_key_bytes.as_bytes())
                {
                    let der: Option<pkcs8::der::Document> = pkcs8::SubjectPublicKeyInfo {
                        algorithm: p256::PublicKey::ALGORITHM_IDENTIFIER,
                        subject_public_key,
                    }
                    .try_into()
                    .ok();

                    der.map(|d| d.to_vec()).unwrap_or_default()
                } else {
                    vec![]
                }
            }
            _ => {
                unimplemented!()
            }
        }
    }

    fn oid(&self) -> String {
        match self {
            Self::P256(_) => NistP256::OID.to_string(),
            _ => {
                unimplemented!()
            }
        }
    }
}

impl XmlSignature for EcdsaSignature {
    type PublicKey = EcdsaPublicKey;

    type Digest = Sha256;

    fn set_signed_info(
        self,
        x_path_transformations: Vec<XPath>,
        digest_value: DigestValueType,
        public_key: Self::PublicKey,
        uri: Option<String>,
    ) -> Self {
        let mut sig = self.inner;

        let transform: Vec<Transform<_>> = x_path_transformations
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

        let transforms = if transform.is_empty() {
            None
        } else {
            Some(Transforms {
                value: TransformsType { transform },
            })
        };

        let id = uuid::Uuid::new_v4().to_string();

        // Set the signed info
        sig.value.signed_info = SignedInfo {
            value: SignedInfoType {
                id: Some(id.clone()),
                reference: vec![Reference {
                    value: ReferenceType {
                        id: Some(id.clone()),
                        r#type: Some("application/xml".to_string()),
                        uri,
                        // Transforms specify how the document was processed
                        // prior to calculating the digest.
                        transforms,
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
                                            urn: public_key.oid(),
                                        }),
                                        explicit_params: None,
                                    },
                                }),
                                public_key: public_key.to_ec_point(),
                                xmlns: iso_20022_dsig::ecdsa::namespace(),
                            },
                        }],
                    },
                },
            },
        };

        // Set signature namespace
        sig.value.xmlns = iso_20022_dsig::dsig::namespace();

        Self { inner: sig }
    }

    fn set_key_info(self, public_key: Self::PublicKey) -> Self {
        let mut sig = self.inner;

        // Encode the public key to ASN.1 DER bytes
        let der_bytes = public_key.to_der_bytes();
        // Encode the ASN.1 DER bytes to base64
        let der_base64 = general_purpose::STANDARD.encode(&der_bytes);

        sig.value.key_info = Some(KeyInfo {
            value: KeyInfoType {
                value: vec![KeyInfoTypeEnum {
                    x_509_data: Some(X509Data {
                        value: X509DataType {
                            value: X509DataTypeEnum {
                                x_509_certificate: Some(Base64Binary { value: der_base64 }),
                                ..Default::default()
                            },
                        },
                    }),
                    ..Default::default()
                }],
                id: None,
            },
        });

        Self { inner: sig }
    }

    fn digest_value(&self) -> Option<DigestValueType> {
        self.inner
            .value
            .signed_info
            .value
            .reference
            .first()
            .map(|r| r.value.digest_value.value.clone())
    }

    fn hash(data: &[u8]) -> DigestValueType {
        // Hash the message
        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash = hasher.finalize();

        DigestValueType {
            value: general_purpose::STANDARD.encode(hash),
        }
    }

    /// Set the signature value
    fn set_signature_value(self, signature_value: Base64Binary, id: Option<String>) -> Self {
        let mut sig = self.inner;

        sig.value.signature_value = SignatureValue {
            value: SignatureValueType {
                value: signature_value,
                id,
            },
        };

        Self { inner: sig }
    }
}

#[derive(Default, Debug, Clone)]
pub enum EcdsaSigner {
    /// P-256
    P256(p256::ecdsa::SigningKey),
    /// Unknown signer
    #[default]
    Unknown,
}

impl signature::Signer<EcdsaSignature> for EcdsaSigner {
    fn try_sign(&self, msg: &[u8]) -> Result<EcdsaSignature, signature::Error> {
        let signature = EcdsaSignature::default();

        match self {
            Self::P256(sk) => {
                let sig: Signature = sk.try_sign(msg)?;
                let bytes = sig.to_vec();

                // Convert signature into base64 value
                let signature_value = Base64Binary {
                    value: general_purpose::STANDARD.encode(&bytes),
                };

                let sig = signature.set_signature_value(signature_value, None);

                Ok(sig)
            }
            _ => {
                unimplemented!()
            }
        }
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
