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

/// Initialize the engine with a campaign + character JSON and RNG seed.
///
/// # Safety
/// - `campaign_json` and `character_json` must be valid, null-terminated C strings.
/// - They must remain valid for the duration of this call.
/// - This function is intended to be called from JNI/FFI boundaries only.
#[no_mangle]
pub unsafe extern "C" fn engine_init(
    campaign_json: *const c_char,
    character_json: *const c_char,
    seed: u64,
) {
    let camp_str = CStr::from_ptr(campaign_json).to_str().unwrap();
    let char_str = CStr::from_ptr(character_json).to_str().unwrap();

    let campaign: Campaign = Campaign::from_json(camp_str).expect("Invalid campaign JSON");
    let character: Character = Character::from_json(char_str).expect("Invalid character JSON");

    let engine = Engine::new(campaign, character, seed);
    let mut guard = ENGINE.lock().unwrap();
    *guard = Some(engine);
}

/// Get the current engine view as a newly allocated C string.
///
/// # Safety
/// - The returned pointer must later be passed to `engine_free_string`.
/// - It must not be freed by any other mechanism.
/// - Returns null if initialization failed.
#[no_mangle]
pub unsafe extern "C" fn engine_current_view() -> *mut c_char {
    let guard = ENGINE.lock().unwrap();
    let engine = guard.as_ref().expect("Engine not initialized");
    let view: NodeView = engine.current_view();
    let json = serde_json::to_string(&view).expect("Failed to serialize NodeView");
    CString::new(json).unwrap().into_raw()
}

/// Apply the given choice ID to advance the engine.
///
/// # Safety
/// - `choice_id` must be a valid, null-terminated C string.
/// - Must be called only after `engine_init`.
#[no_mangle]
pub unsafe extern "C" fn engine_choose(choice_id: *const c_char) {
    let choice = CStr::from_ptr(choice_id).to_str().unwrap();
    let mut guard = ENGINE.lock().unwrap();
    let engine = guard.as_mut().expect("Engine not initialized");
    engine.choose(choice);
}

/// Free a string previously returned by the engine.
///
/// # Safety
/// - `s` must have been allocated by `engine_current_view`.
/// - Must not be used after this call.
#[no_mangle]
pub unsafe extern "C" fn engine_free_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    let _ = CString::from_raw(s);
}
