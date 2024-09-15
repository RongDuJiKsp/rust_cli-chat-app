use std::cmp::PartialEq;
use base64::Engine;
use serde::{Deserialize, Serialize};

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


impl<I: Deserialize> TryInto<I> for BaseSocketMessageBody {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<Option<I>> {
        if self.body.is_none() {
            return Ok(None);
        }
        let Some(serialized) = self.body;
        match self.content_type {
            SocketBodyType::Plain => {
                let res = serde_json::from_str(&serialized)?;
                Ok(Some(res))
            }
            SocketBodyType::Base64 => {
                let bin = base64::engine::general_purpose::STANDARD.decode(&serialized)?;
                let res = serde_json::from_slice::<I>(&bin)?;
                Ok(Some(res))
            }
        }
    }
}
