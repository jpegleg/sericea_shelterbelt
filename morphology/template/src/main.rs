use std::path::Path;
use actix_files::{Files, NamedFile};
use actix_web::{App, HttpRequest, HttpServer, Responder, get, middleware};
use actix_web_lab::header::StrictTransportSecurity;
use actix_web_lab::middleware::RedirectHttps;
use chrono::prelude::*;
use log::debug;
use notify::{Event, RecursiveMode, Watcher as _};
use rustls::ServerConfig;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::PrivateKeyDer;
use rustls::pki_types::pem::PemObject;
use tokio::sync::mpsc;

#[derive(Debug)]
struct TlsUpdated;

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    debug!("{req:?}");
    NamedFile::open_async("./static/index.html").await
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> eyre::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    let (reload_tx, mut reload_rx) = mpsc::channel(1);
    let mut file_watcher =
        notify::recommended_watcher(move |res: notify::Result<Event>| match res {
            Ok(ev) => {
                log::info!("files changed: {:?}", ev.paths);
                reload_tx.blocking_send(TlsUpdated).unwrap();
            }
            Err(err) => {
                log::error!("file watch error: {err}");
            }
        })
        .unwrap();

    file_watcher
        .watch(
            Path::new("/opt/srv/TEMPLATE/cert.pem"),
            RecursiveMode::NonRecursive,
        )
        .unwrap();
    file_watcher
        .watch(
            Path::new("/opt/srv/TEMPLATE/key.pem"),
            RecursiveMode::NonRecursive,
        )
        .unwrap();

    let readi: DateTime<Utc> = Utc::now();
    log::info!("dogwood initialized at {} >>> starting TEMPLATE server", readi);

    loop {
        let tls_config = load_tls_config().expect("Failed to load TLS config");

        let mut server = HttpServer::new(|| {
            App::new()
                .wrap(RedirectHttps::default())
                .wrap(RedirectHttps::with_hsts(
                    StrictTransportSecurity::recommended(),
                ))
                .wrap(middleware::DefaultHeaders::new().add(("x-content-type-options", "nosniff")))
                .wrap(middleware::DefaultHeaders::new().add(("x-frame-options", "SAMEORIGIN")))
                .wrap(middleware::DefaultHeaders::new().add(("x-xss-protection", "1; mode=block")))
                .wrap(middleware::Logger::new(
                    "%{txid}e %a -> HTTP %s %r size: %b server-time: %T %{Referer}i %{User-Agent}i",
                ))
                .service(index)
                .service(Files::new("/", "static"))
        })
        .workers(1)
        .bind_rustls_0_23("0.0.0.0:3443", tls_config)?
        .bind("0.0.0.0:3003")?
        .run();

        let server_hnd = server.handle();

        tokio::select! {
            res = &mut server => {
                log::info!("server shutdown arrived");
                res?;
                break;
            },

            Some(_) = reload_rx.recv() => {
                log::info!("TLS cert or key updated");
                drop(server_hnd.stop(true));
                server.await?;
                continue;
            }
        }
    }

    Ok(())
}

fn load_tls_config() -> Result<ServerConfig, Box<dyn std::error::Error>> {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();

    let cert_chain = CertificateDer::pem_file_iter("cert.pem")
        .unwrap()
        .flatten()
        .collect();

    let key_der =
        PrivateKeyDer::from_pem_file("key.pem").expect("Could not locate PKCS 8 private keys.");

    let config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key_der)?;

    Ok(config)
}
