use std::rc::Rc;
use std::cell::RefCell;

trait Command {
    fn execute(&mut self);
}

trait Positionable {
    fn get_position(&self) -> (u8, u8);
    fn set_position(&mut self, pos: (u8, u8));
}

struct MoveLeftCommand {
    positionable: Rc<RefCell<dyn Positionable>>,
}

impl Command for MoveLeftCommand {
    fn execute(&mut self) {
        let (x, y) = self.positionable.borrow().get_position();
        self.positionable.borrow_mut().set_position((x - 1, y));
    }
}

struct Cursor {
    pub position: (u8, u8),
}

impl Positionable for Cursor {
    fn get_position(&self) -> (u8, u8) {
        self.position
    }
    fn set_position(&mut self, pos: (u8, u8)) {
        self.position = pos;
    }
}

struct Commands(Vec<Box<dyn Command>>);

impl From<Rc<RefCell<Cursor>>> for Commands {
    fn from(cursor: Rc<RefCell<Cursor>>) -> Commands {
        Commands(vec![
            // This will register different commands this is just for example
            Box::new(MoveLeftCommand { positionable: cursor.clone()}),
            Box::new(MoveLeftCommand { positionable: cursor.clone()}),
        ])
    }
}

struct Registry {
    active_commands: Commands,
}

impl Registry {
    fn register<T: Into<Commands>>(&mut self, registrar: T) {
        self.active_commands = registrar.into();
    }
    
    fn process(&mut self) {
        for command in &mut self.active_commands.0 {
            command.execute();
        }
    }
}

fn main() {
    let mut registry = Registry { active_commands: Commands(Vec::new()) };
    
    let cursor = Rc::new(RefCell::new(Cursor { position: (5, 5) }));
    registry.register(cursor.clone());
    
    registry.process();
    
    println!("{:?}", cursor.borrow().position); // Should be 3, 5
}

