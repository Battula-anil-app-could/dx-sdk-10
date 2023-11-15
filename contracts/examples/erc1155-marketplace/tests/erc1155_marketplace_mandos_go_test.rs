#[test]
fn auction_batch_go() {
	dharitri_wasm_debug::mandos_go("mandos/auction_batch.scen.json");
}

#[test]
fn auction_single_token_moax_go() {
	dharitri_wasm_debug::mandos_go("mandos/auction_single_token_moax.scen.json");
}

#[test]
fn bid_first_moax_go() {
	dharitri_wasm_debug::mandos_go("mandos/bid_first_moax.scen.json");
}

#[test]
fn bid_second_moax_go() {
	dharitri_wasm_debug::mandos_go("mandos/bid_second_moax.scen.json");
}

#[test]
fn bid_third_moax_go() {
	dharitri_wasm_debug::mandos_go("mandos/bid_third_moax.scen.json");
}

#[test]
fn end_auction_go() {
	dharitri_wasm_debug::mandos_go("mandos/end_auction.scen.json");
}
