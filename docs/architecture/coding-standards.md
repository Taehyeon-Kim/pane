# Coding Standards

## Core Standards

**Languages & Runtimes:**
- Rust 1.75+ (stable channel only)
- Edition 2021
- Toolchain managed via `rust-toolchain.toml`

**Style & Linting:**
- Formatter: `rustfmt` with default settings (enforced in CI)
- Linter: `clippy` with `-D warnings` (treat all warnings as errors)
- Line Length: 100 characters
- Imports: Grouped as std, external crates, crate modules

**Test Organization:**
- Unit tests co-located with modules using `#[cfg(test)] mod tests { ... }`
- Integration tests in separate `tests/` directory
- Test naming: `test_<function>_<scenario>_<expected_outcome>`
- Fixtures stored in `tests/fixtures/`

## Critical Rules

**MANDATORY rules that AI agents must follow:**

**1. Never use `unwrap()` or `expect()` in application code**
- Always use `?` operator or explicit error handling with `match`
- Exception: Only in tests where panic is intentional
- CLI applications must never panic on user input or runtime conditions

**2. All public functions must have doc comments**
- Use `///` doc comments explaining purpose, parameters, return values, and errors
- Format: Summary sentence, then details, then examples if complex

**3. Use `anyhow::Result<T>` for all fallible functions**
- Never use bare `Result<T, E>` in application code
- Consistent error handling and rich context propagation

**4. All state mutations must be explicit and documented**
- `AppState` modifications must happen through clearly named methods, not direct field access
- Predictable state changes, easier debugging and testing

**5. Never log sensitive information**
- Do not log file contents, environment variables (except `PANE_*`), or full user paths
- Privacy and security first

**6. Terminal state must be managed with RAII guards**
- Use guard structs that implement `Drop` for terminal mode changes
- Ensures terminal restoration even on panic or early return

**7. Use structured bindings, avoid tuple indices**
- When pattern matching or destructuring, use named bindings
- Clarity and resistance to field order changes

**8. Skill execution must validate executables exist before spawning**
- Check `skill.exec` is in PATH or is valid absolute path before `Command::new()`
- Better error messages than "file not found" after spawn

## Language-Specific Guidelines

**Rust Idioms:**
- Prefer `impl Trait` over generic type parameters for return types when possible
- Use `#[derive]` macros for common traits (`Debug`, `Clone`, `PartialEq`)
- Leverage type system: use `newtype` pattern for domain concepts
- Prefer iterators over explicit loops where readable
- Use `const` for compile-time constants, not `static`

**Async/Await:**
- Not used in MVP - Pane is synchronous by design
- If future versions need async: use `tokio` runtime, document clearly
