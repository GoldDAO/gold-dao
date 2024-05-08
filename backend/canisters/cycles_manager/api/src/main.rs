use candid_gen::generate_candid_method;

#[allow(deprecated)]
fn main() {
    generate_candid_method!(cycles_manager, add_canister, update);
    generate_candid_method!(cycles_manager, c2c_request_cycles, update);
    generate_candid_method!(cycles_manager, update_config, update);

    candid::export_service!();
    std::print!("{}", __export_service());
}
