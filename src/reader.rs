use linefeed::{Interface, DefaultTerminal, ReadResult};

const FILE_HISTORY: &str = ".history";
const PROMPT_MESSAGE: &str = "ldb >>> ";

pub struct Reader {
    interface: Interface<DefaultTerminal>,

}

impl Reader {
    pub(crate) fn new() -> Self {
        let mut reader = Interface::new("ldb")
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

    pub(crate) fn save_history(&self, line: &String) {
        if !line.trim().is_empty() {
            self.get_interface()
                .add_history_unique(line.clone());
        }
    }

    fn get_interface(&self) -> &Interface<DefaultTerminal> {
        &self.interface
    }

    pub(crate) fn read_line(&self) -> ReadResult {
        self.get_interface()
            .read_line()
            .expect("Cant read line.")
    }
}