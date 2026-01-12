I will rename the `mofa-settings` app to `settings` and update all related code and configurations.

### 1. Rename Directory
*   Rename directory `apps/mofa-settings` to `apps/settings`.

### 2. Update Package Configuration (Cargo.toml)
*   **apps/settings/Cargo.toml**:
    *   Change package name from `mofa-settings` to `settings`.
*   **mofa-studio-shell/Cargo.toml**:
    *   Update dependencies: Change `mofa-settings` to `settings`.
    *   Update features: Change `mofa-settings` to `settings` in default features and feature definitions.
*   **apps/mofa-fm/Cargo.toml**:
    *   Update dependencies: Change `mofa-settings` to `settings`.
*   **apps/colang/Cargo.toml**:
    *   Update dependencies: Change `mofa-settings` to `settings`.

### 3. Update Rust Code
*   **mofa-studio-shell/src/app.rs**:
    *   Replace `mofa_settings::` imports with `settings::`.
    *   Replace `MoFaSettingsApp` with `settings::MoFaSettingsApp` (or keep it as is if only the crate name changes, but better to check if I should update the struct usage if I rename the struct, but I will stick to crate rename first). *Correction*: The struct is `MoFaSettingsApp` defined in `lib.rs`, I will keep the struct name for now to minimize breaking changes, but update the module usage.
*   **apps/mofa-fm/src/screen/dora_handlers.rs**:
    *   Replace `use mofa_settings::data::Preferences` with `use settings::data::Preferences`.
*   **apps/colang/src/screen/dora_handlers.rs**:
    *   Replace `use mofa_settings::data::Preferences` with `use settings::data::Preferences`.
*   **mofa-widgets/src/app_trait.rs**:
    *   Update comments referencing `mofa_settings`.
*   **apps/settings/src/lib.rs**:
    *   Update `AppInfo` id from `"mofa-settings"` to `"settings"`.

### 4. Update Documentation
*   Update references in `README.md`, `CONTRIBUTING.md`, `ARCHITECTURE.md`, `APP_DEVELOPMENT_GUIDE.md`, and `架构指南.md` to reflect the name change.
