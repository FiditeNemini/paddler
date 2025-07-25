use serde::Deserialize;
use serde::Serialize;

use super::Request;
use crate::jsonrpc::Error;
use crate::jsonrpc::ErrorEnvelope;
use crate::jsonrpc::RequestEnvelope;
use crate::rpc_message::RpcMessage;

#[derive(Deserialize, Serialize)]
pub enum Message {
    Error(ErrorEnvelope<Error>),
    Request(RequestEnvelope<Request>),
}

impl RpcMessage for Message {}
