mod editor;
mod document;
mod row;
mod terminal;
use editor::Editor;
pub use document::Document;
pub use row::Row;
pub use terminal::Terminal;
pub use editor::Position;

fn main() {
    Editor::default().run();
}
