#[test]
fn balanceof_go() {
	dharitri_wasm_debug::mandos_go("mandos/balanceOf.scen.json");
}

#[test]
fn create_go() {
	dharitri_wasm_debug::mandos_go("mandos/create.scen.json");
}

#[test]
fn exceptions_go() {
	dharitri_wasm_debug::mandos_go("mandos/exceptions.scen.json");
}

#[test]
fn joingame_go() {
	dharitri_wasm_debug::mandos_go("mandos/joinGame.scen.json");
}

#[test]
fn rewardandsendtowallet_go() {
	dharitri_wasm_debug::mandos_go("mandos/rewardAndSendToWallet.scen.json");
}

#[test]
fn rewardwinner_go() {
	dharitri_wasm_debug::mandos_go("mandos/rewardWinner.scen.json");
}

#[test]
fn rewardwinner_last_go() {
	dharitri_wasm_debug::mandos_go("mandos/rewardWinner_Last.scen.json");
}

#[test]
fn topup_ok_go() {
	dharitri_wasm_debug::mandos_go("mandos/topUp_ok.scen.json");
}

#[test]
fn topup_withdraw_go() {
	dharitri_wasm_debug::mandos_go("mandos/topUp_withdraw.scen.json");
}

#[test]
fn withdraw_ok_go() {
	dharitri_wasm_debug::mandos_go("mandos/withdraw_Ok.scen.json");
}

#[test]
fn withdraw_toomuch_go() {
	dharitri_wasm_debug::mandos_go("mandos/withdraw_TooMuch.scen.json");
}
