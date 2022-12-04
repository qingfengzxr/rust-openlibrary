use ic_cdk::{
    api::call::ManualReply,
    export::candid::{CandidType, Deserialize},
};
use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::BTreeMap;

type ProfileStore = BTreeMap<String, Profile>;

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct Profile {
    pub book_name: String,
    pub description: String,
    pub sections: Vec<String>,
}

thread_local! {
    static PROFILE_STORE: RefCell<ProfileStore> = RefCell::default();
}

#[query]
fn get(key: String) -> Profile {
    PROFILE_STORE.with(|profile_store| {
        profile_store
            .borrow()
            .get(&key)
            .cloned()
            .unwrap_or_default()
    })
}

#[update]
fn update(key: String, profile: Profile) {
    PROFILE_STORE.with(|profile_store| {
        profile_store.borrow_mut().insert(key, profile);
    });
}

#[query(manual_reply = true)]
fn search(text: String) -> ManualReply<Option<Profile>> {
    let text = text.to_lowercase();
    PROFILE_STORE.with(|profile_store| {
        for (_, p) in profile_store.borrow().iter() {
            if p.book_name.to_lowercase().contains(&text) || p.description.to_lowercase().contains(&text)
            {
                return ManualReply::one(Some(p));
            }

            for x in p.sections.iter() {
                if x.to_lowercase() == text {
                    return ManualReply::one(Some(p));
                }
            }
        }
        ManualReply::one(None::<Profile>)
    })
}