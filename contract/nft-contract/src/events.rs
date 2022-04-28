use std::fmt;

use near_sdk::serde::{Deserialize, Serialize};

/// Enum that represetns the data type of the EventLog. 
/// Can be either NftMint or NftTransfer
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag="event", content="data")]
#[serde(rename_all="snake_case")]
#[serde(crate = "near_sdk::serde")]
#[non_exhaustive]
pub enum EventLogVariant {
  NftMint(Vec<NftMintLog>),
  NftTransfer(Vec<NftTransferLog>),
}


/// Interface to capture data about an event
/// 
/// Arguments:
///   standard: name of standard. E.g. nep171
///   version: version number. E.g. 1.0.0
///   event: associated event data.
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EventLog {
  pub standard: String,
  pub version: String,

  // flatten so no event, we just want the content. 
  #[serde(flatten)]
  pub event: EventLogVariant,
}

impl fmt::Display for EventLog {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_fmt(format_args!(
      "EVENT_JSON:{}",
      &serde_json::to_string(self).map_err(|_| fmt::Error)?
    ))
  }
}


/// An event log to capture token minting
/// 
/// Arguments:
///   owner_id: in "account.near" for example. 
///   token_ids: (array) ["1", "abc"] 
///   memo: (optional) message. 
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftMintLog {
  pub owner_id: String,
  pub token_ids: Vec<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub memo: Option<String>,
}


/// An event log to capture token transfer
/// 
/// Arguments:
///   authorized_id: accounts approved to perform the transfer.
///   old_owner_id: "owner.near"  currently holding
///   new_owner_id: "receiver.near" who to transfer to.
///   token_ids: ["1", "12345abc"]
///   memo: (optional) message.
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftTransferLog {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub authorized_id: Option<String>,

  pub old_owner_id: String,
  pub new_owner_id: String,
  pub token_ids: Vec<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub memo: Option<String>,
}


