// use ic_cdk::api::management_canister::provisional::;
use ic_cdk_macros::{update};
use ic_cdk::api::{self, caller, call::call};
use ic_cdk::api::management_canister::main as mc_m;
use ic_cdk::export::candid::{candid_method, CandidType, Deserialize, Principal};
use ic_kit::{ic};
use ic_kit::interfaces::management::{InstallMode};
use ic_kit::interfaces::{management};
use ic_cdk::api::management_canister::main::{UpdateSettingsArgument, CanisterSettings};

use serde::Serialize;
use serde_bytes::ByteBuf;

const CYCLE_SHARE: u128 = 1_000_000_000_000; // 1T Cycles

pub const WASM: &[u8] = include_bytes!("../../../wasm/openlibrary_book.wasm");
// include_bytes!("../../../wasm/hello_backend.wasm");

#[ic_cdk_macros::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[derive(CandidType, Deserialize)]
pub struct InstallCodeArgumentBorrowed<'a> {
    pub mode: InstallMode,
    pub canister_id: Principal,
    #[serde(with = "serde_bytes")]
    pub wasm_module: &'a [u8],
    pub arg: Vec<u8>,
}

#[update(name = "create_book")]
#[candid_method(update, rename = "create_book")]
pub async fn create_book() -> String {
    use management::{CanisterStatus, WithCanisterId};

    let canister_settings = mc_m::CanisterSettings {
        controllers: Some(vec![api::id(), caller()]),
        // controllers: Some(vec![caller()]),
        compute_allocation: None,
        memory_allocation: None,
        freezing_threshold: None
    };

    let create_arg =  mc_m::CreateCanisterArgument{
        settings : Some(canister_settings),
    };

    let cid = mc_m::create_canister(create_arg).await.unwrap().0;
    let canister_id = cid.canister_id;

    let install_config = InstallCodeArgumentBorrowed {
        mode: InstallMode::Install,
        canister_id,
        wasm_module: WASM,
        arg: vec![],
    };

    let _: () = ic::call(
        Principal::management_canister(),
        "install_code",
        (install_config,),
    ).await.expect("Install code failed.");

    let _ = mc_m::deposit_cycles(cid, CYCLE_SHARE).await.unwrap();

    Principal::to_text(&canister_id).to_string()
}

/// Deposit cycles into the specified canister.
///
/// Note that, beyond the argument as specified in the interface description,
/// there is a second parameter `cycles` which is the amount of cycles to be deposited.
///
/// See [IC method `deposit_cycles`](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-deposit_cycles)
#[ic_cdk_macros::update]
pub async fn deposit_cycles(canister_id: Principal, cycles_amount: u128) -> bool {
    match ic_cdk::api::call::call_with_payment128(
        canister_id,
        "wallet_receive",
        {},
        cycles_amount,
    )
    .await
    {
        Ok(x) => x,
        Err((code, msg)) => {
            // print!("{}", format!(
            //     "An error happened during the call: {}: {}",
            //     code as u8, msg
            // ));
            panic!("{}", format!(
                "An error happened during the call: {}: {}",
                code as u8, msg
            ));
        }
    };

    true
}

#[ic_cdk_macros::update]
pub async fn change_book_owner(book_canister_id: Principal, new_owner: Principal) -> bool {
    // ic::call(Principal::management_canister(), "update_settings", (arg,)).await

    let update_config =  UpdateSettingsArgument{
        canister_id: book_canister_id,
        settings: CanisterSettings {
            controllers: Some(vec![new_owner]),
            compute_allocation: None,
            freezing_threshold: None,
            memory_allocation: None,
        },
    };

    match ic_cdk::api::call::call(
        Principal::management_canister(),
        "update_settings",
        (update_config,),
    ).await
    {
        Ok(x) => x,
        Err((code, msg)) => {
            // print!("{}", format!(
            //     "An error happened during the call: {}: {}",
            //     code as u8, msg
            // ));
            panic!("{}", format!(
                "An error happened during the call: {}: {}",
                code as u8, msg
            ));
        }
    }
    
    true
}