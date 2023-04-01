use crossterm::event::KeyEvent;

pub trait CursorC {
    fn get_x(&self) -> usize;

    fn get_y(&self) -> usize;

    fn change_x(&mut self, changer: usize);

    fn change_y(&mut self, changer: usize);

    fn get_min_height(&self) -> usize;

    fn reset(&mut self, lines: &Vec<String>);

    fn take_input(&mut self, event: KeyEvent, lines: &Vec<String>);
}
pub trait LineC {
    fn get_lines(&self) -> &Vec<String>;
    fn get_header(&self) -> &Vec<String>;
    fn get_spec(&self) -> &Vec<String>;
    fn change_lines(&mut self, lines: Vec<String>);
    fn change_title(&mut self, title: String);
    fn change_status(&mut self, status: String);
    fn take_input<C: CursorC>(&mut self, event: KeyEvent, cursor_controller: &mut C);
}
