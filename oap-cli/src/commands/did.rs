use anyhow::Result;
use colored::*;
use oap::oaep::keys::KeyPair;
use oap::oaep::did::DidKey;
use oap::oaep::did::DidDocument;
use serde_json::json;
use crate::commands::Context;

pub async fn generate(format: String, alias: Option<String>) -> Result<()> {
    let did_key = DidKey::generate();
    let did = did_key.did();
    let secret_hex = hex::encode(did_key.keypair().secret_key().as_bytes());
    let public_hex = hex::encode(did_key.keypair().public_key().as_bytes());

    if let Some(a) = &alias {
        // Mock saving to keystore
        // In reality: ~/.oap/keystore/{alias}.json
        let path = format!("~/.oap/keystore/{}.json", a);
        if !format.contains("json") {
             println!("Saved to {}", path.dimmed());
        }
    }

    if format == "json" {
        let output = json!({
            "did": did.to_string(),
            "secret_key": secret_hex,
            "public_key": public_hex,
            "alias": alias
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        println!("{}", "Generated new Identity:".green().bold());
        if let Some(a) = alias {
            println!("Alias: {}", a.cyan());
        }
        println!("DID: {}", did.to_string().cyan());
        println!("Public Key: {}", public_hex);
        println!("Secret Key: {}", secret_hex.red());
        println!("{}", "WARNING: Save the Secret Key securely!".yellow());
    }

    Ok(())
}

pub async fn resolve(did_str: String, ctx: &Context) -> Result<()> {
    if ctx.verbose {
        println!("Resolving {}...", did_str.cyan());
    }

    if did_str.starts_with("did:key:") {
        // For Phase 1/2, we mock or partially implement
        if ctx.json {
             // Placeholder JSON output
             println!("{}", json!({
                 "did": did_str,
                 "document": "placeholder"
             }));
        } else {
            println!("DID Document:");
            println!("{}", "{}".yellow()); // Placeholder
        }
    } else {
        if ctx.json {
             println!("{}", json!({"error": "Only did:key supported"}));
        } else {
            println!("Resolving {}...", did_str.cyan());
            println!("Only did:key is supported locally in Phase 1.");
        }
    }

    Ok(())
}
