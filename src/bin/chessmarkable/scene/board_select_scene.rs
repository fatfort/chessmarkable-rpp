use super::Scene;
use crate::canvas::*;
use crate::rmpp_hal::types::{InputEvent, MultitouchEvent};

pub struct BoardSelectScene {
    drawn: bool,

    pub selected_gamemode: crate::scene::GameMode,
    pub pvp_piece_rotation_enabled: bool,

    select_slot_1_button_hitbox: Option<mxcfb_rect>,
    pub select_slot_1_button_pressed: bool,
    select_slot_2_button_hitbox: Option<mxcfb_rect>,
    pub select_slot_2_button_pressed: bool,
    select_slot_3_button_hitbox: Option<mxcfb_rect>,
    pub select_slot_3_button_pressed: bool,

    reset_slot_1_button_hitbox: Option<mxcfb_rect>,
    pub reset_slot_1_button_pressed: bool,
    reset_slot_2_button_hitbox: Option<mxcfb_rect>,
    pub reset_slot_2_button_pressed: bool,
    reset_slot_3_button_hitbox: Option<mxcfb_rect>,
    pub reset_slot_3_button_pressed: bool,

    back_button_hitbox: Option<mxcfb_rect>,
    pub back_button_pressed: bool,

    indicate_loading: bool,
}

impl BoardSelectScene {
    pub fn new(
        selected_gamemode: crate::scene::GameMode,
        pvp_piece_rotation_enabled: bool,
    ) -> Self {
        Self {
            drawn: false,
            selected_gamemode,
            pvp_piece_rotation_enabled,
            select_slot_1_button_hitbox: None,
            select_slot_1_button_pressed: false,
            select_slot_2_button_hitbox: None,
            select_slot_2_button_pressed: false,
            select_slot_3_button_hitbox: None,
            select_slot_3_button_pressed: false,

            reset_slot_1_button_hitbox: None,
            reset_slot_1_button_pressed: false,
            reset_slot_2_button_hitbox: None,
            reset_slot_2_button_pressed: false,
            reset_slot_3_button_hitbox: None,
            reset_slot_3_button_pressed: false,
            back_button_hitbox: None,
            back_button_pressed: false,
            indicate_loading: false,
        }
    }

    fn indicate_loading(&self, canvas: &mut Canvas) {
        // Baseline y=280 (rMPP-space); shift 1cm up on both devices.
        let y = (280.0 * crate::BUTTON_SCALE) as i32 - crate::PX_PER_CM as i32;
        let rect = canvas.draw_text(
            Point2 { x: None, y: Some(y) },
            "Loading game... (preparing assets)",
            50.0 * crate::BUTTON_SCALE,
        );
        canvas.update_partial(&rect);
    }
}

