package com.example.solodnd.ui

import android.content.Context
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import android.util.Log
import kotlinx.serialization.SerializationException

object SoloEngine {

    init {
        // Must match lib name: libsolo_engine.so -> "solo_engine"
        System.loadLibrary("solo_engine")
    }

    // These signatures must match the Rust JNI exports exactly
    external fun engineInit(campaignJson: String, characterJson: String, seed: Long)
    external fun engineCurrentView(): String
    external fun engineChoose(choiceId: String)

    @Serializable
    data class CharacterSummary(
        val name: String,
        val level: Int,
        val current_hp: Int,
        val max_hp: Int,
    )

    @Serializable
    data class ChoiceView(
        val id: String,
        val label: String,
    )

    @Serializable
    data class NodeView(
        val title: String? = null,
        val text: List<String> = emptyList(),
        val choices: List<ChoiceView> = emptyList(),
        val character_summary: CharacterSummary,
        val log: String? = null,
    )

    private val json = Json { ignoreUnknownKeys = true }

    fun parseView(raw: String): NodeView {
        return try {
            json.decodeFromString<NodeView>(raw)
    } catch (e: Exception) {
        // Log raw to logcat so we can see what the engine actually sent
        Log.e("SoloEngine", "Failed to parse NodeView from engineCurrentView()", e)
        Log.e("SoloEngine", "Raw JSON from engine: $raw")

        // Show a friendly error node instead of crashing the app
        NodeView(
            title = "Engine Error",
            text = listOf(
                "The solo engine returned data we couldn't parse.",
                e::class.simpleName ?: "Unknown exception",
                e.message ?: "No details available."
            ),
            choices = emptyList(),
            character_summary = CharacterSummary(
                name = "Error",
                level = 0,
                current_hp = 0,
                max_hp = 0,
            ),
            log = null,
        )
    }
}

    /**
     * Convenience helper used by SoloScreen:
     * load bundled JSON from assets and initialize the engine.
     */
    fun initFromAssets(context: Context) {
        val campaignJson = context.assets.open("campaigns/road_to_redcrest.json")
            .bufferedReader()
            .use { it.readText() }

        val characterJson = context.assets.open("characters/pregen_fighter.json")
            .bufferedReader()
            .use { it.readText() }

        // Deterministic seed for now
        engineInit(campaignJson, characterJson, 42L)
    }
}
