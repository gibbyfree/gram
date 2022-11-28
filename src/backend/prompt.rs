use crate::data::{enums::StatusContent, textrow::TextRow};

// PromptProcessor. The PromptProcessor can be thought of as a highly stripped down version of the RenderEngine.
// It carries a single TextRow to contain any user input, cx corresponding to the cursor in the prompt field, and StatusContent to represent
// what kind of prompt interaction is in-progress. The fields of the PromptProc should be primarily manipulated by the OperationsHandler.
pub struct PromptProcessor {
    pub text: TextRow,
    pub status: Option<StatusContent>,
    pub cx: i16,
}

impl PromptProcessor {
    pub fn new() -> Self {
        Self {
            text: TextRow::new("".to_string()),
            status: None,
            cx: 0,
        }
    }

    // Updates status content according to presently input text.
    fn update_status_content(&mut self) {
        let status = &self.status;
        if let Some(content) = status {
            if let StatusContent::SaveAs(_str) = content {
                let new_status = StatusContent::SaveAs(self.text.raw_text.clone());
                self.set_status(new_status);
            }
        }
    }

    // Flushes all data within the processor, resetting to default values.
    pub fn flush(&mut self) {
        self.cx = 0;
        self.text = TextRow::new("".to_string());
        self.status = None;
    }

    // Set the cursor.
    pub fn set_cursor(&mut self, cx: i16) {
        self.cx = cx;
    }

    // Set StatusContent.
    pub fn set_status(&mut self, status: StatusContent) {
        self.status = Some(status);
    }

    // Set text. If there's an ongoing prompt interaction, update status content.
    pub fn set_text(&mut self, text: String) {
        self.text.update_text(text);
        match self.status {
            None => (),
            _ => self.update_status_content(),
        }
    }
}
