use amethyst::{SimpleState, StateData, GameData};
use crate::ball::{Ball, remove_cached_sprite, create_ball, CurrentGame};
use amethyst::prelude::*;
use amethyst::window::ScreenDimensions;
use amethyst::renderer::debug_drawing::{DebugLines, DebugLinesParams, DebugLinesComponent};
use crate::algorithm::calculate_strategy;
use amethyst::input::{is_close_requested, is_key_down, VirtualKeyCode, StringBindings, InputEvent};
use crate::pause::PauseMenuState;
use amethyst::ecs::Entity;
use amethyst::ui::{UiCreator, UiFinder, UiText, UiEvent, UiEventType, UiTransform};
use amethyst::core::math::{Point2, Point3, Vector3};
use amethyst::renderer::palette::Srgba;
use crate::Togglable;
use amethyst::core::{Transform, HiddenPropagate};
use amethyst::core::alga::general::SubsetOf;
use std::ops::Add;
use std::thread::spawn;


pub const BALL_RADIUS: f32 = 50.0;
pub const BALL_PADDING: f32 = 5.0;
pub const MAX_BALLS: u32 = 666;
pub const MIN_BALLS: u32 = 2;

const BALL_COUNT_LABEL: &str = "ball_count_label";
const TAKE_BUTTON: &str = "take_button";
const TURN_LABEL: &str = "turn_label";
const HISTORY_LABEL: &str = "history_label";
const PROMPT_ROOT: &str = "prompt_root";
const ERROR_LABEL: &str = "error_label";

const PLAYER_TURN_TEXT: &str = "Your turn";
const COMPUTER_TURN_TEXT: &str = "Wait...";
const BALL_COUNT_LABEL_SUFFIX: &str = " balls left";

const MAX_HISTORY_LINES: u32 = 14;

#[derive(Default, Debug)]
pub struct GameRules {
    pub ball_count: u32,
    pub allowed_take_variants: Vec<u32>,
}

#[derive(Default, Debug)]
pub struct Game {
    pub rules: GameRules,
    pub strategy: Vec<u32>,
    position: u32,
    player_turn: bool,
    balls: Vec<Entity>,
    ui_root: Option<Entity>,
    ball_count_label: Option<Entity>,
    turn_label: Option<Entity>,
    history_label: Option<Entity>,
    prompt_root: Option<Entity>,
    take_button: Option<Entity>,
    error_label: Option<Entity>,
}

impl Game {
    pub fn new(ball_count: u32, allowed_take_variants: Vec<u32>) -> Self {
        // Я знаю что вычисления не делают на UI Thread'е, но whatever
        let strategy =
            calculate_strategy(ball_count, &allowed_take_variants);
        Self {
            rules: GameRules {
                ball_count,
                allowed_take_variants,
            },
            strategy,
            balls: Vec::with_capacity(ball_count as usize),
            ..Self::default()
        }
    }

    fn begin_game(&mut self, world: &mut World) {
        // Если первый ход проигрышный, то ходит игрок
        self.player_turn = *self.strategy.last().unwrap() == 0;

        let state = CurrentGame {
            can_select_balls: self.player_turn,
            balls_selected: 0,
            balls_selected_max: *self.rules.allowed_take_variants.iter().max().unwrap(),
        };

        world.insert(Some(state));

        // dbg!(&self.rules);
        // dbg!(&self.strategy);
        // dbg!(self.player_turn);

        self.update_prompt(world);
    }

    fn next_turn(&mut self, world: &mut World) {
        log::info!("Performing a step");
        {
            // self.balls.retain(|&x| x != i);
            let selected_balls: Vec<usize> = {
                let mut ball_storage = world.read_storage::<Ball>();
                self.balls.iter()
                    .enumerate()
                    .filter_map(|(i, &b)|
                        ball_storage.get(b).map(|b| (i, b)))
                    .filter(|(_, b)| b.selected)
                    .map(|(i, _)| i)
                    .collect()
            };
            if self.rules.allowed_take_variants.contains(&(selected_balls.len() as u32)) {
                dbg!(&selected_balls);
                dbg!(&self.position);
                self.position -= selected_balls.len() as u32;
                self.player_turn.toggle();
                for i in selected_balls {
                    log::info!("Removing ball {}", i);
                    let ball = self.balls.remove(i);
                    world.delete_entity(ball);
                }
                self.update_turn_ui(world);
                self.update_prompt(world);
            } else {
                let mut txt_store = world.write_storage::<UiText>();
                self.error_label
                    .and_then(|l| txt_store.get_mut(l))
                    .map(|t| t.text = "Invalid selection".into());
            }
        }
    }

