use std::{
    env,
    fs::{self, File},
    path::PathBuf,
};
use str_macro::*;

pub struct CanisterIds {
    pub cycles_management_controller: String,
    pub group_controller: String,
    pub group_management_controller: String,
    pub management_controller: String,
    pub report_controller: String,
    pub storage_controller: String,
    pub storage_management_controller: String,
    pub user_controller: String,
}

pub struct Settings {
    pub environment: String,
    pub max_registered_users: usize,
    pub max_registered_users_enabled: bool,
    pub max_group_canisters: usize,
    pub max_group_canisters_enabled: bool,
    pub max_storage_canisters: usize,
    pub max_storage_canisters_enabled: bool,
}

impl CanisterIds {
    pub fn to_content(&self) -> String {
        let main_text = str!(
            "
/*
This file is automatically generated based on the environment set

File can be generated with the following commands:
    LOCAL: \"ENV=local cargo test --test generate\"
    DEVELOPMENT: \"ENV=development cargo test --test generate\"
    STAGING: \"ENV=staging cargo test --test generate\"
    PRODUCTION: \"ENV=production cargo test --test generate\"

    Note: If the local canister ids are not known yet, 
    the \"dfx canister create --all --no-wallet\" command should be ran
    to generate the local canister ids (.dfx/local/canister_ids.json)
*/"
        );

        let environment_dependent_canisters_text = str!(
            "

// These canisters are environment dependent
"
        );

        let local_canisters_text = str!(
            "

// These canisters are only used for the local deployment
"
        );

        let environment_independent_canisters_text = str!(
            "

// These canisters are environment independent
"
        );

        let generated_setting_text = str!(
            "

// Settings to be specified per environment
"
        );

        let cycles_management_controller = format!(
            "pub const CYCLES_MANAGEMENT_CANISTER_ID: &str = \"{}\";\r",
            self.cycles_management_controller
        );
        let development_management_controller = format!(
            "pub const DEVELOPMENT_MANAGEMENT_CANISTER_ID: &str = \"{}\";\r",
            get_management_canister_id_by_environment(str!("development"))
        );

        let staging_management_controller = format!(
            "pub const STAGING_MANAGEMENT_CANISTER_ID: &str = \"{}\";\r",
            get_management_canister_id_by_environment(str!("staging"))
        );

        let productiom_management_controller = format!(
            "pub const PRODUCTION_MANAGEMENT_CANISTER_ID: &str = \"{}\";\r",
            get_management_canister_id_by_environment(str!("production"))
        );

        let group_controller = format!(
            "pub const GROUP_CANISTER_ID: &str = \"{}\";\r",
            self.group_controller
        );
        let group_management_controller = format!(
            "pub const GROUP_MANAGEMENT_CANISTER_ID: &str = \"{}\";\r",
            self.group_management_controller
        );
        let management_controller = format!(
            "pub const MANAGEMENT_CANISTER_ID: &str = \"{}\";\r",
            self.management_controller
        );
        let report_controller = format!(
            "pub const REPORT_CANISTER_ID: &str = \"{}\";\r",
            self.report_controller
        );
        let storage_controller = format!(
            "pub const STORAGE_CANISTER_ID: &str = \"{}\";\r",
            self.storage_controller
        );
        let storage_management_controller = format!(
            "pub const STORAGE_MANAGEMENT_CANISTER_ID: &str = \"{}\";\r",
            self.storage_management_controller
        );
        let user_controller = format!(
            "pub const USER_CANISTER_ID: &str = \"{}\";\r",
            self.user_controller
        );

        let enviroment = format!(
            "pub const ENVIRONMENT: &str = \"{}\";\r",
            generate_enviroment_settings().environment
        );

        let max_group_canister = format!(
            "pub const MAX_GROUP_CANISTERS: usize = {};\r",
            generate_enviroment_settings().max_group_canisters
        );

        let max_group_canister_enabled = format!(
            "pub const MAX_GROUP_CANISTERS_ENABLED: bool = {};\r",
            generate_enviroment_settings().max_group_canisters_enabled
        );

        let max_storage_canister = format!(
            "pub const MAX_STORAGE_CANISTERS: usize = {};\r",
            generate_enviroment_settings().max_storage_canisters
        );

        let max_storage_canister_enabled = format!(
            "pub const MAX_STORAGE_CANISTERS_ENABLED: bool = {};\r",
            generate_enviroment_settings().max_storage_canisters_enabled
        );

        let max_registered_users = format!(
            "pub const MAX_REGISTERED_USERS: usize = {};\r",
            generate_enviroment_settings().max_registered_users
        );

        let max_registered_users_enabled = format!(
            "pub const MAX_REGISTERED_USERS_ENABLED: bool = {};\r",
            generate_enviroment_settings().max_registered_users_enabled
        );

        format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            main_text,
            environment_independent_canisters_text,
            cycles_management_controller,
            development_management_controller,
            staging_management_controller,
            productiom_management_controller,
            environment_dependent_canisters_text,
            group_management_controller,
            management_controller,
            report_controller,
            storage_management_controller,
            user_controller,
            local_canisters_text,
            group_controller,
            storage_controller,
            generated_setting_text,
            enviroment,
            max_group_canister,
            max_group_canister_enabled,
            max_storage_canister,
            max_storage_canister_enabled,
            max_registered_users,
            max_registered_users_enabled
        )
    }
}

