# Configuration Implementation Plan

## Objective
Externalize application settings, such as bond length, to a `config.toml` file to allow user customization without recompilation.

## Steps

1.  **Dependencies**
    -   Add `toml` to `Cargo.toml`. `serde` is already present with the `derive` feature.

2.  **Configuration Structure**
    -   Create `src/config.rs`.
    -   Define `AppConfig` and sub-structs (e.g., `BondConfig`).
    -   Implement `Default` for `AppConfig` to provide fallback values.

    ```rust
    // src/config.rs
    use serde::Deserialize;

    #[derive(Deserialize, Debug, Clone)]
    pub struct AppConfig {
        #[serde(default)]
        pub bond: BondConfig,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct BondConfig {
        pub default_length: f64,
        pub default_angle_deg: f64,
    }

    impl Default for AppConfig {
        fn default() -> Self {
            Self {
                bond: BondConfig::default(),
            }
        }
    }

    impl Default for BondConfig {
        fn default() -> Self {
            Self {
                default_length: 14.4,
                default_angle_deg: 120.0,
            }
        }
    }
    ```

3.  **Loading Configuration**
    -   In `src/main.rs`, attempt to load `config.toml` from the current directory.
    -   If the file is missing or invalid, log a warning and use `AppConfig::default()`.

4.  **Integration**
    -   Update `ModeContext` in `src/modes.rs` to include a reference to `AppConfig`.
    -   Update `src/main.rs` to pass the loaded config to the `ModeContext`.
    -   Update `src/mode_handlers/bond.rs` to use `ctx.config.bond.default_length` and `ctx.config.bond.default_angle_deg` instead of hardcoded values.

5.  **Usage Example**
    -   Create a sample `config.toml` in the project root:
        ```toml
        [bond]
        default_length = 20.0
        default_angle_deg = 120.0
        ```