    fn perform_computed_turn(&mut self, world: &mut World) {
        let balls_to_take = self.strategy[(self.position as usize) - 1];

        self.position -= balls_to_take;
        self.player_turn.toggle();
        for i in 0..balls_to_take {
            log::info!("Removing ball {}", i);
            let ball = self.balls.pop();
            ball.map(|b| world.delete_entity(b));
        }
        self.update_turn_ui(world);
        self.update_prompt(world);
    }

    fn update_turn_ui(&mut self, world: &mut World) {
        self.set_turn_label(world);
        self.set_ball_count_label(world);
    }

    fn update_prompt(&mut self, world: &mut World) {
        if self.player_turn {
            if self.prompt_root.is_none() {
                self.prompt_root = Some(
                    world.exec(|mut c: UiCreator|
                        c.create("ui/prompt.ron", ())))
            }
            let mut hide_store =
                world.write_storage::<HiddenPropagate>();
            if hide_store.contains(self.prompt_root.unwrap()) {
                hide_store.remove(self.prompt_root.unwrap());
            }
        } else if let Some(pr) = self.prompt_root {
            // self.prompt_root
            //     .map(|r| world.delete_entity(r));
            // self.prompt_root = None;
            let mut hide_store =
                world.write_storage::<HiddenPropagate>();
            if !hide_store.contains(pr) {
                hide_store.insert(pr, HiddenPropagate::new());
            }
        }
    }

    fn set_turn_label(&mut self, world: &mut World) {
        let mut ui_text_storage = world.write_storage::<UiText>();
        self.turn_label
            .and_then(|l| ui_text_storage.get_mut(l))
            .map(|text| text.text =
                if self.player_turn {
                    PLAYER_TURN_TEXT.into()
                } else {
                    COMPUTER_TURN_TEXT.into()
                });
    }

    fn set_ball_count_label(&mut self, world: &mut World) {
        let mut ui_text_storage = world.write_storage::<UiText>();
        self.ball_count_label
            .and_then(|l| ui_text_storage.get_mut(l))
            .map(|text| text.text = self.position
                .to_string().add(BALL_COUNT_LABEL_SUFFIX));
    }
}

