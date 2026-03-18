use database::{
    repositories::log_memos,
    sea_orm::{DatabaseConnection, sea_query::prelude::Utc},
};
use futures_util::future::try_join_all;
use shared::result::Rs;
use sol_lib::pumpfun;
use solana_client::rpc_response::{Response, RpcLogsResponse};
use solana_sdk::signature::Signature;

/// Processes a Solana log response and extracts/handles events
pub async fn handle_log_from_ws(
    db: &DatabaseConnection,
    res: Response<RpcLogsResponse>,
) -> Rs<Option<Signature>> {
    let signature = res.value.signature.parse()?;

    if signature == Signature::default() {
        return Ok(None);
    }

    if res.value.err.is_some() {
        tracing::trace!("Skipping failed transaction, signature {}", signature);
        return Ok(None);
    }

    let events = pumpfun::utils::Event::from_logs(&res.value.logs);

    handle_events(db, &signature, Utc::now().timestamp(), events).await?;

    Ok(Some(signature))
}

/// Processes individual blockchain events
pub async fn handle_events(
    db: &DatabaseConnection,
    signature: &Signature,
    timestamp: i64,
    events: Vec<pumpfun::utils::Event>,
) -> Rs<()> {
    let iter = events
        .into_iter()
        .enumerate()
        .map(|(log_ix, event)| handle_event(db, signature, log_ix as i32, timestamp, event));

    try_join_all(iter).await?;

    Ok(())
}

async fn handle_event(
    db: &DatabaseConnection,
    signature: &Signature,
    log_ix: i32,
    timestamp: i64,
    event: pumpfun::utils::Event,
) -> Rs<()> {
    if log_memos::is_existed(db, signature.to_string(), log_ix).await? {
        return Ok(());
    }

    tracing::info!("event: {:#?}", event);

    log_memos::save(db, signature.to_string(), log_ix, timestamp).await?;

    Ok(())
}

#[cfg(test)]
mod test {
    use sol_lib::pumpfun;

