fn main() {
    #[cfg(target_os = "windows")]
    {
        use tauri_winres::WindowsResource;

        let mut res = WindowsResource::new();
        // English (United States) UI language keeps RC.EXE happy even with ASCII metadata.
        res.set_language(0x0409);
        if let Err(error) = res.compile() {
            eprintln!("Failed to compile Windows resources: {error:?}");
        }
    }

    tauri_build::build()
}
