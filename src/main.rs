use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

// Constants
const SQUADS_PROGRAM_ID: &str = "SQDS4ep65T869zMMBKyuUq6aD6EgTu8psMjkvj52pCf";
const SQUADS_V3_PROGRAM_ID: &str = "SMPLecH534NA9acpos4G6x7uf3LWbCAwZQE9e8ZekMu";
const SEED_PREFIX: &[u8] = b"multisig";
const SEED_VAULT: &[u8] = b"vault";

fn main() {
    let multisig_address =
        Pubkey::from_str("4Hyrj1LraPStLyDW5fhxEkxynbUNEvegx2dbS6jZU1dX").unwrap();
    let program_id = Pubkey::from_str(SQUADS_PROGRAM_ID).unwrap();

    // Let's derive the first 10 vault addresses using the multisig address above. 10 should be enough for TVL purposes, and this
    // can be easily adjusted in the future.
    for i in 0..10 {
        let vault_address = derive_vault_address(&multisig_address, i, &program_id);
        println!("Vault address {}: {}", i, vault_address);
    }

    let v3_multisig_address =
        Pubkey::from_str("6eEtYNMzxFiy5t1mvJmJZRTFFN1Fj6fXCYt5F1ty4PAL").unwrap();
    let v3_program_id = Pubkey::from_str(SQUADS_V3_PROGRAM_ID).unwrap();

    let v3_multisig_vault = derive_vault_address_v3(&v3_multisig_address, &v3_program_id);

    println!("V3 multisig vault: {}", v3_multisig_vault);
}

/// This function derives a vault address from a multisig address and an index. In squads V4, the index 0 is always considered
/// the "default vault". The primary vault, in other terms. Squads users are allowed to create additional vaults to store their
/// funds if they want to.
fn derive_vault_address(multisig_address: &Pubkey, index: u8, program_id: &Pubkey) -> Pubkey {
    let vault_seeds = &[
        SEED_PREFIX,
        multisig_address.as_ref(),
        SEED_VAULT,
        &index.to_le_bytes(),
    ];

    let vault_address: (Pubkey, u8) = Pubkey::find_program_address(vault_seeds, program_id);
    vault_address.0
}

fn derive_vault_address_v3(multisig_address: &Pubkey, program_id: &Pubkey) -> Pubkey {
    let vault_seeds = &[
        b"squad",
        multisig_address.as_ref(),
        &1_u32.to_le_bytes(),
        b"authority",
    ];

    let vault_address: (Pubkey, u8) = Pubkey::find_program_address(vault_seeds, program_id);
    vault_address.0
}
