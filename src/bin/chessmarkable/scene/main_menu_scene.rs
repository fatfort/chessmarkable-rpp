use crate::rmpp_hal::types::{InputEvent, MultitouchEvent};
use super::Scene;
use crate::canvas::*;

pub struct MainMenuScene {
    drawn: bool,

    play_pvp_button_hitbox: Option<mxcfb_rect>,
    pub play_pvp_button_pressed: bool,
    pvp_toggle_piece_rotation_hitbox: Option<mxcfb_rect>,
    pvp_toggle_piece_rotation_redraw: bool,
    play_easy_button_hitbox: Option<mxcfb_rect>,
    pub play_easy_button_pressed: bool,
    play_normal_button_hitbox: Option<mxcfb_rect>,
    pub play_normal_button_pressed: bool,
    play_hard_button_hitbox: Option<mxcfb_rect>,
    pub play_hard_button_pressed: bool,
    viewer_button_hitbox: Option<mxcfb_rect>,
    pub viewer_button_pressed: bool,

    exit_button_hitbox: Option<mxcfb_rect>,
    pub exit_button_pressed: bool,

    exit_xochitl_button_hitbox: Option<mxcfb_rect>,
    pub exit_xochitl_button_pressed: bool,

    only_exit_to_xochitl: bool,
    pub pvp_piece_rotation_enabled: bool,
}

impl MainMenuScene {
    pub fn new(only_exit_to_xochitl: bool, pvp_piece_rotation_enabled: bool) -> Self {
        Self {
            drawn: false,
            play_pvp_button_hitbox: None,
            play_pvp_button_pressed: false,
            pvp_toggle_piece_rotation_hitbox: None,
            pvp_toggle_piece_rotation_redraw: false,
            play_easy_button_hitbox: None,
            play_easy_button_pressed: false,
            play_normal_button_hitbox: None,
            play_normal_button_pressed: false,
            play_hard_button_hitbox: None,
            play_hard_button_pressed: false,
            viewer_button_hitbox: None,
            viewer_button_pressed: false,
            exit_button_hitbox: None,
            exit_button_pressed: false,
            exit_xochitl_button_hitbox: None,
            exit_xochitl_button_pressed: false,
            only_exit_to_xochitl,
            pvp_piece_rotation_enabled,
        }
    }

    fn draw_rotation_button(&mut self, canvas: &mut Canvas) {
        let sp = |p: i32| -> i32 { (p as f32 * crate::BUTTON_SCALE) as i32 };
        let sf = |f: f32| -> f32 { f * crate::BUTTON_SCALE };
        let box_sz = sp(50) as u32;
        let box_pad = sp(10);
        if let Some(hitbox) = self.pvp_toggle_piece_rotation_hitbox {
            // Extend hitbox fully horizontal to accommodate enlargement of button
            canvas.fill_rect(
                Point2 {
                    x: Some(0),
                    y: Some((hitbox.top) as i32),
                },
                Vector2 {
                    x: crate::DISPLAY_WIDTH,
                    y: hitbox.height,
                },
                color::WHITE,
            );
        }
        // Position rotate button dynamically below the Play! button
        let rotate_y = if let Some(ref hitbox) = self.play_pvp_button_hitbox {
            hitbox.top as i32 + hitbox.height as i32 + sp(15)
        } else {
            (780.0 * crate::BUTTON_SCALE) as i32
        };
        self.pvp_toggle_piece_rotation_hitbox = Some(canvas.draw_text(
            Point2 {
                x: None,
                y: Some(rotate_y),
            },
            "      Rotate board",
            sf(50.0),
        ));

        canvas.draw_rect(
            Point2 {
                x: Some(self.pvp_toggle_piece_rotation_hitbox.unwrap().left as i32),
                y: Some((self.pvp_toggle_piece_rotation_hitbox.unwrap().top as i32) - box_pad),
            },
            Vector2 { x: box_sz, y: box_sz },
            2,
        );

        if self.pvp_piece_rotation_enabled {
            let inset = sp(4) as u32;
            canvas.fill_rect(
                Point2 {
                    x: Some(self.pvp_toggle_piece_rotation_hitbox.unwrap().left as i32 + inset as i32),
                    y: Some((self.pvp_toggle_piece_rotation_hitbox.unwrap().top as i32) - box_pad + inset as i32),
                },
                Vector2 {
                    x: box_sz - inset * 2,
                    y: box_sz - inset * 2,
                },
                color::BLACK,
            );
        }

        // Update button and everything to left and right since button may have enlarged
        let mut ext_hitbox = self.pvp_toggle_piece_rotation_hitbox.clone().unwrap();
        ext_hitbox.top = (ext_hitbox.top as i32 - box_pad).max(0) as u32;
        ext_hitbox.height += (box_pad * 2) as u32;
        self.pvp_toggle_piece_rotation_hitbox = Some(ext_hitbox);
    }
}

