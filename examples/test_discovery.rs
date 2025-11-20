use pane::skills::discover_skills;

fn main() {
    println!("🔍 Testing Skill Discovery System\n");
    println!("=================================\n");

    match discover_skills() {
        Ok(skills) => {
            println!("✅ Discovered {} skill(s):\n", skills.len());

            for skill in &skills {
                println!("📦 Skill: {}", skill.manifest.name);
                println!("   ID: {}", skill.manifest.id);
                println!("   Description: {}", skill.manifest.description);
                println!("   Source: {:?}", skill.source);
                println!("   Path: {:?}", skill.manifest_path);
                println!("   Exec: {}", skill.manifest.exec);
                println!("   Version: {}", skill.manifest.version);
                println!();
            }

            if skills.is_empty() {
                println!("ℹ️  No skills found in:");
                println!("   - ./.pane/skills/ (project)");
                println!("   - ~/.config/pane/skills/ (user)");
                println!("   - /usr/local/share/pane/skills/ (system)");
            }
        }
        Err(e) => {
            eprintln!("❌ Error discovering skills: {}", e);
            std::process::exit(1);
        }
    }
}
