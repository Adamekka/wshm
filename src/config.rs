use serde::{Deserialize, Serialize};

trait DefaultButtonsConfig {
    fn default_config() -> Self;
}

impl DefaultButtonsConfig for Vec<Button> {
    fn default_config() -> Self {
        vec![
            Button {
                label: "Lock".to_string(),
                actions: vec!["loginctl".to_string(), "lock-session".to_string()],
            },
            Button {
                label: "Sleep".to_string(),
                actions: vec!["systemctl".to_string(), "suspend".to_string()],
            },
            Button {
                label: "Restart".to_string(),
                actions: vec!["systemctl".to_string(), "reboot".to_string()],
            },
            Button {
                label: "Shutdown".to_string(),
                actions: vec!["shutdown".to_string(), "now".to_string()],
            },
            Button {
                label: "Logout".to_string(),
                actions: vec!["loginctl".to_string(), "terminate-session".to_string()],
            },
        ]
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Margins {
    pub top: i32,
    pub bottom: i32,
    pub left: i32,
    pub right: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Button {
    /// Button label
    pub label: String,

    /// Program to run when the button is clicked
    pub actions: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    /// Window title
    pub title: String,

    /// Margins between the window and the screen edges
    pub margins: Margins,

    /// Buttons to display
    pub buttons: Vec<Button>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            title: "Shutdown Menu".to_string(),
            margins: Margins::default(),
            buttons: Vec::<Button>::default_config(),
        }
    }
}

impl Config {
    pub fn load(path: &std::path::Path) -> Option<Self> {
        let file_path_result = std::fs::File::open(path);

        let parse_result = match file_path_result {
            Ok(file) => serde_json::from_reader(file),
            Err(error) => {
                eprintln!("Failed to open config file: {}", error);
                return None;
            }
        };

        match parse_result {
            Ok(config) => {
                println!("Loaded config file: {:?}", path);
                config
            }
            Err(error) => {
                eprintln!("Failed to parse config file: {}", error);
                None
            }
        }
    }
}
