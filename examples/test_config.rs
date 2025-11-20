use pane::config::{load_config, Config};

fn main() {
    println!("⚙️  Testing Configuration System\n");
    println!("=================================\n");

    // Test loading config
    match load_config() {
        Ok(config) => {
            println!("✅ Configuration loaded successfully!\n");
            println!("Config structure:");
            println!("   {:?}", config);
            println!();
        }
        Err(e) => {
            println!("ℹ️  Configuration error: {}", e);
            println!("\nThis is expected if ~/.config/pane/config.toml doesn't exist yet.");
            println!("The application will use default values.");

            // Test that default config works
            let default_config = Config::default();
            println!("\n✅ Default configuration:");
            println!("   {:?}", default_config);
        }
    }
}
