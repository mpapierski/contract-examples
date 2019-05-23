#![no_std]
#![feature(alloc)]

#[macro_use]
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

extern crate common;
use common::bytesrepr::ToBytes;
use common::contract_api::pointers::ContractPointer;
use common::contract_api::{call_contract, new_uref};
use common::value::Value;

#[no_mangle]
pub extern "C" fn call() {
    // This hash comes from a hash of address=[48; 32]; nonce=2; fn_store_id=0
    let hash = ContractPointer::Hash([
        141, 28, 213, 149, 230, 231, 61, 223, 143, 211, 37, 248, 146, 126, 101, 96, 197, 73, 164,
        185, 147, 226, 1, 127, 25, 96, 126, 228, 231, 147, 193, 252,
    ]);
    let arg = "World";
    let result: String = call_contract(hash, &arg, &Vec::new());
    assert_eq!("Hello, World", result);

    //store the result at a uref so it can be seen as an effect on the global state
    let _uref = new_uref(Value::String(result));
}
