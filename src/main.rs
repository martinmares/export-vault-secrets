mod conf;

use clap::{command, value_parser, Arg};
use shell_quote::{QuoteRefExt, Sh};
use std::env;
use std::path::PathBuf;
use tracing::{error, info};
use tracing_subscriber;
use tracing_subscriber::EnvFilter;
use twelf::Layer;
use vaultrs::auth::oidc;
use vaultrs::client::{Client, VaultClient, VaultClientSettingsBuilder};
use vaultrs::kv1;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_file(true)
        .with_line_number(true)
        .init();

    let matches = command!()
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("Sets a config file")
                .value_parser(value_parser!(PathBuf))
                .required(true),
        )
        .get_matches();

    info!("Application started!");

    let config_path = if let Some(value) = matches.get_one::<PathBuf>("config") {
        value.to_owned()
    } else {
        panic!("config path must be set!")
    };

    info!("Args config path={:?}", config_path);

    let config: conf::Config;

    match conf::Config::with_layers(&[Layer::Toml(config_path.clone())]) {
        Ok(value) => {
            config = value;
        }
        Err(e) => {
            error!("Error loading config {:?}, error: {:?}", config_path, e);
            panic!("Failed to load config file with name {:?}!", config_path)
        }
    }

    // Create client
    let vault_conf = config.get_vault();
    let mut client = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address(&vault_conf.get_server_url())
            .ca_certs(vec![vault_conf.get_ssl_certs().to_string()])
            .build()
            .unwrap(),
    )
    .unwrap();

    // Auth client
    let auth_info = oidc::login(
        &client,
        &vault_conf.get_auth_login_mount(),
        &vault_conf.get_id_token(),
        Some(vault_conf.get_auth_role().to_string()),
    )
    .await;

    match auth_info {
        Ok(auth_info) => {
            client.set_token(&auth_info.client_token);
            info!("Vault client started!");

            let secrets =
                kv1::get_raw(&client, &vault_conf.get_kv_mount(), &vault_conf.get_path()).await;

            match secrets {
                Ok(secrets) => {
                    let data = secrets.data.as_object();
                    if let Some(data) = data {
                        let data_inside = data["data"].as_object();
                        if let Some(data_inside) = data_inside {
                            for key in data_inside.keys() {
                                if let Some(val) = data_inside[key].as_str() {
                                    info!("Found key \"{}\"", key);
                                    'outer: for var in config.get_vars() {
                                        if var.get_key() == key {
                                            let quoted: String = val.quoted(Sh);
                                            println!("export {}={}", var.get_export_to(), quoted);
                                            break 'outer;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Err(client_error) => {
                    error!("KV get error: {:?}", client_error);
                }
            }
        }
        Err(client_error) => {
            error!("Vault client error: {:?}", client_error);
        }
    }

    Ok(())
}
