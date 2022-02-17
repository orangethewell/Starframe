use raylib::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

/// ## Scene Manager 
/// is the core for scene's creation. It makes simple to load up and load down
/// all the scenes from the program. 
/// ### How to use it
/// To use Scene Manager, create a new scene and push it into your Scene Manager:
/// 
/// ```rust
/// struct MyCoolScene {
/// // Some random scope variables...
/// }
/// 
/// impl AsScene for MyCoolScene {
///     // Scene implementation...
/// }
/// 
/// fn main (){
///     let mut scene_manager = SceneManager::new();
///     let mut my_scene = MyCoolScene::new();
/// 
///     // Should be boxed because SceneManager deals with traits.
///     scene_manager.push_scene(Box::new(my_scene)); 
/// }
/// ```
/// 
/// After implementing it and pushing to `scene_manager`, you will need to set the
/// scene manager's current scene with your scene name. Scene name should be 
/// a `&'static str` inside scene's struct. Pretending that scene name
/// is "Cool", all we need to do is this:
/// ```rust
///     scene_manager.set_current_scene("Cool");
/// ```
/// 
/// And then, use `scene_manager.play_scene(...)`. Actually, SceneManager is built with
/// Raylib-rs backend, so you need to provide both `RaylibHandle` and `RaylibThread` to
/// use it.
pub struct SceneManager {
    pub scene_list: Vec<Rc<RefCell<Option<Box<dyn AsScene>>>>>,
    pub current_scene: Rc<RefCell<Option<Box<dyn AsScene>>>>,
}

/// A little set of commands that can be used with SceneManager.
enum Command {
    Exit,
    JumpToNamedScene,
    Stay,
}

/// ## Scene Command
/// Scene Command is a struct used as a return value for the `update` scene's function.
/// It's used to send little commands that can be parsed by `SceneManager`, these commands
/// makes it easy to control the backstage's flow, like exiting the program, jumping to 
/// another scene or, as it is, continue to execute the program on certain scene.
pub struct SceneCommand {
    command: Command,
    args: Vec<&'static str>
}

impl SceneCommand {
    /// Creates a simple flow command to jump to another scene by giving
    /// the name of scene.
    pub fn jump_to_scene(scene_name: &'static str) -> SceneCommand {
        SceneCommand {
            command: Command::JumpToNamedScene,
            args: vec![scene_name]
        }
    }

    /// Generates a scene command to continue the flow with the current scene.
    pub fn continue_program() -> SceneCommand {
        SceneCommand {
            command: Command::Stay,
            args: vec![]
        }
    }

    /// Generates a flow command to finish the program.
    pub fn exit_program() -> SceneCommand {
        SceneCommand {
            command: Command::Exit,
            args: vec![]
        }
    }
}

impl SceneManager {
    /// Creates a new and empty Scene Manager context.
    pub fn new() -> Self {
        Self {
            scene_list: vec![],
            current_scene: Rc::from(RefCell::from(None))
        }
    }

    /// Inserts a scene on scene's list.
    pub fn push_scene(&mut self, scene: Box<dyn AsScene>){
        self.scene_list.push(Rc::from(RefCell::from(Option::from(scene))));
    }

    /// Makes a search on scene list with the given name. 
    /// If finds it, returns a `RefCell` filled with a `Option` with scene
    /// inside it. If don't, also returns a `RefCell` with `Option::None`
    /// inside it.
    pub fn get_scene_by_name(&self, scene_name: &'static str) -> Rc<RefCell<Option<Box<dyn AsScene>>>> {
        let scene = self.scene_list.iter().find(|s| s.borrow().as_ref().unwrap().name() == scene_name).cloned();

        return scene.unwrap_or(Rc::from(RefCell::from(None)));
    }

    /// Set current scene as a scene with given name inside scene's list.
    pub fn set_current_scene(&mut self, scene_name: &'static str) {
        self.current_scene = self.get_scene_by_name(scene_name);
    }

    /// Parses a `SceneCommand` sent by the current scene in execution.    
    pub fn parse_command(& mut self, scene_command: SceneCommand) {
        match scene_command.command {
            Command::JumpToNamedScene => { 
                self.set_current_scene(scene_command.args[0])
            },
            Command::Exit => {std::process::exit(0);},
            Command::Stay => {}
        }
    }

    /// Plays the current scene set on Scene Manager.
    /// 
    /// **OBS:** As said on Scene Manager's doc, `SceneManager` is
    /// built with **Raylib-rs** backend, so you need to borrow `RaylibHandle`
    /// and `RaylibThread` for this function.
    pub fn play_scene(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        let scene = Rc::clone(&self.current_scene);
        let command: SceneCommand = scene.borrow_mut().as_mut().unwrap().update(rl, thread);
        self.parse_command(command);

        if scene.borrow().as_ref().unwrap().name() != self.current_scene.as_ref().borrow().as_ref().unwrap().name() {
            self.play_scene(rl, thread)
        }
    }
}

/// ## "As Scene" Trait
/// This trait turns any struct in a scene that can be inserted to
/// Scene Manager. These structs can be handled in a way that they
/// can display something on screen when they are called. There are
/// 4 functions that need to have a implementation.
/// 
/// ### How to use it
/// The best way to start with it is creating a struct with all your
/// multi-scope variables:
/// 
/// ```rust
/// // my_scene.rs
/// pub struct Menu {
///     name: &'static str, // Used to name scene on SceneManager;
/// 
///     button_start: Button,
///     button_options: Button,
///     button_exit: Button
/// 
///     // other random variables to use in scene...
/// }
/// 
/// // You should create the `fn new()` for your scene to initialize it;
/// impl Menu { 
///     pub fn new() -> Menu {
///         Menu {
///             name: "Menu" // The name chosed for this scene.
/// 
///             // Button initialization there...
///         }
///     }
/// }
/// 
/// // There you implement AsScene trait;
/// 
/// impl AsScene for Menu {
///     // There I will only show update function, anyways, this scene managing 
///     // implementation doesn't call other functions, you will need to call them
///     // yourself.
/// 
///     fn update(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) -> SceneCommand {
///         // do all the stuff...
///         match button.label { // Simple example for SceneCommand states:
///              // SceneCommand is needed as a return value for 
///              // `update` function, it  shows what Scene 
///              // Manager should do next.
///             "Start" => SceneCommand::jump_to_scene("Game"),
///             "Options" => SceneCommand::continue_program(),
///             "Exit" => SceneCommand::exit_program(),
///         }
///     }
/// }
/// ```
/// 
/// After implementing `AsScene` trait, then you can you use it with `SceneManager` and call
/// the functions to show something on the screen. Remembering there that SceneManager uses
/// Raylib-rs as their backend, so you need to pass both `RaylibHandle` and `RaylibThread`
/// to functions.
pub trait AsScene {
    fn load(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread);
    fn update(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) -> SceneCommand;
    fn unload(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread);

    fn name(&self) -> &'static str;
}