const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;
let embeddingInputEl;
let embeddingMsgEl;


async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}
async function generateEmbedding() {
    embeddingMsgEl.textContent = "Generating embedding...";
    try {
      const embedding = await invoke("generate_embedding", { text: embeddingInputEl.value });
      embeddingMsgEl.textContent = "Embedding: " + embedding;

    } catch (error) {
      embeddingMsgEl.textContent = "Error: " + error;
    }
  }
  

window.addEventListener("DOMContentLoaded", () => {
    greetInputEl = document.querySelector("#greet-input");
    greetMsgEl = document.querySelector("#greet-msg");

    embeddingInputEl = document.querySelector("#embedding-input");
    embeddingMsgEl = document.querySelector("#embedding-msg");
  
    document.querySelector("#greet-form").addEventListener("submit", (e) => {
      e.preventDefault();
      greet();
    });
  
    document.querySelector("#embedding-form").addEventListener("submit", (e) => {
        e.preventDefault();
        generateEmbedding();
    });
  });