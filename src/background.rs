/*
Copyright (c) 2018 Matt Carnovale, Julie Rutherford-Fields, Joshua Sander
This work is available under the "MIT License”.
Please see the file LICENSE in this distribution
for license terms.
*/

use constants::{CUB_NUM, END, MID_ROW, NUM_LANE, NUM_LOG, NUM_ROW, RIVER, ROAD, SQUARE_SIZE,
                WIN_H, WIN_W};

use ggez::graphics;
use ggez::{Context, GameResult};
use sprites::Rectangle;

/// Represents the 'road' region of the game environment
pub struct Road {
    form: Rectangle,
}

/// Represents the 'river' region of the game environment
pub struct River {
    form: Rectangle,
}

/// Represents the 'cubbie' regions of the game environment
pub struct Cubbie {
    form: Rectangle,
    is_occupied: bool,
}

/// Represents a vector of cubbies which is scalable
/// depending on the window dimensions
pub struct Cubbies {
    cubbies: Vec<Cubbie>,
    filled_cubbies: u32,
}

/// Represents the game start menu
pub struct Menu {}

/// Implements the road.
impl Road {
    /// Creates a new road which is scalable depending
    /// on the window dimension constants
    pub fn new(w: u32, h: u32) -> Road {
        Road {
            form: Rectangle::construct(
                0.0,
                h as f32 - (MID_ROW - 1) as f32 * SQUARE_SIZE,
                w as f32,
                NUM_LANE as f32 * SQUARE_SIZE,
                ROAD,
            ),
        }
    }

    /// Draws the road graphic on screen
    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.form.draw(ctx)?;
        Ok(())
    }
}

/// Implements the river.
impl River {
    /// Creates a new river which is scalable depending
    /// on the window dimension constants
    pub fn new(w: u32, h: u32) -> River {
        River {
            form: Rectangle::construct(
                0.0,
                h as f32 - (NUM_ROW as f32 - 3.0) * SQUARE_SIZE,
                w as f32,
                NUM_LOG as f32 * SQUARE_SIZE,
                RIVER,
            ),
        }
    }

    /// Draws the river graphic on screen
    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.form.draw(ctx)?;
        Ok(())
    }
}

/// Implements the cubbie.
impl Cubbie {
    /// Creates a new river which is scalable depending
    /// on the window dimension constants
    pub fn new(x: f32) -> Cubbie {
        Cubbie {
            form: Rectangle::construct(
                x,
                END - 2.0 * SQUARE_SIZE,
                2.0 * SQUARE_SIZE,
                2.0 * SQUARE_SIZE,
                RIVER,
            ),
            is_occupied: false,
        }
    }

    /// Draws the cubbie graphic on screen
    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.form.draw(ctx)?;
        Ok(())
    }
}

/// Implements a vector of cubbies
impl Cubbies {
    /// Constructs a a Cubbies object.
    pub fn construct() -> Cubbies {
        Cubbies {
            cubbies: Cubbies::create_cubbies(),
            filled_cubbies: CUB_NUM,
        }
    }

    /// Creates a vector and populates it with a number of
    /// cubbies calculated from window dimensions
    fn create_cubbies() -> Vec<Cubbie> {
        let mut cubbies = vec![];
        for i in 0..CUB_NUM {
            let x = i as f32 * (4.0 * SQUARE_SIZE) + 2.0 * SQUARE_SIZE;
            cubbies.push(Cubbie::new(x))
        }
        cubbies
    }

