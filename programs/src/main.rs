use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::solana_sdk::signature::Signature;
use anchor_client::solana_sdk::signer::EncodableKey;
use anchor_client::{Client, Cluster, Program};
use anchor_client::{
    anchor_lang::prelude::system_program,
    solana_sdk::{commitment_config::CommitmentConfig, signature::Keypair},
};
use anchor_spl::associated_token::{get_associated_token_address, spl_associated_token_account};
use anchor_spl::token::spl_token;
use anyhow::{Result, anyhow};
use clap::Parser;
use escrow::{accounts, instruction};
use solana_cli_config::Config;
use std::{rc::Rc, str::FromStr};

#[derive(Parser)]
struct Args {
    /// Token mint A public key
    #[arg(long)]
    mint_a: String,

    /// Token mint B public key
    #[arg(long)]
    mint_b: String,

    /// Escrow program ID
    #[arg(long)]
    program_id: String,

    /// Amount of token A the maker is offering
    #[arg(long, default_value = "100")]
    offered_amount: u64,
    /// Amount of token B the maker wants in return
    #[arg(long, default_value = "200")]
    wanted_amount: u64,
}

fn make_offer(
    program: &Program<Rc<Keypair>>,
    id: u64,
    maker: Pubkey,
    token_mint_a: Pubkey,
    token_mint_b: Pubkey,
    token_a_offered_amount: u64,
    token_b_wanted_amount: u64,
) -> Result<Signature> {
    let maker_token_account_a = get_associated_token_address(&maker, &token_mint_a);

    let offer_seeds = [b"offer", maker.as_ref(), &id.to_le_bytes()];
    let (offer, _) = Pubkey::find_program_address(&offer_seeds, &program.id());

    let vault = get_associated_token_address(&offer, &token_mint_a);

    let accounts = accounts::MakeOffer {
        maker,
        token_mint_a,
        token_mint_b,
        maker_token_account_a,
        offer,
        vault,
        associated_token_program: spl_associated_token_account::ID,
        token_program: spl_token::ID,
        system_program: system_program::ID,
    };

    let args = instruction::MakeOffer {
        id,
        token_a_offered_amount,
        token_b_wanted_amount,
    };

    program
        .request()
        .accounts(accounts)
        .args(args)
        .send()
        .map_err(Into::into)
}

fn take_offer(
    program: &Program<Rc<Keypair>>,
    id: u64,
    taker: Pubkey,
    maker: Pubkey,
    token_mint_a: Pubkey,
    token_mint_b: Pubkey,
) -> Result<Signature> {
    let maker_token_account_b = get_associated_token_address(&maker, &token_mint_b);

    let taker_token_account_a = get_associated_token_address(&taker, &token_mint_a);
    let taker_token_account_b = get_associated_token_address(&taker, &token_mint_b);

    let offer_seeds = [b"offer", maker.as_ref(), &id.to_le_bytes()];
    let (offer, _) = Pubkey::find_program_address(&offer_seeds, &program.id());

    let vault = get_associated_token_address(&offer, &token_mint_a);

    let accounts = accounts::TakeOffer {
        maker,
        taker,
        token_mint_a,
        token_mint_b,
        maker_token_account_b,
        taker_token_account_a,
        taker_token_account_b,
        offer,
        vault,
        associated_token_program: spl_associated_token_account::ID,
        token_program: spl_token::ID,
        system_program: system_program::ID,
    };

    let args = instruction::TakeOffer {};

    program
        .request()
        .accounts(accounts)
        .args(args)
        .send()
        .map_err(Into::into)
}

fn get_cli() -> anyhow::Result<Client<Rc<Keypair>>> {
    let config_file = solana_cli_config::CONFIG_FILE
        .as_ref()
        .ok_or_else(|| anyhow!("unable to get config file path"))?;
    let cfg = Config::load(config_file)?;

    let wallet = Rc::new(
        Keypair::read_from_file(&cfg.keypair_path)
            .map_err(|e| anyhow!("Failed to read keypair: {}", e))?,
    );

    Ok(Client::new_with_options(
        Cluster::Devnet,
        wallet,
        CommitmentConfig::processed(),
    ))
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mint_a = Pubkey::from_str(&args.mint_a)?;
    let mint_b = Pubkey::from_str(&args.mint_b)?;
    let program_id = Pubkey::from_str(&args.program_id)?;
    let token_a_offered_amount = args.offered_amount;
    let token_b_wanted_amount = args.wanted_amount;
    // In a real application, you'd want to generate unique IDs!!!
    let id = 5;

    let client = get_cli()?;
    let program = client.program(program_id)?;

    let maker_signature = make_offer(
        &program,
        id,
        program.payer(),
        mint_a,
        mint_b,
        token_a_offered_amount,
        token_b_wanted_amount,
    )?;

    println!("Maker signature: {}", maker_signature);

    let taker_signature = take_offer(
        &program,
        id,
        program.payer(),
        program.payer(),
        mint_a,
        mint_b,
    )?;

    println!("Taker signature: {}", taker_signature);

    Ok(())
}
