#[cfg(test)]
mod tests {
    use pocket_ic::PocketIc;

    // 2T cycles
    const INIT_CYCLES: u128 = 2_000_000_000_000;
    #[test]
    #[should_panic(expected = "is out of cycles")]
    fn test_sanity() {
        let pic = PocketIc::new();
        let canister_id = pic.create_canister();
        let wasm = b"\x00\x61\x73\x6d\x01\x00\x00\x00".to_vec();
        pic.install_canister(canister_id, wasm.clone(), vec![], None);
    }
}
