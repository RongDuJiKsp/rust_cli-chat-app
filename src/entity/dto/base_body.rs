use base64::Engine;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::io::Write;
use std::ops::Add;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum SocketBodyType {
    #[serde(rename = "raw")]
    Raw,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "binary_base64")]
    Base64,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct BaseSocketMessageBody {
    pub end_point: String,
    pub content_type: SocketBodyType,
    body: Option<String>,
}

impl BaseSocketMessageBody {
    pub fn try_trans<I: for<'a> Deserialize<'a>>(self) -> anyhow::Result<Option<I>> {
        match self.body {
            Some(serialized) => match self.content_type {
                SocketBodyType::Json => {
                    let res = serde_json::from_str(&serialized)?;
                    Ok(Some(res))
                }
                SocketBodyType::Base64 => {
                    let bin =
                        base64::engine::general_purpose::STANDARD.decode(serialized.clone())?;
                    let res = serde_json::from_slice::<I>(&bin)?;
                    Ok(Some(res))
                }
                _ => Err(anyhow::anyhow!("Raw Data Can't be trans")),
            },
            None => Ok(None),
        }
    }
    pub fn be_raw(self) -> String {
        self.body.unwrap_or_else(String::new)
    }
    pub fn write_to<T: Write>(&self, &mut stream: T) -> anyhow::Result<()> {
        let json = serde_json::to_string(&self)?.add("\n");
        stream.write_all(json.as_bytes())?;
        stream.flush()?;
        Ok(())
    }
    pub fn try_make_serializable<T: Serialize>(end_point: String, data: T) -> anyhow::Result<Self> {
        Ok(Self {
            end_point,
            body: Some(serde_json::to_string(data)?),
            content_type: SocketBodyType::Json,
        })
    }
    pub fn make_bin(end_point: String, bin: &[u8]) -> Self {
        let encoded = base64::prelude::BASE64_STANDARD.encode(bin);
        Self {
            end_point,
            body: Some(encoded),
            content_type: SocketBodyType::Base64,
        }
    }
    pub fn make_raw(end_point: String, raw: Option<String>) -> Self {
        Self {
            end_point,
            body: raw,
            content_type: SocketBodyType::Raw,
        }
    }
}
