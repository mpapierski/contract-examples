#![no_std]
#![feature(alloc)]

#[macro_use]
extern crate alloc;
use alloc::vec::Vec;

extern crate common;
use common::bytesrepr::ToBytes;
use common::contract_api::call_contract;
use common::contract_api::pointers::ContractPointer;

#[no_mangle]
pub extern "C" fn call() {
    // This hash comes from a hash of address=[48; 32]; nonce=3; fn_store_id=0
    let hash = ContractPointer::Hash([
        141, 28, 213, 149, 230, 231, 61, 223, 143, 211, 37, 248, 146, 126, 101, 96, 197, 73, 164,
        185, 147, 226, 1, 127, 25, 96, 126, 228, 231, 147, 193, 252,
    ]);
    let arg = "inc";
    let _result: () = call_contract(hash.clone(), &arg, &Vec::new());
    let value: i32 = {
        let arg = "get";
        call_contract(hash, &arg, &Vec::new())
    };
    assert_eq!(value, 1);
}
