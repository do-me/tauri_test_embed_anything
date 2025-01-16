use embed_anything::{
    config::TextEmbedConfig,
    embed_query,
    embeddings::embed::Embedder,
};
use std::sync::Arc;
//use tauri::async_runtime::block_on; // Import block_on - Removed because we don't need it anymore.
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn greet(name: &str) -> Result<String, String> { //Modified return type
    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}


#[tauri::command]
async fn generate_embedding(text: String) -> Result<String, String> {
    let embedder = Arc::new(Embedder::from_pretrained_hf(
        "bert",
        "sentence-transformers/all-MiniLM-L6-v2",
        None,
    )
    .map_err(|e| e.to_string())?);
   
    let config = TextEmbedConfig::default().with_batch_size(1);

    let embeddings = embed_query(vec![text], &embedder, Some(&config))
        .await
        .map_err(|e| e.to_string())?;

    let embedding_string = format!("{:?}", embeddings);
        Ok(embedding_string)
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, generate_embedding])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}