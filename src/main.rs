use std::collections::HashMap;
use std::env;
use vaultrs::auth::oidc;
use vaultrs::client::{Client, VaultClient, VaultClientSettingsBuilder};
use vaultrs::kv1;

#[tokio::main]
async fn main() {
    let vault_server = env::var("VAULT_SERVER_URL").unwrap();
    let vault_path = String::from("data/gitlab/it/tsm_group/tsm-apps/tsm-build-app/dev");
    let vault_role = Some("devops_tools_production_vault_tsm2_ro".to_string());
    let vault_id_token = env::var("VAULT_ID_TOKEN").unwrap();

    // Create a client
    let mut client = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address(vault_server)
            .ca_certs(vec!["certs/ca.pem".to_string()])
            .build()
            .unwrap(),
    )
    .unwrap();

    let auth_info = oidc::login(&client, "data", &vault_id_token, vault_role).await;

    match auth_info {
        Ok(auth_info) => {
            client.set_token(&auth_info.client_token);
            let secrets: HashMap<String, String> =
                kv1::get(&client, "data", &vault_path).await.unwrap();

            println!("client: {:?}", client.settings);
            println!("secrets: {:?}", secrets);
        }
        Err(client_error) => {
            println!("client_erorr: {:?}", client_error);
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
}
