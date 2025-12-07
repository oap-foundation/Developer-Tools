use anyhow::{Context as _, Result};
use colored::*;
use oap::oatp::JweContainer;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use std::fs;
use crate::commands::Context;

pub async fn decode(jwe_str: String, ctx: &Context) -> Result<()> {
    if ctx.verbose {
        println!("{}", "Decoding JWE Headers...".cyan());
    }

    let parts: Vec<&str> = jwe_str.split('.').collect();
    if parts.is_empty() {
        if ctx.json { println!("{{ \"error\": \"Invalid JWE\" }}"); }
        else { println!("{}", "Invalid JWE format".red()); }
        return Ok(());
    }

    let header_b64 = parts[0];
    let header_bytes = URL_SAFE_NO_PAD.decode(header_b64)
        .or_else(|_| base64::engine::general_purpose::STANDARD.decode(header_b64))
        .context("Failed to decode header Base64")?;

    let header_json: serde_json::Value = serde_json::from_slice(&header_bytes)
        .context("Failed to parse header JSON")?;

    if ctx.json {
        println!("{}", serde_json::to_string_pretty(&header_json)?);
    } else {
        println!("{}", serde_json::to_string_pretty(&header_json)?);
        if let Some(kid) = header_json.get("kid") {
            println!("Key ID (kid): {}", kid.to_string().green());
        }
        if let Some(alg) = header_json.get("alg") {
            println!("Algorithm (alg): {}", alg.to_string().yellow());
        }
        if let Some(enc) = header_json.get("enc") {
            println!("Encryption (enc): {}", enc.to_string().yellow());
        }
    }

    Ok(())
}

pub async fn decrypt(jwe_str: String, key_path: String, ctx: &Context) -> Result<()> {
    if ctx.verbose {
        println!("Decrypting JWE...");
    }

    let secret_bytes = if std::path::Path::new(&key_path).exists() {
        let content = fs::read_to_string(&key_path).context("Failed to read key file")?;
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(sk) = json.get("secret_key") {
                hex::decode(sk.as_str().unwrap_or(""))?
            } else {
                hex::decode(content.trim())?
            }
        } else {
            hex::decode(content.trim())?
        }
    } else {
        hex::decode(&key_path).context("Failed to decode key hex")?
    };

    if secret_bytes.len() != 32 {
        if !ctx.json { println!("{}", "Invalid secret key length".red()); }
    }

    let jwe = JweContainer::from_compact(&jwe_str).context("Failed to parse JWE")?;

    let secret_array: [u8; 32] = secret_bytes.clone().try_into()
        .map_err(|_| anyhow::anyhow!("Invalid key length"))?;

    match jwe.decrypt(&secret_array) {
        Ok(payload) => {
            if ctx.json {
                 if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&payload) {
                    println!("{}", serde_json::to_string(&json)?);
                 } else {
                    // Output as hex string in json
                    println!("{{ \"payload_hex\": \"{}\" }}", hex::encode(payload));
                 }
            } else {
                println!("{}", "Decryption Successful!".green());
                if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&payload) {
                    println!("{}", serde_json::to_string_pretty(&json)?);
                } else {
                    println!("Payload (Hex): {}", hex::encode(&payload));
                    println!("Payload (UTF8): {}", String::from_utf8_lossy(&payload));
                }
            }
        }
        Err(e) => {
            if ctx.json {
                println!("{{ \"error\": \"{}\" }}", e);
            } else {
                println!("{} {}", "Decryption Failed:".red(), e);
            }
        }
    }

    Ok(())
}
