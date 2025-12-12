use std::sync::Mutex;
use std::ptr;

use lazy_static::lazy_static;
use jni::objects::{JObject, JString};
use jni::sys::{jlong, jstring};
use jni::JNIEnv;

use crate::campaign::{Campaign, NodeView};
use crate::engine::Engine;
use crate::rules::Character;

lazy_static! {
    static ref ENGINE_JNI: Mutex<Option<Engine>> = Mutex::new(None);
}

fn init_engine_internal(
    campaign_json: &str,
    character_json: &str,
    seed: u64,
) -> Result<(), String> {
    let campaign: Campaign = serde_json::from_str(campaign_json)
        .map_err(|e| format!("Invalid campaign JSON: {e}"))?;
    let character: Character = serde_json::from_str(character_json)
        .map_err(|e| format!("Invalid character JSON: {e}"))?;

    let engine = Engine::new(campaign, character, seed);

    let mut guard = ENGINE_JNI
        .lock()
        .map_err(|e| format!("Engine mutex poisoned: {e}"))?;
    *guard = Some(engine);
    Ok(())
}

fn current_view_internal() -> Result<String, String> {
    let guard = ENGINE_JNI
        .lock()
        .map_err(|e| format!("Engine mutex poisoned: {e}"))?;
    let engine = guard
        .as_ref()
        .ok_or_else(|| "Engine not initialized".to_string())?;
    let view: NodeView = engine.current_view();
    serde_json::to_string(&view).map_err(|e| format!("Failed to serialize view: {e}"))
}

fn choose_internal(choice_id: &str) -> Result<(), String> {
    let mut guard = ENGINE_JNI
        .lock()
        .map_err(|e| format!("Engine mutex poisoned: {e}"))?;
    let engine = guard
        .as_mut()
        .ok_or_else(|| "Engine not initialized".to_string())?;
    engine.choose(choice_id);
    Ok(())
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_solodnd_ui_SoloEngine_engineInit(
    mut env: JNIEnv,
    _this: JObject,               // instance method receiver (Kotlin `object`)
    campaign_json: JString,
    character_json: JString,
    seed: jlong,
) {
    let camp: String = match env.get_string(&campaign_json) {
        Ok(s) => s.into(),
        Err(e) => {
            let _ = env.throw_new(
                "java/lang/RuntimeException",
                format!("Failed to read campaign_json: {e}"),
            );
            return;
        }
    };

    let chara: String = match env.get_string(&character_json) {
        Ok(s) => s.into(),
        Err(e) => {
            let _ = env.throw_new(
                "java/lang/RuntimeException",
                format!("Failed to read character_json: {e}"),
            );
            return;
        }
    };

    if let Err(err) = init_engine_internal(&camp, &chara, seed as u64) {
        let _ = env.throw_new("java/lang/RuntimeException", err);
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_solodnd_ui_SoloEngine_engineCurrentView(
    mut env: JNIEnv,
    _this: JObject,
) -> jstring {
    match current_view_internal() {
        Ok(json) => match env.new_string(json) {
            Ok(java_str) => java_str.into_raw(),
            Err(e) => {
                let _ = env.throw_new(
                    "java/lang/RuntimeException",
                    format!("Failed to create Java string: {e}"),
                );
                ptr::null_mut()
            }
        },
        Err(err) => {
            let _ = env.throw_new("java/lang/IllegalStateException", err);
            ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_example_solodnd_ui_SoloEngine_engineChoose(
    mut env: JNIEnv,
    _this: JObject,
    choice_id: JString,
) {
    let choice: String = match env.get_string(&choice_id) {
        Ok(s) => s.into(),
        Err(e) => {
            let _ = env.throw_new(
                "java/lang/RuntimeException",
                format!("Failed to read choice_id: {e}"),
            );
            return;
        }
    };

    if let Err(err) = choose_internal(&choice) {
        let _ = env.throw_new("java/lang/RuntimeException", err);
    }
}
