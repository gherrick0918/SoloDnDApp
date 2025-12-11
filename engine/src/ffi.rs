use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Mutex;

use lazy_static::lazy_static;

use crate::campaign::Campaign;
use crate::campaign::NodeView;
use crate::engine::Engine;
use crate::rules::Character;

lazy_static! {
    static ref ENGINE: Mutex<Option<Engine>> = Mutex::new(None);
}

#[no_mangle]
pub extern "C" fn engine_init(
    campaign_json: *const c_char,
    character_json: *const c_char,
    seed: u64,
) {
    let camp_str = unsafe { CStr::from_ptr(campaign_json) }.to_str().unwrap();
    let char_str = unsafe { CStr::from_ptr(character_json) }.to_str().unwrap();

    let campaign: Campaign = Campaign::from_json(camp_str).expect("Invalid campaign JSON");
    let character: Character = Character::from_json(char_str).expect("Invalid character JSON");

    let engine = Engine::new(campaign, character, seed);
    let mut guard = ENGINE.lock().unwrap();
    *guard = Some(engine);
}

#[no_mangle]
pub extern "C" fn engine_current_view() -> *mut c_char {
    let guard = ENGINE.lock().unwrap();
    let engine = guard.as_ref().expect("Engine not initialized");
    let view: NodeView = engine.current_view();
    let json = serde_json::to_string(&view).expect("Failed to serialize NodeView");
    CString::new(json).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn engine_choose(choice_id: *const c_char) {
    let choice = unsafe { CStr::from_ptr(choice_id) }.to_str().unwrap();
    let mut guard = ENGINE.lock().unwrap();
    let engine = guard.as_mut().expect("Engine not initialized");
    engine.choose(choice);
}

/// Helper for native side to free strings allocated by `engine_current_view`
#[no_mangle]
pub extern "C" fn engine_free_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(s);
    }
}
