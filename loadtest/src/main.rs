use goose::prelude::*;

async fn loadtest_index(user: &mut GooseUser) -> TransactionResult {
    let _goose_metrics = user.get("/?key=QXlj").await?;

    Ok(())
}

async fn loadtest_all(user: &mut GooseUser) -> TransactionResult {
    let _goose_metrics = user.get("/?key=QXlj").await?;
    Ok(())
}

async fn loadtest_query(user: &mut GooseUser) -> TransactionResult {
    let _goose_metrics = user.get("/1/?key=QXlj").await?;
    Ok(())
}

async fn loadtest_fill(user: &mut GooseUser) -> TransactionResult {
    let json = &serde_json::json!(vec![1, 2, 3, 4, 7, 8, 9, 10]);
    for i in 1..50 {
        let _goose_metrics = user.post_json(&format!("/{}/?key=QXlj", i), &json).await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    GooseAttack::initialize()?
        .register_scenario(
            scenario!("LoadtestTransactions")
                //  .register_transaction(transaction!(loadtest_index))
                .register_transaction(transaction!(loadtest_fill).set_on_start())
                .register_transaction(transaction!(loadtest_query))
                .register_transaction(transaction!(loadtest_all)),
        )
        .execute()
        .await?;

    Ok(())
}