    /// Draws each cubbie in the vector and manages which
    /// ones are occupied with a crab sprite
    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        for cubbie in &mut self.cubbies {
            cubbie.draw(ctx)?;
            if cubbie.is_occupied == true {
                let image_cubbie_crab = graphics::Image::new(ctx, "/tiny_crab.png")?;
                let dest_point = graphics::Point2::new(
                    cubbie.form.x + SQUARE_SIZE / 2.0,
                    cubbie.form.y + SQUARE_SIZE / 2.0,
                );
                graphics::draw(ctx, &image_cubbie_crab, dest_point, 0.0)?;
            }
        }
        Ok(())
    }

    /// Returns the status of whether a cubbie is occupied
    /// by the crab sprite
    pub fn get_is_occupied(&mut self, i: usize) -> bool {
        self.cubbies[i].is_occupied
    }

    /// Sets the status of whether a cubbie is occupied by
    /// the crab sprite and number of occupied cubbies
    pub fn set_is_occupied(&mut self, i: usize) {
        self.cubbies[i].is_occupied = true;
        self.filled_cubbies -= 1;
    }

    /// Returns the number of occupied cubbies
    pub fn get_filled_cubbies(&mut self) -> u32 {
        self.filled_cubbies
    }
}

/// Implements the game start menu
impl Menu {
    /// Draws the start menu graphics
    pub fn draw(&mut self, ctx: &mut Context, selection: u32) -> GameResult<()> {
        // Draw Crabber name upper-middle
        let game_name = format! {"CRABBER"};
        let font = graphics::Font::new(ctx, "/game_over.ttf", 56).unwrap();
        let name_text = graphics::Text::new(ctx, &game_name, &font)?;
        let horizontal: f32 = WIN_W as f32 / 2.0 - name_text.width() as f32 / 2.0;
        let vertical: f32 = WIN_H as f32 / 2.0 + SQUARE_SIZE * 1.0;
        let dest_point = graphics::Point2::new(horizontal, vertical);
        graphics::draw(ctx, &name_text, dest_point, 0.0)?;

        // Draw the crab logo sprite
        let image_big_crab = graphics::Image::new(ctx, "/crab.png")?;
        let horizontal_crab: f32 = WIN_W as f32 / 2.0 - image_big_crab.width() as f32 / 2.0;
        let vertical_crab: f32 = vertical - SQUARE_SIZE * 8.0;
        let dest_point = graphics::Point2::new(horizontal_crab, vertical_crab);
        graphics::draw(ctx, &image_big_crab, dest_point, 0.0)?;

        // Draw Start option
        let start = format! {"Start"};
        let font_start = graphics::Font::new(ctx, "/game_over.ttf", 20).unwrap();
        let start_text = graphics::Text::new(ctx, &start, &font_start)?;
        let horizontal2: f32 = WIN_W as f32 / 2.0 - start_text.width() as f32 / 2.0;
        let vertical2: f32 = WIN_H as f32 / 2.0 + SQUARE_SIZE * 5.0;
        let dest_point = graphics::Point2::new(horizontal2, vertical2);
        graphics::draw(ctx, &start_text, dest_point, 0.0)?;

        // Draw Scores option
        let scores = format! {"Scores"};
        let font_score = graphics::Font::new(ctx, "/game_over.ttf", 20).unwrap();
        let score_text = graphics::Text::new(ctx, &scores, &font_score)?;
        let horizontal3: f32 = WIN_W as f32 / 2.0 - score_text.width() as f32 / 2.0;
        let vertical3: f32 = vertical2 + SQUARE_SIZE;
        let dest_point = graphics::Point2::new(horizontal3, vertical3);
        graphics::draw(ctx, &score_text, dest_point, 0.0)?;

        if selection == 0 {
            let image_small_crab = graphics::Image::new(ctx, "/tiny_crab.png")?;
            let horizontal_small_crab: f32 = horizontal2 - 30.0;
            let dest_point = graphics::Point2::new(horizontal_small_crab, vertical2 + 10.0);
            graphics::draw(ctx, &image_small_crab, dest_point, 0.0)?;
        } else if selection == 1 {
            let image_small_crab = graphics::Image::new(ctx, "/tiny_crab.png")?;
            let horizontal_small_crab: f32 = horizontal3 - 30.0;
            let dest_point = graphics::Point2::new(horizontal_small_crab, vertical3 + 10.0);
            graphics::draw(ctx, &image_small_crab, dest_point, 0.0)?;
        }

        Ok(())
    }
}
