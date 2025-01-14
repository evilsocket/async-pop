use std::env;

// use async_native_tls::{TlsConnector, TlsStream};
#[cfg(feature = "runtime-async-std")]
use async_std::net::TcpStream;
use dotenv::dotenv;
use log::info;
#[cfg(feature = "runtime-tokio")]
use tokio::net::TcpStream;

use crate::{
    response::{capability::Capability, list::ListResponse, types::DataType, uidl::UidlResponse},
    ClientState,
};

use super::Client;

#[derive(Debug)]
struct ClientInfo {
    server: String,
    port: u16,
    username: String,
    password: String,
}

fn create_client_info() -> ClientInfo {
    dotenv().ok();

    ClientInfo {
        server: env::var("SERVER").unwrap().to_owned(),
        port: env::var("PORT").unwrap().parse().unwrap(),
        username: env::var("USERNAME").unwrap().to_owned(),
        password: env::var("PASSWORD").unwrap().to_owned(),
    }
}

async fn create_logged_in_client() -> Client<TcpStream> {
    let client_info = create_client_info();
    let server = client_info.server.as_ref();
    let port = client_info.port;

    let username = client_info.username;
    let password = client_info.password;

    let mut client = super::connect_plain((server, port)).await.unwrap();

    client.login(username, password).await.unwrap();

    client
}

// async fn create_logged_in_client_tls() -> Client<TlsStream<TcpStream>> {
//     let client_info = create_client_info();
//     let server = client_info.server.as_ref();
//     let port = client_info.port;

//     let username = client_info.username;
//     let password = client_info.password;

//     let tls = TlsConnector::new();

//     let mut client = super::connect((server, port), server, &tls).await.unwrap();

//     client.login(username, password).await.unwrap();

//     client
// }

#[cfg_attr(feature = "runtime-tokio", tokio::test)]
#[cfg_attr(feature = "runtime-async-std", async_std::test)]
async fn e2e_connect() {
    let client_info = create_client_info();

    let server = client_info.server.as_ref();
    let port = client_info.port;

    let mut client = super::connect_plain((server, port)).await.unwrap();

    let greeting = client.greeting().unwrap();

    info!("{}", greeting);

    // assert_eq!(greeting, "POP3 GreenMail Server v1.6.12 ready");

    client.quit().await.unwrap();
}

#[cfg_attr(feature = "runtime-tokio", tokio::test)]
#[cfg_attr(feature = "runtime-async-std", async_std::test)]
async fn e2e_login() {
    let mut client = create_logged_in_client().await;

    assert_eq!(client.get_state(), &ClientState::Transaction);

    client.quit().await.unwrap();
}

#[cfg_attr(feature = "runtime-tokio", tokio::test)]
#[cfg_attr(feature = "runtime-async-std", async_std::test)]
async fn e2e_noop() {
    let mut client = create_logged_in_client().await;

    assert_eq!(client.noop().await.unwrap(), ());

    client.quit().await.unwrap();
}

#[cfg_attr(feature = "runtime-tokio", tokio::test)]
#[cfg_attr(feature = "runtime-async-std", async_std::test)]
async fn e2e_stat() {
    let mut client = create_logged_in_client().await;

    let stats = client.stat().await.unwrap();

    assert_eq!(stats.size().value().unwrap(), 0);

    client.quit().await.unwrap();
}

#[cfg_attr(feature = "runtime-tokio", tokio::test)]
#[cfg_attr(feature = "runtime-async-std", async_std::test)]
async fn e2e_list() {
    let mut client = create_logged_in_client().await;

    // let list = client.list(Some(1)).await.unwrap();

    let response = client.list(None).await.unwrap();

    match response {
        ListResponse::Multiple(list) => {
            assert_eq!(list.items().len(), 0)
        }
        _ => {
            unreachable!()
        }
    };

    client.quit().await.unwrap();
}

#[cfg_attr(feature = "runtime-tokio", tokio::test)]
#[cfg_attr(feature = "runtime-async-std", async_std::test)]
async fn e2e_capa() {
    let mut client = create_logged_in_client().await;

    let capas = client.capa().await.unwrap();

    for capa in capas {
        match capa {
            Capability::LoginDelay(time) => {
                println!("{}", time.value().unwrap().as_secs())
            }
            _ => {}
        }
    }

    client.quit().await.unwrap();
}

// #[cfg_attr(feature = "runtime-tokio", tokio::test)]
// #[cfg_attr(feature = "runtime-async-std", async_std::test)]
// async fn e2e_retr() {

//     let mut client = create_logged_in_client().await;

//     let bytes = client.retr(2).await.unwrap();

//     // println!("{}", String::from_utf8(bytes).unwrap());

//     client.quit().await.unwrap();
// }

// #[cfg_attr(feature = "runtime-tokio", tokio::test)]
// #[cfg_attr(feature = "runtime-async-std", async_std::test)]
// async fn e2e_top() {
//     let mut client = create_logged_in_client().await;

//     let bytes = client.top(3, 0).await.unwrap();

//     println!("{}", std::str::from_utf8(&bytes).unwrap());

//     client.quit().await.unwrap();
// }

#[cfg_attr(feature = "runtime-tokio", tokio::test)]
#[cfg_attr(feature = "runtime-async-std", async_std::test)]
async fn e2e_uidl() {
    let mut client = create_logged_in_client().await;

    // let uidl = client.uidl(Some(1)).await.unwrap();

    // match uidl {
    //     UidlResponse::Single(unique_id) => {
    //         println!("{}", unique_id.id());
    //     }
    //     _ => {}
    // };

    let uidl = client.uidl(None).await.unwrap();

    match uidl {
        UidlResponse::Multiple(list) => {
            assert_eq!(list.items().len(), 0)
        }
        _ => {
            unreachable!()
        }
    };

    client.quit().await.unwrap();
}
