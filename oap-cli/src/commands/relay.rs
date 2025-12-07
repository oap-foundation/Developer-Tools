use anyhow::Result;
use colored::*;
use std::time::Instant;
use crate::commands::Context;

pub async fn ping(url: String, ctx: &Context) -> Result<()> {
    if ctx.verbose {
        println!("Pinging Relay at {}...", url.cyan());
    }

    let client = reqwest::Client::new();
    let start = Instant::now();
    
    let response = client.get(&url).send().await;

    let duration = start.elapsed();

    match response {
        Ok(res) => {
            if ctx.json {
                println!("{{ \"status\": {}, \"latency_ms\": {} }}", res.status().as_u16(), duration.as_millis());
            } else {
                println!("Status: {}", res.status().to_string().green());
                println!("Latency: {:.2?}", duration);
                
                if let Ok(text) = res.text().await {
                    if !text.is_empty() {
                        println!("Response: {}", text.dimmed());
                    }
                }
            }
        }
        Err(e) => {
            if ctx.json {
                println!("{{ \"error\": \"{}\" }}", e);
            } else {
                println!("{} {}", "Connection Failed:".red(), e);
            }
        }
    }

    Ok(())
}
