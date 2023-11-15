#[test]
fn changeboard_go() {
	dharitri_wasm_debug::mandos_go("mandos/changeBoard.scen.json");
}

#[test]
fn changequorum_go() {
	dharitri_wasm_debug::mandos_go("mandos/changeQuorum.scen.json");
}

#[test]
fn changequorum_toobig_go() {
	dharitri_wasm_debug::mandos_go("mandos/changeQuorum_tooBig.scen.json");
}

#[test]
fn deployadder_err_go() {
	dharitri_wasm_debug::mandos_go("mandos/deployAdder_err.scen.json");
}

#[test]
fn deployadder_then_call_go() {
	dharitri_wasm_debug::mandos_go("mandos/deployAdder_then_call.scen.json");
}

#[test]
fn deployfactorial_go() {
	dharitri_wasm_debug::mandos_go("mandos/deployFactorial.scen.json");
}

#[test]
fn deploy_duplicate_bm_go() {
	dharitri_wasm_debug::mandos_go("mandos/deploy_duplicate_bm.scen.json");
}

#[test]
fn remove_everyone_go() {
	dharitri_wasm_debug::mandos_go("mandos/remove_everyone.scen.json");
}
