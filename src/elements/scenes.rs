use raylib::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

pub struct SceneManager {
    pub scene_list: Vec<Rc<RefCell<Option<Box<dyn AsScene>>>>>,
    pub current_scene: Rc<RefCell<Option<Box<dyn AsScene>>>>,
}

enum Command {
    Exit,
    JumpToNamedScene,
    Stay,
}

pub struct SceneCommand {
    command: Command,
    args: Vec<&'static str>
}

impl SceneCommand {
    pub fn jump_to_scene(scene_name: &'static str) -> SceneCommand {
        SceneCommand {
            command: Command::JumpToNamedScene,
            args: vec![scene_name]
        }
    }

    pub fn continue_program() -> SceneCommand {
        SceneCommand {
            command: Command::Stay,
            args: vec![]
        }
    }

    pub fn exit_program() -> SceneCommand {
        SceneCommand {
            command: Command::Exit,
            args: vec![]
        }
    }
}

impl SceneManager {
    pub fn new() -> Self {
        Self {
            scene_list: vec![],
            current_scene: Rc::from(RefCell::from(None))
        }
    }

    pub fn push_scene(&mut self, scene: Box<dyn AsScene>){
        self.scene_list.push(Rc::from(RefCell::from(Option::from(scene))));
    }

    pub fn get_scene_by_name(&self, scene_name: &'static str) -> Rc<RefCell<Option<Box<dyn AsScene>>>> {
        let scene = self.scene_list.iter().find(|s| s.borrow().as_ref().unwrap().name() == scene_name).cloned();

        return scene.unwrap_or(Rc::from(RefCell::from(None)));
    }

    pub fn set_current_scene(&mut self, scene_name: &'static str) {
        self.current_scene = self.get_scene_by_name(scene_name);
    }

    pub fn parse_command(& mut self, scene_command: SceneCommand) {
        match scene_command.command {
            Command::JumpToNamedScene => { 
                self.set_current_scene(scene_command.args[0])
            },
            Command::Exit => {},
            Command::Stay => {}
        }
    }

    pub fn play_scene(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        let scene = Rc::clone(&self.current_scene);
        let command: SceneCommand = scene.borrow_mut().as_mut().unwrap().update(rl, thread);
        self.parse_command(command);

        if scene.borrow().as_ref().unwrap().name() != self.current_scene.as_ref().borrow().as_ref().unwrap().name() {
            self.play_scene(rl, thread)
        }
    }
}

pub trait AsScene {
    fn load(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread);
    fn update(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) -> SceneCommand;
    fn unload(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread);

    fn name(&self) -> &'static str;
}
