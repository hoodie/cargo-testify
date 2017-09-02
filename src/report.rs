/// This enum represents an outcome of attempt to run tests.
/// It's passed to a notifier in order to display a message to a user.
pub enum Outcome {
    /// Tests have passed successfully
    TestsPassed,

    /// Tests failed
    TestsFailed,

    /// Compilation error detected
    CompileError
}

pub struct Report {
    pub outcome: Outcome,
    pub detail: Option<String>
}

impl Report {
    pub fn title(&self) -> &'static str {
        match self.outcome {
            Outcome::TestsPassed => "Tests passed",
            Outcome::TestsFailed => "Tests failed",
            Outcome::CompileError => "Error"
        }
    }
}
