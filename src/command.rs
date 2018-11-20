use std::path::PathBuf;

use sheet::Frame;
use state::Document;

pub enum Command {
    NewDocument,
    OpenDocument,
    FocusDocument(PathBuf),
    CloseCurrentDocument,
    CloseAllDocuments,
    SaveCurrentDocument,
    SaveCurrentDocumentAs,
    SaveAllDocuments,
    Import,
    SelectFrame(PathBuf),
    EditFrame(PathBuf),
    ZoomIn,
    ZoomOut,
}

pub struct CommandBuffer {
    queue: Vec<Command>,
}

impl CommandBuffer {
    pub fn new() -> CommandBuffer {
        CommandBuffer { queue: vec![] }
    }

    pub fn append(&mut self, mut other: CommandBuffer) {
        self.queue.append(&mut other.queue);
    }

    pub fn flush(&mut self) -> Vec<Command> {
        std::mem::replace(&mut self.queue, vec![])
    }

    pub fn new_document(&mut self) {
        self.queue.push(Command::NewDocument);
    }

    pub fn open_document(&mut self) {
        self.queue.push(Command::OpenDocument);
    }

    pub fn focus_document(&mut self, document: &Document) {
        self.queue
            .push(Command::FocusDocument(document.get_source().to_owned()));
    }

    pub fn close_current_document(&mut self) {
        self.queue.push(Command::CloseCurrentDocument);
    }

    pub fn close_all_documents(&mut self) {
        self.queue.push(Command::CloseAllDocuments);
    }

    pub fn save(&mut self) {
        self.queue.push(Command::SaveCurrentDocument);
    }

    pub fn save_as(&mut self) {
        self.queue.push(Command::SaveCurrentDocumentAs);
    }

    pub fn save_all(&mut self) {
        self.queue.push(Command::SaveAllDocuments);
    }

    pub fn import(&mut self) {
        self.queue.push(Command::Import);
    }

    pub fn select_frame(&mut self, frame: &Frame) {
        self.queue
            .push(Command::SelectFrame(frame.get_source().to_owned()));
    }

    pub fn edit_frame(&mut self, frame: &Frame) {
        self.queue
            .push(Command::EditFrame(frame.get_source().to_owned()));
    }

    pub fn zoom_in(&mut self) {
        self.queue.push(Command::ZoomIn);
    }

    pub fn zoom_out(&mut self) {
        self.queue.push(Command::ZoomOut);
    }
}
