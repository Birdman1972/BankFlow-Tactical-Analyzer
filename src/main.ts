import "./app.css";
import App from "./App.svelte";
import { mount } from "svelte";
import { initializePlatform } from "$lib/stores/platform";

// Initialize platform (Tauri or WASM) before mounting the app
initializePlatform()
  .then(() => {
    const app = mount(App, {
      target: document.getElementById("app")!,
    });
    (window as unknown as { app: typeof app }).app = app;
  })
  .catch((error) => {
    console.error("Failed to initialize platform:", error);
    // Show error message to user
    const appEl = document.getElementById("app");
    if (appEl) {
      appEl.innerHTML = `
        <div style="color: red; padding: 20px; text-align: center;">
          <h1>Failed to load application</h1>
          <p>${error.message}</p>
          <p>Please refresh the page or try again later.</p>
        </div>
      `;
    }
  });
