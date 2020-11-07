use amethyst::{SimpleState, StateData, GameData, StateEvent, SimpleTrans, Trans};
use log::info;
use crate::init;
use crate::game::{Game};
use amethyst::ui::{UiCreator, UiFinder, UiEventType, UiEvent, UiText};
use amethyst::input::is_close_requested;
use amethyst::core::ecs::{Entity, WorldExt};
use crate::parse_input::parse_number_selection;
use amethyst::prelude::World;
use std::error::Error;
use inflector::Inflector;
use crate::ball::CurrentGame;


const BUTTON_START: &str = "start";
const BALL_COUNT_INPUT: &str = "ball_count_input";
const TAKE_VARIANTS_INPUT: &str = "take_variants_input";
const ERROR_LABEL: &str = "error_label";

#[derive(Default, Debug)]
pub struct MainMenu {
    ui_root: Option<Entity>,
    error_label: Option<Entity>,
    button_start: Option<Entity>,
    ball_count_input: Option<Entity>,
    take_variants_input: Option<Entity>,
}

impl SimpleState for MainMenu {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.insert::<Option<CurrentGame>>(None);
        init::initialise_camera(world);
        world.exec(|mut creator: UiCreator<'_>| {
            self.ui_root = Some(creator.create("ui/main_menu.ron", ()));
        });
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(root_entity) = self.ui_root {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to remove MainMenu");
        }

        self.ui_root = None;
        self.button_start = None;
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match event {
            StateEvent::Window(event) =>
                if is_close_requested(&event) {
                    Trans::Quit
                } else {
                    Trans::None
                },
            StateEvent::Ui(UiEvent {
                               event_type: UiEventType::Click,
                               target
                           }) => {
                if Some(target) == self.button_start {
                    match self.switch_to_game(data.world) {
                        Ok(trans) => trans,
                        Err(e) => {
                            log::warn!("Error switching: {}", e);
                            let mut ui_text =
                                data.world.write_storage::<UiText>();
                            self.error_label
                                .and_then(|l| ui_text.get_mut(l))
                                .map(|text| text.text =
                                    e.to_string().to_sentence_case());
                            Trans::None
                        }
                    }
                } else {
                    Trans::None
                }
            }
            // StateEvent::Ui(UiEvent {
            //     event_type: UiEventType::ValueChange,
            //     target
            //            }) => {
            //     if Some(target) == self.ball_count_input
            //         || Some(target) == self.take_variants_input {
            //         let mut ui_text = data.world.write_storage::<UiText>();
            //         self.error_label
            //             .and_then(|l| ui_text.get_mut(l))
            //             .map(|text| text.text = "".into());
            //     }
            //     Trans::None
            // }
            _ => Trans::None
        }
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.button_start.is_none() {
            data.world.exec(|f: UiFinder|
                self.button_start = f.find(BUTTON_START))
        };

        if self.ball_count_input.is_none() {
            data.world.exec(|f: UiFinder|
                self.ball_count_input = f.find(BALL_COUNT_INPUT))
        }

        if self.take_variants_input.is_none() {
            data.world.exec(|f: UiFinder|
                self.take_variants_input = f.find(TAKE_VARIANTS_INPUT))
        }

        if self.error_label.is_none() {
            data.world.exec(|f: UiFinder|
                self.error_label = f.find(ERROR_LABEL)
            )
        }

        Trans::None
    }
}

impl MainMenu {
    fn switch_to_game(&mut self, world: &mut World) -> Result<SimpleTrans, Box<dyn Error>> {
        let storage = world.write_storage::<UiText>();
        let input = self.ball_count_input
            .and_then(|input| storage.get(input))
            .ok_or("No balls count input field found")?;
        let range_input = self.take_variants_input
            .and_then(|input| storage.get(input))
            .ok_or("No range input field found")?;

        if input.text.is_empty() {
            return Err("Please input ball count".into())
        }
        if range_input.text.is_empty() {
            return Err("Please input range".into());
        }

        let range = parse_number_selection(&range_input.text)?;

        let ball_count = input.text.parse()?;

        if range.iter().any(|&x| x >= ball_count || x <= 0) {
            Err("Must take only between 0 and ball count".into())
        } else if ball_count > crate::game::MAX_BALLS {
            Err("Too many balls".into())
        } else if ball_count < crate::game::MIN_BALLS {
            Err("Too few balls".into())
        } else {
            info!("Switching from MainMenu to Game");

            Ok(Trans::Switch(Box::new(
                Game::new(ball_count, range)
            )))
        }
    }
}
