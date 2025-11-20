# Database Schema

**N/A** – Pane does not use a traditional relational or NoSQL database. All persistent data is stored in human-editable text files for simplicity, transparency, and zero-dependency operation.

## Data Storage Strategy

**Configuration Data (`~/.config/pane/config.toml`)** - Stores user preferences, favorites, and recent skills list using TOML format

**Skill Manifests (`pane-skill.yaml`)** - Each skill has its own manifest file in YAML format

## Rationale for File-Based Storage

**Advantages:**
- Human-editable with any text editor
- Version control friendly (plain text)
- Zero database dependencies
- Transparent data storage
- Portable and easy to backup

**Trade-offs:**
- No query capabilities (acceptable for <1000 skills)
- No atomic transactions (mitigated by temp file + rename pattern)
- Parse overhead on load (acceptable for small files <100KB)

**Scaling Considerations:** If skill ecosystem grows beyond ~1000 skills, SQLite may be introduced for skill metadata indexing while keeping TOML/YAML for config and manifests.
