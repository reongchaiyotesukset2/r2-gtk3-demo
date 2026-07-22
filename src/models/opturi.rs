use std::{fmt::Write, str::FromStr};

use percent_encoding::{percent_decode_str, utf8_percent_encode, NON_ALPHANUMERIC};
use url::Url;
use zeroize::{Zeroize, ZeroizeOnDrop};


use crate::{
    //backup::RestorableItem,
    models::{Algorithm, Method},
};
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq, Zeroize, ZeroizeOnDrop)]
pub struct OTPUri {
    #[zeroize(skip)]
    pub(crate) algorithm: Algorithm,
    #[zeroize(skip)]
    pub(crate) label: String,
    pub(crate) secret: String,
    #[zeroize(skip)]
    pub(crate) issuer: String,
    #[zeroize(skip)]
    pub(crate) method: Method,
    #[zeroize(skip)]
    pub(crate) digits: Option<u32>,
    #[zeroize(skip)]
    pub(crate) period: Option<u32>,
    #[zeroize(skip)]
    pub(crate) counter: Option<u32>,
}
impl TryFrom<Url> for OTPUri {
    type Error = anyhow::Error;
    fn try_from(url: Url) -> Result<Self, Self::Error> {
        if url.scheme() != "otpauth" {
            anyhow::bail!(
                "Invalid OTP uri format, expected otpauth, got {}",
                url.scheme()
            );
        }

        let mut period = None;
        let mut counter = None;
        let mut digits = None;
        let mut provider_name = None;
        let mut algorithm = None;
        let mut secret = None;

        let pairs = url.query_pairs();

        let method = Method::from_str(url.host_str().unwrap())?;

        let account_info = url
        .path()
        .trim_start_matches('/')
        .split(':')
        .collect::<Vec<&str>>();

        let account_name = if account_info.len() == 1 {
            account_info.first().unwrap()
        } else {
            // If we have "Provider:Account"
            provider_name = Some(account_info.first().unwrap().to_string());
            account_info.get(1).unwrap()
        };

        pairs.for_each(|(key, value)| match key.into_owned().as_str() {
            "period" => {
                period = value.parse::<u32>().ok();
            }
            "digits" => {
                digits = value.parse::<u32>().ok();
            }
            "counter" => {
                counter = value.parse::<u32>().ok();
            }
            "issuer" => {
                provider_name = Some(value.to_string());
            }
            "algorithm" => {
                algorithm = Algorithm::from_str(&value).ok();
            }
            "secret" => {
                secret = Some(value.to_string());
            }
            _ => (),
        });

        if secret.is_none() {
            anyhow::bail!("OTP uri must contain a secret");
        }

        let label = percent_decode_str(account_name).decode_utf8()?.into_owned();
        let issuer = if let Some(n) = provider_name {
            percent_decode_str(&n).decode_utf8()?.into_owned()
        } else {
            "Default".to_string()
        };

        Ok(Self {
            method,
            label,
            secret: secret.unwrap(),
           issuer,
           algorithm: algorithm.unwrap_or_default(),
           digits,
           period,
           counter,
        })
    }
}

impl FromStr for OTPUri {
    type Err = anyhow::Error;
    fn from_str(uri: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(uri)?;
        OTPUri::try_from(url)
    }
}
