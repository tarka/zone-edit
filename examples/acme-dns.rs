use std::env;

use acme_micro::DirectoryUrl;
use anyhow::Result;
use zone_edit::{gandi::{Auth, Gandi}, Config, DnsProvider};


fn get_client() -> Result<impl DnsProvider> {
    let auth = if let Some(key) = env::var("GANDI_APIKEY").ok() {
        Auth::ApiKey(key)
    } else if let Some(key) = env::var("GANDI_PATKEY").ok() {
        Auth::PatKey(key)
    } else {
        panic!("No Gandi auth key set");
    };

    let config = Config {
        domain: env::var("GANDI_TEST_DOMAIN")?,
        dry_run: false,
    };

    Ok(Gandi {
        config,
        auth,
    })
}


fn main() -> Result<()> {

    let client = get_client()?;

    Ok(())
}

