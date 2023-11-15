use dharitri_wasm::*;
use dharitri_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/crypto-bubbles.wasm",
		Box::new(|context| Box::new(crypto_bubbles::contract_obj(context))),
	);
	contract_map
}

#[test]
fn balanceof_rs() {
	dharitri_wasm_debug::mandos_rs("mandos/balanceOf.scen.json", &contract_map());
}

#[test]
fn create_rs() {
	dharitri_wasm_debug::mandos_rs("mandos/create.scen.json", &contract_map());
}

#[test]
fn exceptions_rs() {
	dharitri_wasm_debug::mandos_rs("mandos/exceptions.scen.json", &contract_map());
}

#[test]
fn joingame_rs() {
	dharitri_wasm_debug::mandos_rs("mandos/joinGame.scen.json", &contract_map());
}

#[test]
fn rewardandsendtowallet_rs() {
	dharitri_wasm_debug::mandos_rs("mandos/rewardAndSendToWallet.scen.json", &contract_map());
}

#[test]
fn rewardwinner_rs() {
	dharitri_wasm_debug::mandos_rs("mandos/rewardWinner.scen.json", &contract_map());
}

#[test]
fn rewardwinner_last_rs() {
	dharitri_wasm_debug::mandos_rs("mandos/rewardWinner_Last.scen.json", &contract_map());
}

#[test]
fn topup_ok_rs() {
	dharitri_wasm_debug::mandos_rs("mandos/topUp_ok.scen.json", &contract_map());
}

#[test]
fn topup_withdraw_rs() {
	dharitri_wasm_debug::mandos_rs("mandos/topUp_withdraw.scen.json", &contract_map());
}

#[test]
fn withdraw_ok_rs() {
	dharitri_wasm_debug::mandos_rs("mandos/withdraw_Ok.scen.json", &contract_map());
}

#[test]
fn withdraw_toomuch_rs() {
	dharitri_wasm_debug::mandos_rs("mandos/withdraw_TooMuch.scen.json", &contract_map());
}
