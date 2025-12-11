package com.example.solodnd.ui

import android.content.Context
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Button
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.unit.dp

@Composable
fun SoloScreen() {
    val context = LocalContext.current
    var view by remember { mutableStateOf<SoloEngine.NodeView?>(null) }

    LaunchedEffect(Unit) {
        // Replace this with loading from assets / resources
        val campaignJson = loadAsset(context, "campaigns/road_to_redcrest.json")
        val characterJson = loadAsset(context, "characters/pregen_fighter.json")
        SoloEngine.engineInit(campaignJson, characterJson, System.currentTimeMillis())
        val raw = SoloEngine.engineCurrentView()
        view = SoloEngine.parseView(raw)
    }

    view?.let { node ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(16.dp)
        ) {
            Column(
                modifier = Modifier
                    .weight(1f)
                    .verticalScroll(rememberScrollState())
            ) {
                node.title?.let {
                    Text(text = it)
                    Spacer(Modifier.height(8.dp))
                }
                node.text.forEach { para ->
                    Text(text = para)
                    Spacer(Modifier.height(4.dp))
                }
                Spacer(Modifier.height(16.dp))
                Text(
                    text = "${'$'}{node.character_summary.name} " +
                            "Lv ${'$'}{node.character_summary.level} " +
                            "HP ${'$'}{node.character_summary.current_hp}/${'$'}{node.character_summary.max_hp}"
                )
                node.log?.let {
                    Spacer(Modifier.height(8.dp))
                    Text(text = it)
                }
            }

            Spacer(Modifier.height(16.dp))

            node.choices.forEach { choice ->
                Button(
                    onClick = {
                        SoloEngine.engineChoose(choice.id)
                        val raw = SoloEngine.engineCurrentView()
                        view = SoloEngine.parseView(raw)
                    },
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(vertical = 4.dp)
                ) {
                    Text(choice.label)
                }
            }
        }
    } ?: Box(
        modifier = Modifier.fillMaxSize(),
        contentAlignment = Alignment.Center
    ) {
        CircularProgressIndicator()
    }
}

private fun loadAsset(context: Context, path: String): String {
    return context.assets.open(path).bufferedReader().use { it.readText() }
}
