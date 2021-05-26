use crate::utils;

use console::style;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use indicatif::{ProgressBar, ProgressStyle};

use crate::models::{ClientToken, TomlDump};
use std::io::Write;
use std::str;
use std::sync::Arc;
use std::time::Duration;

type AnyError = Box<dyn std::error::Error + Send + Sync>;

pub fn log_number_action(number: i8, message: &str) {
    println!(
        "{} {}",
        style(format!("[{}/4]", number)).bold().dim(),
        message
    );
}

pub fn run() -> anyhow::Result<()> {
    let out: bool = utils::confirm("Can we open dagpi's website to facilitate a login")?;
    if out {
        utils::open_browser("https://dagpi.xyz/tokens?cli_redirect=true")?;
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        let local = tokio::task::LocalSet::new();
        let out = local.block_on(&rt, start_future());
        let r_opt = match out {
            Ok(r) => r,
            Err(_e) => {
                println!("{}", style("Error getting Payload").red());
                println!();
                return Ok(());
            }
        };
        log_number_action(1, "Parsing Recieved Payload");
        let r = match r_opt {
            Some(res) => res,
            None => {
                println!("{}", style("Payload had no data").red());
                println!();
                return Ok(());
            }
        };
        let t = match serde_json::from_str::<ClientToken>(&r) {
            Ok(t) => t,
            Err(_e) => {
                println!("{}", style("Error parsing token from response").red());
                println!();
                return Ok(());
            }
        };
        log_number_action(2, "Parsed Payload into Token");
        let mut d = match dirs::config_dir() {
            Some(d) => d,
            None => {
                println!("{}", style("Unable to find config directory").red());
                println!();
                return Ok(());
            }
        };
        d.push("dagpi");
        match std::fs::create_dir(d.clone()) {
            Ok(_e) => println!("Created Directory!"),
            Err(e) => {
                if e.kind() != std::io::ErrorKind::AlreadyExists {
                    println!(
                        "{}",
                        style(format!("Unable to make directory as {:?}", e)).red()
                    );
                    return Ok(());
                }
            }
        }
        log_number_action(3, "Located and opened config directory");
        d.push("dagpi.toml");
        let mut file = match std::fs::File::create(d.clone()) {
            Ok(e) => e,
            Err(e) => {
                println!(
                    "{}\n{}",
                    style(format!(
                        "Error creating `dagpi.toml` in {:?} with error:",
                        d
                    ))
                    .red(),
                    e.to_string()
                );
                println!();
                return Ok(());
            }
        };
        log_number_action(
            4,
            &*format!("Overwrote `dagpi.toml` in {}", d.to_str().unwrap()),
        );
        let toml_data = match toml::to_string(&TomlDump { cli_token: t }) {
            Ok(y) => y,
            Err(_e) => {
                println!("{}", style("Could not encode payload to toml").red());
                println!();
                return Ok(());
            }
        };
        match file.write_all(toml_data.as_bytes()) {
            Ok(_s) => {
                println!(
                    "{}",
                    style("Success! Wrote all of your settings to the config file!").green()
                );
                println!();
            }
            Err(_e) => {
                println!("{}\n{}", style("Could not encode payload to toml. Please find file at path above and save the following:").red(), toml_data);
                println!();
            }
        };
    } else {
        println!("Aborting login....")
    };
    Ok(())
}

async fn start_future() -> Result<Option<String>, AnyError> {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(1);
    let server_task = tokio::spawn(start_server(tx));
    let result = rx.recv().await;
    server_task.abort();
    Ok(result)
}

async fn start_server(tx: tokio::sync::mpsc::Sender<String>) -> Result<(), AnyError> {
    let addr = ([127, 0, 0, 1], 3127).into();
    let tx = Arc::new(tx);
    let (tx_kill, mut rx_kill) = tokio::sync::mpsc::channel::<()>(1);
    let make_service = make_service_fn(move |_| {
        let tx = tx.clone();

        async move {
            let o = service_fn(move |req: Request<Body>| echo(req, tx.clone()));
            Ok::<_, hyper::Error>(o)
        }
    });
    let server = Server::bind(&addr).serve(make_service);
    let graceful = server.with_graceful_shutdown(async {
        rx_kill.recv().await.unwrap();
    });
    tokio::spawn(async move {
        let tx = tx_kill.clone();
        let style_spinner = ProgressStyle::default_spinner().template("{spinner}   {msg}");
        let spinner = ProgressBar::new_spinner().with_style(style_spinner);
        spinner.set_message("Waiting for API token...");
        spinner.enable_steady_tick(20);
        tokio::time::sleep(Duration::from_secs(600)).await;
        spinner.abandon();
        println!(
            "{}",
            style("Server Timeout no api token was posted to /cli_token. Exiting").cyan()
        );
        tx.send(()).await.unwrap();
    });
    println!("Started Server on http://{}", addr);
    Ok(graceful.await?)
}

async fn echo(
    req: Request<Body>,
    tx: Arc<tokio::sync::mpsc::Sender<String>>,
) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => {
            let mut r = Response::new(Body::from("{\"message\": \"This server is waiting for you to post yur token. Visit the Website that has been opened and follow the instructions. \"}"));
            r.headers_mut()
                .insert("Content-Type", "application/json".parse().unwrap());
            Ok(r)
        }
        (&Method::POST, "/cli_token") => {
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;
            let str_body = str::from_utf8(&whole_body).unwrap();
            let reversed_body = whole_body.iter().rev().cloned().collect::<Vec<u8>>();
            tx.send(str_body.to_string()).await.unwrap();
            Ok(Response::new(Body::from(reversed_body)))
        }
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}
