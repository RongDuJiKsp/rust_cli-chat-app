use base64::Engine;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum SocketBodyType {
    #[serde(rename = "plain")]
    Plain,
    #[serde(rename = "binary_base64")]
    Base64,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct BaseSocketMessageBody {
    end_point: String,
    content_type: SocketBodyType,
    body: Option<String>,
}

impl BaseSocketMessageBody {
    fn try_trans<I: for<'a> Deserialize<'a>>(self) -> anyhow::Result<Option<I>> {
        match self.body {
            Some(serialized) => match self.content_type {
                SocketBodyType::Plain => {
                    let res = serde_json::from_str(&serialized)?;
                    Ok(Some(res))
                }
                SocketBodyType::Base64 => {
                    let bin =
                        base64::engine::general_purpose::STANDARD.decode(serialized.clone())?;
                    let res = serde_json::from_slice::<I>(&bin)?;
                    Ok(Some(res))
                }
            },
            None => Ok(None),
        }
    }
}
