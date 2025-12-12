use std::sync::Mutex;

use lazy_static::lazy_static;
use jni::objects::{JClass, JString};
use jni::sys::{jlong, jstring};
use jni::JNIEnv;

use crate::campaign::{Campaign, NodeView};
use crate::engine::Engine;
use crate::rules::Character;

lazy_static! {
    static ref ENGINE: Mutex<Option<Engine>> = Mutex::new(None);
}

/// Kotlin: external fun engineInit(campaignJson: String, characterJson: String, seed: Long)
///
/// Signature: Java_com_example_solodnd_ui_SoloEngine_engineInit
#[no_mangle]
pub unsafe extern "system" fn Java_com_example_solodnd_ui_SoloEngine_engineInit(
    mut env: JNIEnv,
    _class: JClass,
    j_campaign_json: JString,
    j_character_json: JString,
    seed: jlong,
) {
    let campaign_json: String = env
        .get_string(&j_campaign_json)
        .expect("Invalid campaign JSON string")
        .into();

    let character_json: String = env
        .get_string(&j_character_json)
        .expect("Invalid character JSON string")
        .into();

    let campaign =
        Campaign::from_json(&campaign_json).expect("Failed to parse campaign JSON");
    let character =
        Character::from_json(&character_json).expect("Failed to parse character JSON");

    let engine = Engine::new(campaign, character, seed as u64);

    let mut guard = ENGINE.lock().unwrap();
    *guard = Some(engine);
}

/// Kotlin: external fun engineCurrentView(): String
///
/// Signature: Java_com_example_solodnd_ui_SoloEngine_engineCurrentView
#[no_mangle]
pub unsafe extern "system" fn Java_com_example_solodnd_ui_SoloEngine_engineCurrentView(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    let guard = ENGINE.lock().unwrap();
    let engine = guard.as_ref().expect("Engine not initialized");

    let view: NodeView = engine.current_view();
    let json = serde_json::to_string(&view).expect("Failed to serialize NodeView");

    env.new_string(json)
        .expect("Failed to create Java string")
        .into_raw()
}

/// Kotlin: external fun engineChoose(choiceId: String)
///
/// Signature: Java_com_example_solodnd_ui_SoloEngine_engineChoose
#[no_mangle]
pub unsafe extern "system" fn Java_com_example_solodnd_ui_SoloEngine_engineChoose(
    mut env: JNIEnv,
    _class: JClass,
    j_choice_id: JString,
) {
    let choice_id: String = env
        .get_string(&j_choice_id)
        .expect("Invalid choice id string")
        .into();

    let mut guard = ENGINE.lock().unwrap();
    let engine = guard.as_mut().expect("Engine not initialized");

    engine.choose(&choice_id);
}
