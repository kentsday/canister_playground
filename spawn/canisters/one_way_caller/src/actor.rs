use std::cell::RefCell;
use std::future::Future;

use candid::{candid_method, decode_args, decode_one, encode_args, Principal};
use ic_cdk::{api, trap};
use ic_cdk::api::call::{call_raw, CallResult};
use ic_cdk_macros::*;


#[derive(Default)]
pub struct State {
    pub counter: u64,
}
thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

async fn call_host(canister_id: &Principal, counter: u64) -> u64 {
    let args_raw = encode_args((counter, )).expect("Failed to encode arguments.");
    let result = call_raw(canister_id.clone(), "test_update", args_raw, 0).await;
    let result = result.unwrap();
    let time = decode_one::<u64>(result.as_slice()).unwrap();
    api::print(format!("result in {}", time));
    time
}

#[update(name = "test_call_with_async")]
#[candid_method(update)]
async fn test_call_with_async(ps: Principal, target: Principal) {
    // print the counter
    let counter = STATE.with(|state| {
        let state = state.borrow();
        api::print(format!("counter before: {}", state.counter));
        state.counter
    });

    call_host(&target, counter).await;

    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.counter += 1;
    });

    api::print("test_call_with_async");


    // print the counter
    STATE.with(|state| {
        let state = state.borrow();
        api::print(format!("counter after: {}", state.counter));
    });

    // running logging
    /*
    [Canister ryjl3-tyaaa-aaaaa-aaaba-cai] counter before: 0
    [Canister ryjl3-tyaaa-aaaaa-aaaba-cai] one way hoster: test_one_way called by: ryjl3-tyaaa-aaaaa-aaaba-cai, args:
        0
    [Canister ryjl3-tyaaa-aaaaa-aaaba-cai] result in 1649925875799523900
    [Canister ryjl3-tyaaa-aaaaa-aaaba-cai] test_call_with_async
    [Canister ryjl3-tyaaa-aaaaa-aaaba-cai] counter after: 1
    */
}

#[update(name = "test_call_with_spawn")]
#[candid_method(update)]
pub fn test_call_with_spawn(ps: Principal, target: Principal) {
    // print the counter
    let counter = STATE.with(|state| {
        let state = state.borrow();
        api::print(format!("counter before: {}", state.counter));
        state.counter
    });


    STATE.with(|state| {
        let mut state = state.borrow_mut();
        ic_cdk::spawn(async move {
            call_host(&target, counter).await;
        });
        state.counter += 1;
    });

    api::print("test_call_with_spawn");

    // print the counter
    STATE.with(|state| {
        let state = state.borrow();
        api::print(format!("counter after: {}", state.counter));
    });


    // running logging
    /*
    [Canister ryjl3-tyaaa-aaaaa-aaaba-cai] counter before: 7
    [Canister ryjl3-tyaaa-aaaaa-aaaba-cai] test_call_with_spawn
    [Canister ryjl3-tyaaa-aaaaa-aaaba-cai] counter after: 8
    [Canister ryjl3-tyaaa-aaaaa-aaaba-cai] one way hoster: test_one_way called by: ryjl3-tyaaa-aaaaa-aaaba-cai, args:
        7
    [Canister ryjl3-tyaaa-aaaaa-aaaba-cai] result in 1649926545448466400
    */
}

#[update(name = "test_call_with_multiple_spawn")]
#[candid_method(update)]
pub fn test_call_with_multiple_spawn(target: Principal) {
    // print the counter
    let counter = STATE.with(|state| {
        let state = state.borrow();
        api::print(format!("counter before: {}", state.counter));
        state.counter
    });

    ic_cdk::spawn(async move {
        call_host(&target, counter).await;
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            state.counter += 1;
            api::print(format!("counter after 1: {}", state.counter));
        });
    });
    ic_cdk::spawn(async move {
        call_host(&target, counter).await;
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            state.counter += 1;
            api::print(format!("counter after 2: {}", state.counter));
        });
    });

    api::print("test_call_with_multiple_spawn");

    // running logging
    /*
[Canister rrkah-fqaaa-aaaaa-aaaaq-cai] counter before: 8
[Canister rrkah-fqaaa-aaaaa-aaaaq-cai] heartbeat start 8
[Canister rrkah-fqaaa-aaaaa-aaaaq-cai] counter before 1: 9
[Canister rrkah-fqaaa-aaaaa-aaaaq-cai] result in 1649929935657414400
[Canister rrkah-fqaaa-aaaaa-aaaaq-cai] counter before 2: 10
[Canister ryjl3-tyaaa-aaaaa-aaaba-cai] one way hoster: test_one_way called by: rrkah-fqaaa-aaaaa-aaaaq-cai, args:
    3
Apr 14 09:52:48.238 WARN s:3heiq-6xfi2-szyiw-at3u2-ehlbt-x5fin-xxfmh-d6ldb-v36q3-ciscn-zqe/n:zkv27-yrl22-qipi4-eimwr-5gdtg-egv24-fm7mz-d3c34-x5ssj-mxi5f-lqe/ic_execution_environment/scheduler Finished executing message type "Request, method name test_update," on canister CanisterId(ryjl3-tyaaa-aaaaa-aaaba-cai) after 6.6923988 seconds, messaging: {"round":60,"canister_id":"ryjl3-tyaaa-aaaaa-aaaba-cai","message_id":null}
[Canister rrkah-fqaaa-aaaaa-aaaaq-cai] counter before: 10
[Canister rrkah-fqaaa-aaaaa-aaaaq-cai] heartbeat start 10
[Canister rrkah-fqaaa-aaaaa-aaaaq-cai] counter before 1: 11
[Canister rrkah-fqaaa-aaaaa-aaaaq-cai] result in 1649929936275921800
[Canister rrkah-fqaaa-aaaaa-aaaaq-cai] heartbeat end 3
    */
}


