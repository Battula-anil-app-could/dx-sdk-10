use dharitri_wasm::*;
use dharitri_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/adder.wasm",
		Box::new(|context| Box::new(adder::contract_obj(context))),
	);
	contract_map
}

#[test]
fn adder_rs() {
	dharitri_wasm_debug::mandos_rs("mandos/adder.scen.json", &contract_map());
}
