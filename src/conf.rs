#![allow(dead_code)]

use twelf::config;
use twelf::reexports::serde::{Deserialize, Serialize};

#[config]
#[derive(Debug)]
pub struct Config {
    vault: Vault,
    vars: Vec<Var>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Vault {
    server_url: String,
    auth_role: String,
    auth_login_mount: String,
    kv_mount: String,
    id_token: String,
    ssl_certs: String,
    path: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Var {
    key: String,
    export_to: String,
}

impl Config {
    pub fn get_vars(&self) -> &Vec<Var> {
        &self.vars
    }
    pub fn get_vault(&self) -> &Vault {
        &self.vault
    }
}

impl Vault {
    pub fn get_server_url(&self) -> &str {
        &self.server_url
    }
    pub fn get_auth_role(&self) -> &str {
        &self.auth_role
    }
    pub fn get_auth_login_mount(&self) -> &str {
        &self.auth_login_mount
    }
    pub fn get_kv_mount(&self) -> &str {
        &self.kv_mount
    }
    pub fn get_id_token(&self) -> &str {
        &self.id_token
    }
    pub fn get_ssl_certs(&self) -> &str {
        &self.ssl_certs
    }
    pub fn get_path(&self) -> &str {
        &self.path
    }
}

impl Var {
    pub fn get_key(&self) -> &str {
        &self.key
    }
    pub fn get_export_to(&self) -> &str {
        &self.export_to
    }
}
