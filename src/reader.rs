use linefeed::{Interface, DefaultTerminal, ReadResult};

const FILE_HISTORY: &str = "history.txt";
const PROMPT_MESSAGE: &str = "ldb >>> ";

pub struct Reader {
    interface: Interface<DefaultTerminal>,
}

impl Reader {
    pub fn new() -> Self {
        let reader = Interface::new("ldb")
            .expect("Can't create a new application.");

        // Set prompt
        reader.set_prompt(PROMPT_MESSAGE)
            .expect("Can't set a prompt.");

        // Load history
        if reader.load_history(FILE_HISTORY).is_err() {
            println!("No History.");
        }

        Self {
            interface: reader,
        }
    }

    pub fn save_history(&self, line: &String) {
        if !line.trim().is_empty() {
            self.get_interface()
                .add_history_unique(line.clone());

            // TODO: send to a command like [history save]
            // self.get_interface()
            //     .save_history(FILE_HISTORY);
        }
    }

    pub fn get_interface(&self) -> &Interface<DefaultTerminal> {
        &self.interface
    }

    pub fn read_line(&self) -> ReadResult {
        self.get_interface()
            .read_line()
            .expect("Cant read line.")
    }
}