// #[heartbeat]
pub fn heartbeat_spawn() {
    // print the counter
    let counter = STATE.with(|state| {
        let state = state.borrow();
        api::print(format!("counter before: {}", state.counter));
        state.counter
    });
    api::print(format!("heartbeat start {}", counter));


    ic_cdk::spawn(async move {
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            state.counter += 1;
            api::print(format!("counter before 1: {}", state.counter));
        });

        call_host(&Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(), counter).await;
    });
    ic_cdk::spawn(async move {
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            state.counter += 1;
            api::print(format!("counter before 2: {}", state.counter));
        });
        call_host(&Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(), counter).await;
    });

    api::print(format!("heartbeat end {}", counter));

    // running logging
    /*
[Canister rrkah-fqaaa-aaaaa-aaaaq-cai] counter before: 20
[Canister rrkah-fqaaa-aaaaa-aaaaq-cai] heartbeat start 20
[Canister rrkah-fqaaa-aaaaa-aaaaq-cai] counter before 1: 21
[Canister rrkah-fqaaa-aaaaa-aaaaq-cai] counter before 2: 22
[Canister rrkah-fqaaa-aaaaa-aaaaq-cai] heartbeat end 20
[Canister rrkah-fqaaa-aaaaa-aaaaq-cai] result in 1649929489015366900
[Canister ryjl3-tyaaa-aaaaa-aaaba-cai] one way hoster: test_one_way called by: rrkah-fqaaa-aaaaa-aaaaq-cai, args:
    12
Apr 14 09:45:35.664 WARN s:y2uvh-hasgl-lg6uc-setbr-zls6u-gi5ml-k3mpp-cgydy-auboq-32b3y-2qe/n:63uud-b3tll-ravml-zvcqd-iaahf-wy7pa-dnkjo-v3z3g-67q6m-b4nkn-bqe/ic_execution_environment/scheduler Finished executing message type "Request, method name test_update," on canister CanisterId(ryjl3-tyaaa-aaaaa-aaaba-cai) after 6.6980395999999995 seconds, messaging: {"round":72,"canister_id":"ryjl3-tyaaa-aaaaa-aaaba-cai","message_id":null}
Apr 14 09:45:35.664 WARN s:y2uvh-hasgl-lg6uc-setbr-zls6u-gi5ml-k3mpp-cgydy-auboq-32b3y-2qe/n:63uud-b3tll-ravml-zvcqd-iaahf-wy7pa-dnkjo-v3z3g-67q6m-b4nkn-bqe/ic_execution_environment/scheduler At Round 72 @ time 2022-04-14 09:44:55.658452 UTC, canister rrkah-fqaaa-aaaaa-aaaaq-cai has invalid state after execution. Invariants check failed with err: Invariant broken: Canister rrkah-fqaaa-aaaaa-aaaaq-cai: Number of call contexts (5) is different than the accumulated number of reservations and responses (10), messaging: {"round":72,"canister_id":null,"message_id":null}
    */
}


#[heartbeat]
pub async fn heartbeat_async() {
    // print the counter
    let counter = STATE.with(|state| {
        let state = state.borrow();
        api::print(format!("counter before: {}", state.counter));
        state.counter
    });
    api::print(format!("heartbeat start {}", counter));

    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.counter += 1;
        api::print(format!("counter before 1: {}", state.counter));
    });

    call_host(&Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(), counter).await;

    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.counter += 1;
        api::print(format!("counter before 2: {}", state.counter));
    });
    call_host(&Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(), counter).await;

    api::print(format!("heartbeat end {}", counter));

    // running logging
    /*
[Canister rrkah-fqaaa-aaaaa-aaaaq-cai] counter before: 6
[Canister rrkah-fqaaa-aaaaa-aaaaq-cai] counter before 1: 7
[Canister rrkah-fqaaa-aaaaa-aaaaq-cai] result in 1649928833422822100
[Canister rrkah-fqaaa-aaaaa-aaaaq-cai] counter before 2: 8
[Canister ryjl3-tyaaa-aaaaa-aaaba-cai] one way hoster: test_one_way called by: rrkah-fqaaa-aaaaa-aaaaq-cai, args:
    1
Apr 14 09:34:26.219 WARN s:r65c6-kxdqy-faxla-yqb7h-dgb46-n7632-mrgak-jwm5l-b6egg-7ossf-oae/n:7mubp-goi3b-6ntl2-4c7zl-kcu2g-cesxu-ejmw5-7zt36-tpknu-xgcam-dae/ic_execution_environment/scheduler Finished executing message type "Request, method name test_update," on canister CanisterId(ryjl3-tyaaa-aaaaa-aaaba-cai) after 6.7196789 seconds, messaging: {"round":121,"canister_id":"ryjl3-tyaaa-aaaaa-aaaba-cai","message_id":null}
[Canister rrkah-fqaaa-aaaaa-aaaaq-cai] counter before: 8
[Canister rrkah-fqaaa-aaaaa-aaaaq-cai] counter before 1: 9
[Canister rrkah-fqaaa-aaaaa-aaaaq-cai] result in 1649928834040462300
[Canister rrkah-fqaaa-aaaaa-aaaaq-cai] heartbeat
    */
}