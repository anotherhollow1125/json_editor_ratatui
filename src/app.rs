use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub enum CurrentScreen {
    Main,
    Editing(CurrentlyEditing),
    Exiting,
}

#[derive(Clone, Copy, Debug)]
pub enum CurrentlyEditing {
    Key,
    Value,
}

pub struct App {
    pub key_input: String,
    pub value_input: String,
    pub pairs: HashMap<String, String>,
    pub current_screen: CurrentScreen,
}

impl App {
    pub fn new() -> App {
        App {
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
        }
    }

    pub fn save_key_value(&mut self) {
        self.pairs
            .insert(self.key_input.clone(), self.value_input.clone());

        self.key_input = String::new();
        self.value_input = String::new();
    }

    pub fn toggle_editing(&mut self) {
        use CurrentScreen::*;

        self.current_screen = match self.current_screen {
            Editing(CurrentlyEditing::Key) => Editing(CurrentlyEditing::Value),
            Editing(CurrentlyEditing::Value) => Editing(CurrentlyEditing::Key),
            s => s,
        };
    }

    pub fn print_json(&self) -> serde_json::Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{}", output);
        Ok(())
    }
}
