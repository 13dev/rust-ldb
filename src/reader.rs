use linefeed::{DefaultTerminal, Interface, ReadResult};

const FILE_HISTORY: &str = "history.txt";
const PROMPT_MESSAGE: &str = "ldb >>> ";

pub struct Reader {
    interface: Interface<DefaultTerminal>,
}

impl Reader {
    pub fn new(name: &'static str) -> Self {
        let reader = Interface::new(name).expect("Can't create a new application.");

        // Set prompt
        reader
            .set_prompt(PROMPT_MESSAGE)
            .expect("Can't set a prompt.");

        // Load history
        if reader.load_history(FILE_HISTORY).is_err() {
            println!("No History.");
        }

        Self { interface: reader }
    }

    pub fn save_history(&self, line: &str) {
        if !line.trim().is_empty() {
            self.interface.add_history_unique(String::from(line));

            // TODO: send to a command like [history save]
            self.interface
                .save_history(FILE_HISTORY)
                .expect("Cant save the history.");
        }
    }

    pub fn read_line(&self) -> ReadResult {
        self.interface.read_line().expect("Cant read line.")
    }
}
