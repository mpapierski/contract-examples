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
        245, 250, 252, 78, 217, 1, 125, 208, 254, 74, 122, 141, 168, 215, 37, 40, 30, 168, 234, 6,
        133, 205, 117, 189, 86, 86, 186, 31, 59, 216, 223, 123,
    ]);
    let arg = "inc";
    let _result: () = call_contract(hash.clone(), &arg, &Vec::new());
    let value: i32 = {
        let arg = "get";
        call_contract(hash, &arg, &Vec::new())
    };
    assert_eq!(value, 1);
}
