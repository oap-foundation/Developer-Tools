use anyhow::Result;
use colored::*;
use crate::commands::Context;
use oap::Agent;

pub async fn run(message: String, recipient: String, ctx: &Context) -> Result<()> {
    if ctx.verbose {
        println!("Sending message to {}...", recipient.cyan());
    }

    // Load config
    let cfg = crate::config::load(ctx.config.clone()).await?;

    // Initialize Agent
    let agent = Agent::new(&cfg.default_relay)?;

    // Send logic
    // agent.send(recipient, message).await?
    
    // Mock output for workflow example
    let relay_url = "wss://relay.local";
    let msg_id = "uuid-123";

    if ctx.json {
        println!("{{ \"status\": \"sent\", \"id\": \"{}\", \"relay\": \"{}\" }}", msg_id, relay_url);
    } else {
        println!("Sent via Relay [{}]. ID: {}", relay_url.blue(), msg_id.green());
    }

    Ok(())
}
