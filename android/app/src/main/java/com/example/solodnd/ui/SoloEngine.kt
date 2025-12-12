package com.example.solodnd.ui

import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json

object SoloEngine {
    init {
        // IMPORTANT: this must be uncommented
        System.loadLibrary("solo_engine")
    }

    external fun engineInit(campaignJson: String, characterJson: String, seed: Long)
    external fun engineCurrentView(): String
    external fun engineChoose(choiceId: String)

    @Serializable
    data class ChoiceView(val id: String, val label: String)

    @Serializable
    data class CharacterSummary(
        val name: String,
        val level: UByte,
        val current_hp: Int,
        val max_hp: Int
    )

    @Serializable
    data class NodeView(
        val title: String? = null,
        val text: List<String> = emptyList(),
        val choices: List<ChoiceView> = emptyList(),
        val character_summary: CharacterSummary,
        val log: String? = null
    )

    private val json = Json { ignoreUnknownKeys = true }

    fun parseView(raw: String): NodeView = json.decodeFromString(raw)
}
