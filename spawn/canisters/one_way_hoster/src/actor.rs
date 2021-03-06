use candid::{candid_method, Principal};
use ic_cdk::api;
use ic_cdk_macros::*;

#[update(name = "test_update")]
#[candid_method(update)]
pub fn test_update(index: u64) -> u64 {
    let caller = api::caller();

    // try to sleep 3 seconds
    let mut vec = vec![1, 2, 3];
    // add item and remove item from the vector for 1000_000 times
    for _ in 0..1000_000_000 {
        vec.push(4);
        vec.pop();
    }

    api::print(format!("one way hoster: test_one_way called by: {}, args:
    {}", caller, index));
    api::time()
}

candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
#[candid_method(query, rename = "__get_candid_interface_tmp_hack")]
fn __export_did_tmp_() -> String {
    __export_service()
}