impl Scene for MainMenuScene {
    fn draw(&mut self, canvas: &mut Canvas) {
        if self.pvp_toggle_piece_rotation_redraw {
            self.draw_rotation_button(canvas);
            canvas.update_partial(&self.pvp_toggle_piece_rotation_hitbox.unwrap());
            self.pvp_toggle_piece_rotation_redraw = false;
        }

        if self.drawn {
            return;
        }
        self.drawn = true;

        canvas.clear();

        // sy/sp = layout-space scaling (BUTTON_SCALE only). sf/sbp add a
        // per-device MENU_BTN_BUMP on top so menu buttons are 0.8× on
        // Ferrari (baseline was too chunky) and 1.2× on Move (baseline
        // was too small after the 0.589 width-fit shrink).
        let sy = |y: i32| -> i32 { (y as f32 * crate::BUTTON_SCALE) as i32 };
        let sp = |p: u32| -> u32 { (p as f32 * crate::BUTTON_SCALE) as u32 };
        let sf = |f: f32| -> f32 { f * crate::BUTTON_SCALE * crate::MENU_BTN_BUMP };
        let sbp = |p: u32| -> u32 {
            (p as f32 * crate::BUTTON_SCALE * crate::MENU_BTN_BUMP) as u32
        };

        canvas.draw_text(
            Point2 {
                x: None,
                y: Some(sy(200)),
            },
            "chessMarkable",
            sf(150.0),
        );

        canvas.draw_text(
            Point2 {
                x: None,
                y: Some(sy(400)),
            },
            "Player vs Player",
            sf(75.0),
        );

        self.play_pvp_button_hitbox = Some(canvas.draw_button(
            Point2 {
                x: None,
                y: Some(sy(530)),
            },
            "Play!",
            sf(125.0),
            sbp(25),
            sbp(50),
        ));

        self.draw_rotation_button(canvas);

        canvas.draw_text(
            Point2 {
                x: None,
                y: Some(sy(800)),
            },
            "Player vs Bot",
            sf(75.0),
        );

        self.play_easy_button_hitbox = Some(canvas.draw_button(
            Point2 {
                x: None,
                y: Some(sy(920)),
            },
            "Easy",
            sf(125.0),
            sbp(25),
            sbp(50),
        ));
        self.play_normal_button_hitbox = Some(canvas.draw_button(
            Point2 {
                x: None,
                y: Some(
                    sy(100) + self.play_easy_button_hitbox.unwrap().top as i32
                        + self.play_easy_button_hitbox.unwrap().height as i32,
                ),
            },
            "Normal",
            sf(125.0),
            sbp(25),
            sbp(50),
        ));
        self.play_hard_button_hitbox = Some(canvas.draw_button(
            Point2 {
                x: None,
                y: Some(
                    sy(100) + self.play_normal_button_hitbox.unwrap().top as i32
                        + self.play_normal_button_hitbox.unwrap().height as i32,
                ),
            },
            "Hard",
            sf(125.0),
            sbp(25),
            sbp(50),
        ));
        self.viewer_button_hitbox = Some(canvas.draw_button(
            Point2 {
                x: None,
                y: Some(
                    sy(120) + self.play_hard_button_hitbox.unwrap().top as i32
                        + self.play_hard_button_hitbox.unwrap().height as i32,
                ),
            },
            "PGN Viewer",
            sf(125.0),
            sbp(25),
            sbp(50),
        ));

        // Exit button in top-right corner
        if self.only_exit_to_xochitl {
            self.exit_xochitl_button_hitbox = Some(canvas.draw_button(
                Point2 {
                    x: Some(crate::DISPLAY_WIDTH as i32 - sp(450) as i32),
                    y: Some(sy(30)),
                },
                "Exit to Xochitl",
                sf(50.0),
                sbp(15),
                sbp(30),
            ));
        } else {
            self.exit_button_hitbox = Some(canvas.draw_button(
                Point2 {
                    x: Some(crate::DISPLAY_WIDTH as i32 - sp(200) as i32),
                    y: Some(sy(30)),
                },
                "Exit",
                sf(50.0),
                sbp(15),
                sbp(30),
            ));
        }

        canvas.update_full();
    }

    fn on_input(&mut self, event: InputEvent) {
        if let InputEvent::MultitouchEvent { event } = event {
            if let MultitouchEvent::Release { finger, .. } = event {
                let position = finger.pos;
                if self.play_pvp_button_hitbox.is_some()
                    && Canvas::is_hitting(position, self.play_pvp_button_hitbox.unwrap())
                {
                    self.play_pvp_button_pressed = true;
                }
                if self.pvp_toggle_piece_rotation_hitbox.is_some()
                    && Canvas::is_hitting(position, self.pvp_toggle_piece_rotation_hitbox.unwrap())
                {
                    self.pvp_piece_rotation_enabled = !self.pvp_piece_rotation_enabled;
                    self.pvp_toggle_piece_rotation_redraw = true;
                } else if self.play_easy_button_hitbox.is_some()
                    && Canvas::is_hitting(position, self.play_easy_button_hitbox.unwrap())
                {
                    self.play_easy_button_pressed = true;
                } else if self.play_normal_button_hitbox.is_some()
                    && Canvas::is_hitting(position, self.play_normal_button_hitbox.unwrap())
                {
                    self.play_normal_button_pressed = true;
                } else if self.play_hard_button_hitbox.is_some()
                    && Canvas::is_hitting(position, self.play_hard_button_hitbox.unwrap())
                {
                    self.play_hard_button_pressed = true;
                } else if self.viewer_button_hitbox.is_some()
                    && Canvas::is_hitting(position, self.viewer_button_hitbox.unwrap())
                {
                    self.viewer_button_pressed = true;
                } else if self.exit_button_hitbox.is_some()
                    && Canvas::is_hitting(position, self.exit_button_hitbox.unwrap())
                {
                    self.exit_button_pressed = true;
                } else if self.exit_xochitl_button_hitbox.is_some()
                    && Canvas::is_hitting(position, self.exit_xochitl_button_hitbox.unwrap())
                {
                    self.exit_xochitl_button_pressed = true;
                }
            }
        }
    }
}
