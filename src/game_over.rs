use amethyst::{
    ecs::Entity,
    input::{is_close_requested, is_key_down},
    prelude::*,
    shrev::EventChannel,
    ui::{UiCreator, UiEvent, UiEventType, UiFinder},
    winit::VirtualKeyCode,
    TransEvent,
};

use crate::main_menu::MainMenu;

#[derive(Default)]
pub struct GameOver {
    exit_to_main_menu_button: Option<Entity>,
    exit_button: Option<Entity>,
    root: Option<Entity>,
}

const EXIT_TO_MAIN_MENU_BUTTON_ID: &str = "exit_to_main_menu";
const EXIT_BUTTON_ID: &str = "exit";

// load the pause_menu.ron prefab then instantiate it
// if the "resume" button is clicked, goto MainGameState
// if the "exit_to_main_menu" button is clicked, remove the pause and main game states and go to MenuState.
// if the "exit" button is clicked, quit the program.
impl<'a> SimpleState for GameOver {
    fn on_start(&mut self, data: StateData<GameData>) {
        self.root =
            Some(data.world.exec(|mut creator: UiCreator<'_>|
                creator.create("ui/game_over.ron", ())));
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(root) = self.root {
            if data.world.delete_entity(root).is_ok() {
                self.root = None;
            }
        }
        self.exit_to_main_menu_button = None;
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    log::info!("[Trans::Quit] Quitting Application!");
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(UiEvent {
                               event_type: UiEventType::Click,
                               target,
                           }) => {
                if Some(target) == self.exit_to_main_menu_button {
                    let mut state_transition_event_channel = data
                        .world
                        .write_resource::<EventChannel<TransEvent<GameData, StateEvent>>>();

                    // this allows us to first 'Pop' this state, and then exchange whatever was
                    // below that with a new MainMenu state.
                    state_transition_event_channel.single_write(Box::new(|| Trans::Pop));
                    state_transition_event_channel
                        .single_write(Box::new(||
                            Trans::Switch(Box::new(MainMenu::default()))));

                    log::info!("[Trans::Pop] Closing Game Over overlay!");
                    log::info!("[Trans::Switch] Switching to MainMenu!");

                    Trans::None
                } else if Some(target) == self.exit_button {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        if self.exit_to_main_menu_button.is_none()
            || self.exit_button.is_none()
        {
            data.world.exec(|ui_finder: UiFinder<'_>| {
                self.exit_to_main_menu_button = ui_finder.find(EXIT_TO_MAIN_MENU_BUTTON_ID);
                self.exit_button = ui_finder.find(EXIT_BUTTON_ID);
            });
        }
        Trans::None
    }
}
