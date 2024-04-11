use ic_cdk::update;

#[update]
async fn read() -> Result<bool, String> {
    Ok(true)
}
