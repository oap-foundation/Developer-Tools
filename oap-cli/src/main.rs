use clap::{Parser, Subcommand};
use std::path::PathBuf;
use colored::*;

mod commands;
mod config;

#[derive(Parser)]
#[command(name = "oap")]
#[command(about = "Open Agent Protocol CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, global = true)]
    config: Option<PathBuf>,

    /// Output in JSON format
    #[arg(long, global = true)]
    json: bool,

    /// Verbose mode
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// DID Operations
    Did {
        #[command(subcommand)]
        command: DidCommands,
    },
    /// Relay Operations
    Relay {
        #[command(subcommand)]
        command: RelayCommands,
    },
    /// Message Operations
    Msg {
        #[command(subcommand)]
        command: MsgCommands,
    },
    /// Initiate a Handshake
    Connect {
        /// Target DID
        did: String,
        /// Identity alias to use
        #[arg(long)]
        identity: Option<String>,
    },
    /// Listen for incoming connections
    Listen {
        /// Port or Relay URL to listen on (optional)
        #[arg(short, long)]
        port: Option<u16>,
    },
    /// Send a message
    Send {
        /// Message content
        message: String,
        /// Recipient DID
        #[arg(long)]
        recipient: String,
    },
    /// Generate shell completions
    Completions {
        /// Shell to generate completions for (bash, zsh, fish, powershell, elvish)
        shell: clap_complete::Shell,
    },
}

#[derive(Subcommand)]
enum DidCommands {
    /// Generate a new DID and keypair
    Gen {
        /// Output format (json or text)
        #[arg(short, long, default_value = "text")]
        format: String,
        /// Save identity with alias
        #[arg(long)]
        alias: Option<String>,
    },
    /// Resolve a DID
    Resolve {
        /// The DID to resolve
        did: String,
    },
}

#[derive(Subcommand)]
enum RelayCommands {
    /// Ping a Relay
    Ping {
        /// Relay URL (defaults to config)
        url: Option<String>,
    },
}

#[derive(Subcommand)]
enum MsgCommands {
    /// Decode JWE Headers (no decryption)
    Decode {
        /// Base64 encoded JWE string
        jwe: String,
    },
    /// Decrypt JWE Payload
    Decrypt {
        /// Base64 encoded JWE string
        jwe: String,
        /// Path to Secret Key file (or hex string)
        #[arg(short, long)]
        key: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Pass global flags to context or commands
    let ctx = commands::Context {
        json: cli.json,
        verbose: cli.verbose,
        config: cli.config,
    };

    match cli.command {
        Commands::Did { command } => match command {
            DidCommands::Gen { format, alias } => {
                // Override format if json flag is set
                let fmt = if ctx.json { "json".to_string() } else { format };
                commands::did::generate(fmt, alias).await?;
            }
            DidCommands::Resolve { did } => {
                commands::did::resolve(did, &ctx).await?;
            }
        },
        Commands::Relay { command } => match command {
            RelayCommands::Ping { url } => {
                let url = url.unwrap_or_else(|| "http://localhost:3000".to_string());
                commands::relay::ping(url, &ctx).await?;
            }
        },
        Commands::Msg { command } => match command {
            MsgCommands::Decode { jwe } => {
                commands::msg::decode(jwe, &ctx).await?;
            }
            MsgCommands::Decrypt { jwe, key } => {
                commands::msg::decrypt(jwe, key, &ctx).await?;
            }
        },
        Commands::Connect { did, identity } => {
            commands::connect::run(did, identity, &ctx).await?;
        }
        Commands::Listen { port } => {
            commands::listen::run(port, &ctx).await?;
        }
        Commands::Send { message, recipient } => {
            commands::send::run(message, recipient, &ctx).await?;
        }
        Commands::Completions { shell } => {
            use clap::CommandFactory;
            let mut cmd = Cli::command();
            let bin_name = cmd.get_name().to_string();
            clap_complete::generate(shell, &mut cmd, bin_name, &mut std::io::stdout());
        }
    }

    Ok(())
}