impl Scene for BoardSelectScene {
    fn draw(&mut self, canvas: &mut Canvas) {
        if self.indicate_loading {
            self.indicate_loading(canvas);
            self.indicate_loading = false;
            return;
        }

        if self.drawn {
            return;
        }
        self.drawn = true;

        canvas.clear();

        // sy/sp = layout-space scaling (BUTTON_SCALE only). sf/sbp add
        // per-device MENU_BTN_BUMP — 0.8× on Ferrari, 1.2× on Move — so
        // slot/Back buttons read right on both panels.
        let sy = |y: i32| -> i32 { (y as f32 * crate::BUTTON_SCALE) as i32 };
        let sp = |p: u32| -> u32 { (p as f32 * crate::BUTTON_SCALE) as u32 };
        let sf = |f: f32| -> f32 { f * crate::BUTTON_SCALE * crate::MENU_BTN_BUMP };
        let sbp = |p: u32| -> u32 {
            (p as f32 * crate::BUTTON_SCALE * crate::MENU_BTN_BUMP) as u32
        };

        canvas.draw_text(
            Point2 {
                x: None,
                y: Some(sy(300)),
            },
            "chessMarkable",
            sf(150.0),
        );

        canvas.draw_text(
            Point2 {
                x: None,
                y: Some(sy(500)),
            },
            "Continue on...",
            sf(75.0),
        );

        self.select_slot_1_button_hitbox = Some(canvas.draw_button(
            Point2 {
                x: None,
                y: Some(sy(640)),
            },
            "Slot 1",
            sf(75.0),
            sbp(25),
            sbp(50),
        ));

        self.select_slot_2_button_hitbox = Some(canvas.draw_button(
            Point2 {
                x: None,
                y: Some(
                    sy(80) + self.select_slot_1_button_hitbox.unwrap().top as i32
                        + self.select_slot_1_button_hitbox.unwrap().height as i32,
                ),
            },
            "Slot 2",
            sf(75.0),
            sbp(25),
            sbp(50),
        ));

        self.select_slot_3_button_hitbox = Some(canvas.draw_button(
            Point2 {
                x: None,
                y: Some(
                    sy(80) + self.select_slot_2_button_hitbox.unwrap().top as i32
                        + self.select_slot_2_button_hitbox.unwrap().height as i32,
                ),
            },
            "Slot 3",
            sf(75.0),
            sbp(25),
            sbp(50),
        ));

        // Position "Start over" section dynamically below the last continue slot
        let start_over_y = self.select_slot_3_button_hitbox.unwrap().top as i32
            + self.select_slot_3_button_hitbox.unwrap().height as i32
            + sy(80);
        canvas.draw_text(
            Point2 {
                x: None,
                y: Some(start_over_y),
            },
            "Start over on...",
            sf(75.0),
        );

        let reset_slot_1_y = start_over_y + sy(160);
        self.reset_slot_1_button_hitbox = Some(canvas.draw_button(
            Point2 {
                x: None,
                y: Some(reset_slot_1_y),
            },
            "Slot 1",
            sf(75.0),
            sbp(25),
            sbp(50),
        ));

        self.reset_slot_2_button_hitbox = Some(canvas.draw_button(
            Point2 {
                x: None,
                y: Some(
                    sy(80) + self.reset_slot_1_button_hitbox.unwrap().top as i32
                        + self.reset_slot_1_button_hitbox.unwrap().height as i32,
                ),
            },
            "Slot 2",
            sf(75.0),
            sbp(25),
            sbp(50),
        ));

        self.reset_slot_3_button_hitbox = Some(canvas.draw_button(
            Point2 {
                x: None,
                y: Some(
                    sy(80) + self.reset_slot_2_button_hitbox.unwrap().top as i32
                        + self.reset_slot_2_button_hitbox.unwrap().height as i32,
                ),
            },
            "Slot 3",
            sf(75.0),
            sbp(25),
            sbp(50),
        ));

        // On Move the Back button was a touch too close to the bottom edge;
        // pull it up 0.5cm there. Ferrari leaves it where the rMPP layout
        // had it.
        #[cfg(feature = "move")]
        let back_shift = (crate::PX_PER_CM * 0.5) as i32;
        #[cfg(not(feature = "move"))]
        let back_shift: i32 = 0;
        self.back_button_hitbox = Some(canvas.draw_button(
            Point2 {
                x: None,
                y: Some(crate::DISPLAY_HEIGHT as i32 - sy(200) - back_shift),
            },
            "Back",
            sf(125.0),
            sbp(25),
            sbp(50),
        ));

        canvas.update_full();
    }

    fn on_input(&mut self, event: InputEvent) {
        if let InputEvent::MultitouchEvent { event } = event {
            if let MultitouchEvent::Release { finger, .. } = event {
                let position = finger.pos;
                if self.select_slot_1_button_hitbox.is_some()
                    && Canvas::is_hitting(position, self.select_slot_1_button_hitbox.unwrap())
                {
                    self.select_slot_1_button_pressed = true;
                    self.indicate_loading = true;
                } else if self.select_slot_2_button_hitbox.is_some()
                    && Canvas::is_hitting(position, self.select_slot_2_button_hitbox.unwrap())
                {
                    self.select_slot_2_button_pressed = true;
                    self.indicate_loading = true;
                } else if self.select_slot_3_button_hitbox.is_some()
                    && Canvas::is_hitting(position, self.select_slot_3_button_hitbox.unwrap())
                {
                    self.select_slot_3_button_pressed = true;
                    self.indicate_loading = true;
                } else if self.reset_slot_1_button_hitbox.is_some()
                    && Canvas::is_hitting(position, self.reset_slot_1_button_hitbox.unwrap())
                {
                    self.reset_slot_1_button_pressed = true;
                    self.indicate_loading = true;
                } else if self.reset_slot_2_button_hitbox.is_some()
                    && Canvas::is_hitting(position, self.reset_slot_2_button_hitbox.unwrap())
                {
                    self.reset_slot_2_button_pressed = true;
                    self.indicate_loading = true;
                } else if self.reset_slot_3_button_hitbox.is_some()
                    && Canvas::is_hitting(position, self.reset_slot_3_button_hitbox.unwrap())
                {
                    self.reset_slot_3_button_pressed = true;
                    self.indicate_loading = true;
                } else if self.back_button_hitbox.is_some()
                    && Canvas::is_hitting(position, self.back_button_hitbox.unwrap())
                {
                    self.back_button_pressed = true;
                }
            }
        }
    }
}
