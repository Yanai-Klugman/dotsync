pub const APP_NAME: &str = "DotSync";
pub const APP_VERSION: &str = "0.1.0";
pub const APP_AUTHOR: &str = "Your Name <your.email@example.com>";
pub const APP_ABOUT: &str = "A CLI tool for managing and synchronizing dotfiles";

// Subcommands
pub const SUBCOMMAND_SYNC: &str = "sync";
pub const SUBCOMMAND_SETUP: &str = "setup";
pub const SUBCOMMAND_RESET: &str = "reset";
pub const SUBCOMMAND_UPDATE: &str = "update";
pub const SUBCOMMAND_SWITCH: &str = "switch";
pub const SUBCOMMAND_LIST_CONFIGS: &str = "list-configs";
pub const SUBCOMMAND_CREATE_PROFILE: &str = "create-profile";
pub const SUBCOMMAND_ROLLBACK: &str = "rollback";

// Arguments
pub const ARG_PROFILE: &str = "profile";
pub const ARG_DRY_RUN: &str = "dry-run";

// Descriptions
pub const DESC_SYNC: &str = "Sync dotfiles for the specified profile";
pub const DESC_SETUP: &str = "Setup dotfiles for the specified profile";
pub const DESC_RESET: &str = "Reset dotfiles to their default state";
pub const DESC_UPDATE: &str = "Update dotfiles from the repository";
pub const DESC_SWITCH: &str = "Switch to a different dotfiles profile";
pub const DESC_LIST_CONFIGS: &str = "List available dotfiles configurations";
pub const DESC_CREATE_PROFILE: &str = "Create a new dotfiles profile";
pub const DESC_ROLLBACK: &str = "Rollback to a previous state of dotfiles";