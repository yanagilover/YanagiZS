use std::time::Duration;

use anyhow::Result;
use protocol::{RpcGetPlayerBasicInfoArg, RpcGetPlayerBasicInfoRet};
use qwer_rpc::{ProtocolServiceFrontend, RpcPtcServiceFrontend};

#[tokio::main]
async fn main() -> Result<()> {
    common::logging::init(tracing::Level::DEBUG);

    let service = RpcPtcServiceFrontend::new(ProtocolServiceFrontend::new());
    let protocol_point = service.create_point(None).await?;

    let rsp = protocol_point
        .call_rpc::<_, RpcGetPlayerBasicInfoRet>(
            "127.0.0.1:10101".parse()?,
            RpcGetPlayerBasicInfoArg::default(),
            Vec::with_capacity(0),
            Duration::from_secs(5),
        )
        .await?;

    println!("rsp: {rsp:?}");

    let protocol_point = service.create_point(None).await?;

    let rsp = protocol_point
        .call_rpc::<_, RpcGetPlayerBasicInfoRet>(
            "127.0.0.1:10101".parse()?,
            RpcGetPlayerBasicInfoArg::default(),
            Vec::with_capacity(0),
            Duration::from_secs(5),
        )
        .await?;

    println!("rsp: {rsp:?}");

    Ok(())
}
