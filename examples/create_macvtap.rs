// SPDX-License-Identifier: MIT

use futures::stream::TryStreamExt;
use netlink_packet_route::link::MacVtapMode;
use rtnetlink::{new_connection, Error, Handle};
use std::env;

#[tokio::main]
async fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage();
        return Ok(());
    }
    let link_name = &args[1];

    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);

    create_macvtap(handle, link_name.to_string())
        .await
        .map_err(|e| format!("{e}"))
}

async fn create_macvtap(
    handle: Handle,
    veth_name: String,
) -> Result<(), Error> {
    let mut links = handle.link().get().match_name(veth_name.clone()).execute();
    if let Some(link) = links.try_next().await? {
        let request = handle.link().add().macvtap(
            "test_macvtap".into(),
            link.header.index,
            MacVtapMode::Bridge,
        );
        request.execute().await?
    } else {
        println!("no link link {veth_name} found");
    }
    Ok(())
}

fn usage() {
    eprintln!(
        "usage:
    cargo run --example create_macvtap -- <link name>

Note that you need to run this program as root. Instead of running cargo as root,
build the example normally:

    cd rtnetlink; cargo build --example create_macvtap

Then find the binary in the target directory:

    cd ../target/debug/example ; sudo ./create_macvtap <link_name>"
    );
}