pub fn generate_canister_ids() {
    let mut env_path = str!(".dfx/local/canister_ids.json");
    if get_env() != str!("local") {
        env_path = str!("canister_ids.json");
    }
    let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let file_path = dir.parent().unwrap().parent().unwrap().join(env_path);
    let file = File::open(file_path).expect("file should open read only");
    let json: serde_json::Value =
        serde_json::from_reader(file).expect("file should be proper JSON");

    let canister_ids = CanisterIds {
        cycles_management_controller: get_json_value(&json, str!("cycles_management_controller")),
        group_controller: get_json_value(&json, str!("group_controller")),
        group_management_controller: get_json_value(&json, str!("group_management_controller")),
        management_controller: get_json_value(&json, str!("management_controller")),
        report_controller: get_json_value(&json, str!("report_controller")),
        storage_controller: get_json_value(&json, str!("storage_controller")),
        storage_management_controller: get_json_value(&json, str!("storage_management_controller")),
        user_controller: get_json_value(&json, str!("user_controller")),
    };

    let _ = fs::write("environment_settings.rs", canister_ids.to_content());
}

pub fn generate_enviroment_settings() -> Settings {
    let env = get_env();
    if env == str!("development") {
        return Settings {
            max_registered_users_enabled: true,
            max_registered_users: 500,
            max_group_canisters_enabled: true,
            max_group_canisters: 10,
            max_storage_canisters_enabled: true,
            max_storage_canisters: 1,
            environment: env,
        };
    }

    if env == str!("staging") {
        return Settings {
            max_registered_users_enabled: true,
            max_registered_users: 0,
            max_group_canisters_enabled: true,
            max_group_canisters: 3,
            max_storage_canisters_enabled: true,
            max_storage_canisters: 1,
            environment: env,
        };
    }

    if env == str!("production") {
        return Settings {
            max_registered_users_enabled: true,
            max_registered_users: 0,
            max_group_canisters_enabled: false,
            max_group_canisters: 0,
            max_storage_canisters_enabled: false,
            max_storage_canisters: 0,
            environment: env,
        };
    }

    return Settings {
        max_registered_users_enabled: false,
        max_registered_users: 0,
        max_group_canisters_enabled: false,
        max_group_canisters: 0,
        max_storage_canisters_enabled: false,
        max_storage_canisters: 0,
        environment: str!("local"),
    };
}

// pub fn get_canister_ids_array() -> Vec<String> {
//     let canisters = vec![
//         // CYCLES_MANAGEMENT_CANISTER.to_string(),
//         GROUP_CANISTER_ID.to_string(),
//         GROUP_MANAGEMENT_CANISTER_ID.to_string(),
//         MANAGEMENT_CANISTER_ID.to_string(),
//         REPORT_CANISTER_ID.to_string(),
//         STORAGE_CANISTER_ID.to_string(),
//         STORAGE_MANAGEMENT_CANISTER_ID.to_string(),
//         USER_CANISTER_ID.to_string(),
//     ];

//     // filter out empty canisters
//     canisters
//         .iter()
//         .filter(|c| c != &&str!(""))
//         .cloned()
//         .collect()
// }

pub fn get_env() -> String {
    let env = env::var("ENV");
    match env {
        Ok(_env) => {
            return _env;
        }
        Err(_) => str!("local"),
    }
}

fn get_json_value(json: &serde_json::Value, name: String) -> String {
    let mut return_value: String = String::from("");

    let json_value = json.get(name);

    match json_value {
        Some(value) => match value.get(get_env()) {
            Some(v) => match v {
                serde_json::Value::String(_v) => return_value = _v.clone(),
                serde_json::Value::Null => {}
                serde_json::Value::Bool(_) => {}
                serde_json::Value::Number(_) => {}
                serde_json::Value::Array(_) => {}
                serde_json::Value::Object(_) => {}
            },
            None => return_value = str!(""),
        },
        None => return_value = str!(""),
    }

    return_value
}

pub fn get_management_canister_id_by_environment(environment: String) -> String {
    let env_path = str!("canister_ids.json");
    let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let file_path = dir.parent().unwrap().parent().unwrap().join(env_path);
    let file = File::open(file_path).expect("file should open read only");
    let json: serde_json::Value =
        serde_json::from_reader(file).expect("file should be proper JSON");

    let mut return_value: String = String::from("");
    let json_value = json.get("management_controller");

    match json_value {
        Some(value) => match value.get(environment) {
            Some(v) => match v {
                serde_json::Value::String(_v) => return_value = _v.clone(),
                serde_json::Value::Null => {}
                serde_json::Value::Bool(_) => {}
                serde_json::Value::Number(_) => {}
                serde_json::Value::Array(_) => {}
                serde_json::Value::Object(_) => {}
            },
            None => return_value = str!(""),
        },
        None => return_value = str!(""),
    }

    return_value
}