fn get_row(num: f32) -> f32 {
    (-1.0 + (1.0 + 8.0 * num).sqrt()) / 2.0
}

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.ui_root = Some(data.world.exec(|mut creator: UiCreator<'_>|
            creator.create("ui/game.ron", ())
        ));

        data.world.register::<Ball>();
        data.world.insert(DebugLines::new());
        data.world.insert(DebugLinesParams {
            line_width: 2.0
        });

        let ball_count = self.rules.ball_count;
        self.position = ball_count;

        let screen = (*data.world.read_resource::<ScreenDimensions>()).clone();
        let game_area_center = screen.width() as f32 / 3.0;


        // let mut lines = DebugLinesComponent::with_capacity(100);
        // lines.add_rectangle_2d(
        //     Point2::new(0.0, 0.0),
        //     Point2::new(screen.width() / 3.0 * 2.0, screen.height()),
        //     0.0,
        //     Srgba::new(1.0, 0.0, 0.0, 1.0),
        // );
        // lines.add_line(
        //     Point3::new(game_area_center, screen.height(), 0.0),
        //     Point3::new(game_area_center, 0.0, 0.0),
        //     Srgba::new(0.0, 1.0, 0.0, 1.0));
        // data.world.register::<DebugLinesComponent>();
        // data.world
        //     .create_entity()
        //     .with(lines)
        //     .build();

        assert!(ball_count <= MAX_BALLS);
        let row_count = get_row(ball_count as f32).ceil() as u32;

        // Arrangement, not an array
        let mut arr = vec![0u32; row_count as usize];

        for _ in 0..ball_count {
            'inner: for i in (0..row_count).rev() {
                if arr[i as usize] < i + 1 {
                    arr[i as usize] += 1;
                    break 'inner;
                }
            }
        }

        debug_assert_eq!(arr.iter().sum::<u32>(), ball_count);

        // 2 thirds of the screen divided by count of balls in the largest row
        let ball_space = (screen.width() as f32 / 3.0 * 2.0)
            .min(screen.height() - BALL_PADDING * 8.0) / (row_count as f32);

        let ball_radius = ((ball_space - 2.0 * BALL_PADDING) / 2.0).min(BALL_RADIUS);

        let ball_padding = (ball_radius / 10.0).min(BALL_PADDING);

        for i in 0..row_count {
            let current_ball_count = arr[arr.len() - 1 - (i as usize)];
            if current_ball_count == 0 {
                continue;
            }
            let center = current_ball_count as f32 / 2.0;

            let y = (i as f32) * ((ball_radius + BALL_PADDING) * 2.0)
                + BALL_RADIUS + ball_padding * 4.0;

            for j in 1..=current_ball_count {
                let pos = center as f32 - j as f32 - 0.5;
                let x = game_area_center +
                    ((pos + 1.0) * 2.0 * (ball_radius + ball_padding));

                let ball = create_ball(data.world, ball_radius, x, y);
                self.balls.push(ball);
            }
        }

        self.begin_game(data.world);
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        remove_cached_sprite(data.world);
        data.world.insert::<Option<CurrentGame>>(None);
        let _ = data.world.delete_entities(self.balls.as_slice())
            .map_err(|e| log::error!("Error when deleting balls: {:?}", e));
        self.ui_root.map(|e|
            data.world.delete_entity(e));
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        // log::info!("Got an event: {:?}", &event);
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    log::info!("[Trans::Quit] Quitting Application!");
                    Trans::Quit
                } else if is_key_down(&event, VirtualKeyCode::Escape) {
                    log::info!("[Trans::Push] Pausing Game!");
                    Trans::Push(Box::new(PauseMenuState::default()))
                } else {
                    Trans::None
                }
            }
            // StateEvent::Ui(event) => Trans::None, // TODO
            // StateEvent::Input(input_event) => {
            //    Trans::None
            // }
            StateEvent::Ui(UiEvent {
                               event_type: ev,
                               target
                           }) => {
                match ev {
                    UiEventType::Dragging { offset_from_mouse, .. } => {
                        if Some(*target) == self.take_button {
                            let mut transforms =
                                data.world.write_storage::<Transform>();
                            transforms.get_mut(self.prompt_root.unwrap())
                                .map(|t|
                                    t.prepend_translation(
                                        Vector3::new(offset_from_mouse.x,
                                                     offset_from_mouse.y, 0.0)));
                        }
                    }
                    UiEventType::ClickStart => {
                        let mut txt_store = data.world.write_storage::<UiText>();
                        self.error_label
                            .and_then(|e| txt_store.get_mut(e))
                            .map(|t| t.text = String::new());
                    }
                    UiEventType::Click => {
                        if Some(*target) == self.take_button {
                            self.next_turn(data.world);
                        }
                    }
                    _ => {}
                }
                Trans::None
            }
            _ => Trans::None
        }
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.ball_count_label.is_none()
            || self.history_label.is_none()
            || self.turn_label.is_none()
            || self.error_label.is_none() {
            data.world.exec(|f: UiFinder| {
                self.ball_count_label = f.find(BALL_COUNT_LABEL);
                self.history_label = f.find(HISTORY_LABEL);
                self.turn_label = f.find(TURN_LABEL);
                self.error_label = f.find(ERROR_LABEL);
            });
            self.update_turn_ui(data.world);
        }
        if self.player_turn
            && (self.prompt_root.is_none() || self.take_button.is_none()) {
            // log::info!("Creating prompt");
            data.world.exec(|f: UiFinder| {
                self.prompt_root = f.find(PROMPT_ROOT);
                self.take_button = f.find(TAKE_BUTTON);
            });
        } else if !self.player_turn && self.prompt_root.is_some() {
            log::info!("Deleting prompt");
            data.world.delete_entity(self.prompt_root.unwrap());
            self.prompt_root = None;
            self.take_button = None;
        }

        if !self.player_turn {
            self.perform_computed_turn(data.world);
        }

        Trans::None
    }
}

// pub fn debug_ball(world: &mut World) {
//     let size = world.read_resource::<ScreenDimensions>();
//     let x = size.width() / 2.0;
//     let y = size.height() / 2.0;
//     drop(size);
//     ball::create_ball(world, BALL_RADIUS, x, y);
// }