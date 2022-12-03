use candid::Nat;
use ic_kit::interfaces::management::{CanisterStatusResponse, DefiniteCanisterSettings, Status};
use once_cell::sync::Lazy;
use ic_cdk::{export::candid::{candid_method, CandidType, Principal}, api::{management_canister::provisional::CanisterIdRecord, call::{CallResult, call_with_payment128}}};
use serde::{Deserialize, Serialize};
use ic_cdk::api::management_canister::main;
use ic_cdk::call;

// book's info
static mut IS_USE: bool = false;

static mut BOOK_NAME: Lazy<String> = Lazy::new(|| {
    "".to_string()
});

static mut AUTHOR_NAME: Lazy<String> =Lazy::new(|| {
    "".to_string()
});

static mut BOOK_DESC: Lazy<String> = Lazy::new(|| {
    "".to_string()
});

static mut BOOK_ISN: Lazy<String> = Lazy::new(|| {
    "".to_string()
});

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Section {
    name: String,
    content: String,
}

static mut SECTION_LIST: Vec<Section> = vec![];


#[ic_cdk_macros::update]
pub fn init_book(book_name: String, author: String, desc: String) -> bool {
    unsafe {
        if !IS_USE {
            *BOOK_NAME = book_name;
            *AUTHOR_NAME = author;
            *BOOK_DESC = desc;

            IS_USE = true;
        } else {
            IS_USE = false;
        }
    };

    unsafe {
        IS_USE
    }
}

#[ic_cdk_macros::update]
pub fn add_section(name: String, content: String) {
    let section: Section = Section {
        name,
        content,
    };

    unsafe {
        SECTION_LIST.push(section);
    };
}

#[ic_cdk_macros::query]
pub fn get_section_size() -> i32 {
    unsafe {
        SECTION_LIST.len().try_into().unwrap()
    }
}

#[ic_cdk_macros::query]
pub fn get_section(index: usize) -> Section {
    unsafe {
        let ret = SECTION_LIST.get(index).clone().unwrap();
        
        Section { name: (*ret.name).to_string(), content: (*ret.content).to_string() }
    }
}

#[ic_cdk_macros::query]
pub fn get_book_name() -> String {
    unsafe {
        (*BOOK_NAME.clone()).to_string()
    }
}

#[ic_cdk_macros::query]
pub fn get_author_name() -> String {
    unsafe {
        (*AUTHOR_NAME.clone()).to_string()
    }
}

#[ic_cdk_macros::query]
pub fn get_book_desc() -> String {
    unsafe {
        (*BOOK_DESC.clone()).to_string()
    }
}

// It is necessary to revice cycles. 
// If no defined this method, will get error when despoit cycles into this canister.
#[ic_cdk_macros::update]
fn wallet_receive() -> () {
    let amount = ic_cdk::api::call::msg_cycles_available128();
    if amount > 0 {
        ic_cdk::api::call::msg_cycles_accept128(amount);
    }
}

#[ic_cdk_macros::query]
fn balance_cycles() -> u64 {
    ic_cdk::api::canister_balance()
}