    #[test]
    fn decode_event() {
        // signature: 4jD6jHWj8As6Da65KgWHSoJgFnmR8CdQqy3EXB1NRroSTtFnkPHxixZbTaLpAckEGBPzVjoQzLvwHSjzCnds9ggS

        let logs = [
            "Program ComputeBudget111111111111111111111111111111 invoke [1]",
            "Program ComputeBudget111111111111111111111111111111 success",
            "Program ComputeBudget111111111111111111111111111111 invoke [1]",
            "Program ComputeBudget111111111111111111111111111111 success",
            "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL invoke [1]",
            "Program log: CreateIdempotent",
            "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb invoke [2]",
            "Program log: Instruction: GetAccountDataSize",
            "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb consumed 1444 of 161436 compute units",
            "Program return: TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb qgAAAAAAAAA=",
            "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb success",
            "Program 11111111111111111111111111111111 invoke [2]",
            "Program 11111111111111111111111111111111 success",
            "Program log: Initialize the associated token account",
            "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb invoke [2]",
            "Program log: Instruction: InitializeImmutableOwner",
            "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb consumed 674 of 155170 compute units",
            "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb success",
            "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb invoke [2]",
            "Program log: Instruction: InitializeAccount3",
            "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb consumed 2027 of 152161 compute units",
            "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb success",
            "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL consumed 22849 of 172700 compute units",
            "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL success",
            "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P invoke [1]",
            "Program log: Instruction: Buy",
            "Program pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ invoke [2]",
            "Program log: Instruction: GetFees",
            "Program pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ consumed 3136 of 104319 compute units",
            "Program return: pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ AAAAAAAAAABfAAAAAAAAAB4AAAAAAAAA",
            "Program pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ success",
            "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb invoke [2]",
            "Program log: Instruction: TransferChecked",
            "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb consumed 2475 of 97433 compute units",
            "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb success",
            "Program 11111111111111111111111111111111 invoke [2]",
            "Program 11111111111111111111111111111111 success",
            "Program 11111111111111111111111111111111 invoke [2]",
            "Program 11111111111111111111111111111111 success",
            "Program 11111111111111111111111111111111 invoke [2]",
            "Program 11111111111111111111111111111111 success",
            "Program data: vdt/007mYe6U8qyryHOonj/aChEPODrNn0rb6QLaALgosnVzCc+zrwA7WAgAAAAAxoekU/UAAAABDpU1lpTOu9lkGY1pDPNXNRLPI94oabmL9It5PMinovW3ZrlpAAAAAI6bRD4PAAAAIGtLjS6/AQCO7yBCCAAAACDTOEGdwAAAg4R0KS5nWpS0NuywqZiJQjKKg93GIzgClhJnxc1hF8tfAAAAAAAAAFBLFAAAAAAAGe2xhzgM8DNyCaYdVzmoQiScmIfamV4W7xts2OemxXAeAAAAAAAAAKBoBgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAwAAAGJ1eQAAAAAAAAAAAAAAAAAAAAAA",
            "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P invoke [2]",
            "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P consumed 2060 of 77691 compute units",
            "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P success",
            "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P consumed 75827 of 149851 compute units",
            "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P success",
        ];

        // let logs = [
        //     "Program ComputeBudget111111111111111111111111111111 invoke [1]",
        //     "Program ComputeBudget111111111111111111111111111111 success",
        //     "Program ComputeBudget111111111111111111111111111111 invoke [1]",
        //     "Program ComputeBudget111111111111111111111111111111 success",
        //     "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL invoke [1]",
        //     "Program log: Create",
        //     "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb invoke [2]",
        //     "Program log: Instruction: GetAccountDataSize",
        //     "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb consumed 1444 of 108902 compute units",
        //     "Program return: TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb qgAAAAAAAAA=",
        //     "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb success",
        //     "Program 11111111111111111111111111111111 invoke [2]",
        //     "Program 11111111111111111111111111111111 success",
        //     "Program log: Initialize the associated token account",
        //     "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb invoke [2]",
        //     "Program log: Instruction: InitializeImmutableOwner",
        //     "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb consumed 674 of 102636 compute units",
        //     "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb success",
        //     "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb invoke [2]",
        //     "Program log: Instruction: InitializeAccount3",
        //     "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb consumed 2027 of 99627 compute units",
        //     "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb success",
        //     "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL consumed 16811 of 114128 compute units",
        //     "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL success",
        //     "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P invoke [1]",
        //     "Program log: Instruction: Buy",
        //     "Program pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ invoke [2]",
        //     "Program log: Instruction: GetFees",
        //     "Program pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ consumed 3136 of 60886 compute units",
        //     "Program return: pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ AAAAAAAAAABfAAAAAAAAAB4AAAAAAAAA",
        //     "Program pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ success",
        //     "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb invoke [2]",
        //     "Program log: Instruction: TransferChecked",
        //     "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb consumed 2475 of 54000 compute units",
        //     "Program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb success",
        //     "Program 11111111111111111111111111111111 invoke [2]",
        //     "Program 11111111111111111111111111111111 success",
        //     "Program 11111111111111111111111111111111 invoke [2]",
        //     "Program 11111111111111111111111111111111 success",
        //     "Program 11111111111111111111111111111111 invoke [2]",
        //     "Program 11111111111111111111111111111111 success",
        //     "Program data: vdt/007mYe4DoRUqww8bKGAyN+LPzJDZCrLORYbzG2mzot5RKdXqf4GkvwcAAAAAKmYuissAAAABhjwrPWLK4FWCHHQdwZSOixLE4xOaA2DGYb5lYXVFVRG3ZrlpAAAAAGinxR8QAAAAo21de8CmAQBo+6EjCQAAAKPVSi8vqAAASsL40N1cvJfjKJwZfLUGKlTz2Va5zm5RFfllZ6pcs+ZfAAAAAAAAADnYEgAAAAAAe0IzQ5sez07XY5JJLgfTKlEl9yE70vq1B1eOUaE3jv8eAAAAAAAAAHHzBQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAwAAAGJ1eQAAAAAAAAAAAAAAAAAAAAAA",
        //     "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P invoke [2]",
        //     "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P consumed 2060 of 35758 compute units",
        //     "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P success",
        //     "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P consumed 65226 of 97317 compute units",
        //     "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P success",
        //     "Program troY36YiPGqMyAYCNbEqYCdN2tb91Zf7bHcQt7KUi61 invoke [1]",
        //     "Program log: Instruction: FeeTransfer",
        //     "Program 11111111111111111111111111111111 invoke [2]",
        //     "Program 11111111111111111111111111111111 success",
        //     "Program troY36YiPGqMyAYCNbEqYCdN2tb91Zf7bHcQt7KUi61 consumed 3506 of 32091 compute units",
        //     "Program troY36YiPGqMyAYCNbEqYCdN2tb91Zf7bHcQt7KUi61 success",
        // ];

        let event = pumpfun::utils::Event::from_logs(logs);

        dbg!(event);
    }
}
