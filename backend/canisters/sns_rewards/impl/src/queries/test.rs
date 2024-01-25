use ic_cdk::query;

#[query]
fn test() -> String {
    "Hello, world!".to_string()
}
