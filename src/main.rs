use std::collections::HashMap;
use std::env;
use tracing::*;
use vaultrs::auth::oidc;
use vaultrs::client::{Client, VaultClient, VaultClientSettingsBuilder};
use vaultrs::kv1;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // let vault_server = env::var("VAULT_SERVER_URL").unwrap();
    let vault_server = "https://devops.cetin:8200";
    let vault_path = String::from("kv/data/gitlab/it/tsm_group/tsm-apps/tsm-build-app/dev");
    let vault_role = Some(env::var("VAULT_AUTH_ROLE").unwrap());
    // let vault_role = Some("devops_tools_production_vault_tsm2_ro".to_string());
    let vault_id_token = env::var("VAULT_ID_TOKEN").unwrap();
    // let vault_id_token = "eyJraWQiOiJJWDFYUTNJLXdpenlpZ1BiWDlTa25lQjROSUUxem5HNGMzRUhSbmJ1MTkwIiwidHlwIjoiSldUIiwiYWxnIjoiUlMyNTYifQ.eyJuYW1lc3BhY2VfaWQiOiIxMjMiLCJuYW1lc3BhY2VfcGF0aCI6Iml0L3RzbV9ncm91cC90c20tYXBwcyIsInByb2plY3RfaWQiOiIxMjAiLCJwcm9qZWN0X3BhdGgiOiJpdC90c21fZ3JvdXAvdHNtLWFwcHMvdHNtLWJ1aWxkLWFwcCIsInVzZXJfaWQiOiI0OSIsInVzZXJfbG9naW4iOiJ4MDU0OTk4MyIsInVzZXJfZW1haWwiOiJtYXJ0aW4ubWFyZXNAY2V0aW4uY3oiLCJwaXBlbGluZV9pZCI6IjE3MjQyNiIsInBpcGVsaW5lX3NvdXJjZSI6InB1c2giLCJqb2JfaWQiOiI1NTAzMzciLCJyZWYiOiJ2YXVsdF9jbGllbnQiLCJyZWZfdHlwZSI6ImJyYW5jaCIsInJlZl9wYXRoIjoicmVmcy9oZWFkcy92YXVsdF9jbGllbnQiLCJyZWZfcHJvdGVjdGVkIjoiZmFsc2UiLCJydW5uZXJfaWQiOjIsInJ1bm5lcl9lbnZpcm9ubWVudCI6InNlbGYtaG9zdGVkIiwic2hhIjoiOTg2MzNhYTNiN2IzNjNlZjU0M2RiNmFhMmQ5MmI5N2I5N2M3YzVkYSIsInByb2plY3RfdmlzaWJpbGl0eSI6InByaXZhdGUiLCJjaV9jb25maWdfcmVmX3VyaSI6ImRldm9wcy5jZXRpbi9pdC90c21fZ3JvdXAvdHNtLWFwcHMvdHNtLWJ1aWxkLWFwcC8vLmdpdGxhYi1jaS55bWxAcmVmcy9oZWFkcy92YXVsdF9jbGllbnQiLCJjaV9jb25maWdfc2hhIjoiOTg2MzNhYTNiN2IzNjNlZjU0M2RiNmFhMmQ5MmI5N2I5N2M3YzVkYSIsImp0aSI6Ijc5NzYzMTVlLTU4ZTYtNGMxYS04YjJkLWY1ZTc4Y2EzNTA1MCIsImlzcyI6Imh0dHBzOi8vZGV2b3BzLmNldGluIiwiaWF0IjoxNzE3NjIxMzI4LCJuYmYiOjE3MTc2MjEzMjMsImV4cCI6MTcxNzYyNDkyOCwic3ViIjoicHJvamVjdF9wYXRoOml0L3RzbV9ncm91cC90c20tYXBwcy90c20tYnVpbGQtYXBwOnJlZl90eXBlOmJyYW5jaDpyZWY6dmF1bHRfY2xpZW50IiwiYXVkIjoiaHR0cHM6Ly9kZXZvcHMuY2V0aW46ODIwMCJ9.RT-xTlxrDulXl1pnZZkLEvgxVEhw3GW5j74qDOMARnLDqDJtzSe_-3gqqNHgAWnC_4wdCNlbiAdi0L02W-gaNsdUdxSlHrtSFd1qmpAQJc0SzU_dYJxW9rrjZEbxJnOUmlJvMaT0tSTv2rV9HuyvF_Hord2LYcofS1e4pjZLykS4eHj4uR1x31aRx_L1y8TcF0B0UXUqzmqbwhQfMYfsfvUyKZ6i7hjLBB9MS2OfPUK6OHL__Hqwj1uAg7r57Y5dk9cx6XmHSnvRQ8Bia-PcoTK-3s2647LI7rviW3SHVEfwyIxFyUajYcOuED4k9-ZsJTugu1wbFrYP6EDpVKVUCw";

    info!("vault_server: {}", vault_server);
    info!("vault_path: {}", vault_path);
    info!("vault_role: {}", vault_role.clone().unwrap());
    info!("vault_id_token: {}", vault_id_token);

    // Create a client
    let mut client = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address(vault_server)
            .ca_certs(vec!["certs/ca.pem".to_string()])
            .build()
            .unwrap(),
    )
    .unwrap();

    let auth_info = oidc::login(&client, &"oidc", &vault_id_token, vault_role).await;

    match auth_info {
        Ok(auth_info) => {
            client.set_token(&auth_info.client_token);
            let secrets: HashMap<String, String> = kv1::get(&client, "", &vault_path).await?;

            info!("client: {:?}", client.settings);
            info!("secrets: {:?}", secrets)
        }
        Err(client_error) => {
            error!("client_error: {:?}", client_error);
        }
    }

    // if let Ok(auth_info) = auth_info {
    //     client.set_token(&auth_info.client_token);
    //     let secrets: HashMap<String, String> =
    //         kv1::get(&client, "data", &vault_path).await.unwrap();

    //     println!("client: {:?}", client.settings);
    //     println!("secrets: {:?}", secrets);

    //     // println!("LDAP_USER_NAME: {:?}", read_secrets["LDAP_USER_NAME"]);
    //     // println!(
    //     //     "LDAP_USER_PASSWORD: {:?}",
    //     //     read_secrets["LDAP_USER_PASSWORD"]
    //     // );
    // }

    Ok(())
}
