mod generate;

#[test]
fn generate_canister_ids_test() {
    use crate::generate::generate_canister_ids;
    println!("{:?}", generate_canister_ids());
}
