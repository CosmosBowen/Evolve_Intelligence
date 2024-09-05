mod world;
mod food;
mod creature;
mod eye;
mod simulation;
mod creature_individual;

use food::*;
use creature::*;
use simulation::*;

use std::f32::consts::PI;

use ggez::{Context, GameResult};
use ggez::glam::Vec2;
use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh};
use ggez::event::{self, EventHandler};
use ggez::conf::{WindowMode, WindowSetup};

pub const WINDOW_WIDTH: f32 = 2500.0; // 1920.0
pub const WINDOW_HEIGHT: f32 = 1500.0; // 1080.0
// const MAX_EVOLUTION_EPOCH: i32 = 50;

struct MainState {
    simulation: Simulation,
}

impl MainState {
    fn new(ctx: &Context) -> GameResult<MainState> {
        let (width, height) = ctx.gfx.size();
        let s = MainState { 
            simulation: Simulation::new(width, height),
        };
        Ok(s)
    }
}

impl EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(60) {
            self.simulation.update();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {

        let background_color = Color::new(
            31.0 / 255.0,  // R: 1f in hex is 31 in decimal
            38.0 / 255.0,  // G: 26 in hex is 38 in decimal
            57.0 / 255.0,  // B: 39 in hex is 57 in decimal
            1.0,           // A: fully opaque
        );

        let mut canvas = graphics::Canvas::from_frame(ctx, background_color);

        // Draw food
        for food in &self.simulation.world.foods {
            if food.is_eaten == false {
                let circle = Mesh::new_circle(
                    ctx,
                    DrawMode::fill(),
                    Vec2::new(food.position.x, food.position.y),
                    FOOD_SIZE,
                    0.1,
                    Color::GREEN,
                )?;
                canvas.draw(&circle, DrawParam::default())
            }
        }

        // Draw creature
        for creature in &self.simulation.world.creatures {
            
            // body
            let triangle = Mesh::new_polygon(
                ctx,
                DrawMode::fill(),
                &[
                    Vec2::new(
                        creature.position.x + creature.rotation.sin() * BODY_LENGTH, 
                        creature.position.y - creature.rotation.cos() * BODY_LENGTH),
                    Vec2::new(
                        creature.position.x + (creature.rotation + 2.0/3.0 * PI).sin() * CREATURE_SIZE, 
                        creature.position.y - (creature.rotation + 2.0/3.0 * PI).cos() * CREATURE_SIZE),
                    Vec2::new(
                        creature.position.x + (creature.rotation + 4.0/3.0 * PI).sin() * CREATURE_SIZE, 
                        creature.position.y - (creature.rotation + 4.0/3.0 * PI).cos() * CREATURE_SIZE),
                ],
                creature.color,
            )?;
            canvas.draw(&triangle, DrawParam::default());

            // eye
            let circle = Mesh::new_circle(
                ctx,
                DrawMode::fill(),
                Vec2::new(
                    creature.position.x + creature.rotation.sin() * EYE_POSITION, 
                    creature.position.y - creature.rotation.cos() * EYE_POSITION),
                EYE_SIZE,
                0.1,
                Color::WHITE,
            )?;
            canvas.draw(&circle, DrawParam::default());


            // eyeball
            let circle = Mesh::new_circle(
                ctx,
                DrawMode::fill(),
                Vec2::new(
                    creature.position.x + creature.rotation.sin() * EYEBALL_POSITION, 
                    creature.position.y - creature.rotation.cos() * EYEBALL_POSITION),
                EYEBALL_SIZE,
                0.1,
                Color::BLACK,
            )?;
            canvas.draw(&circle, DrawParam::default());
            
            // mouth
            let mouth_color = Color::from_rgb(255,182,193);
            let circle = Mesh::new_circle(
                ctx,
                DrawMode::fill(),
                Vec2::new(
                    creature.position.x + (creature.rotation - MOUTH_OPEN_ANGLE).sin() * MOUTH_POSITION, 
                    creature.position.y - (creature.rotation - MOUTH_OPEN_ANGLE).cos() * MOUTH_POSITION),
                MOUTH_SIZE,
                0.1,
                mouth_color,
            )?;
            canvas.draw(&circle, DrawParam::default());
            let circle = Mesh::new_circle(
                ctx,
                DrawMode::fill(),
                Vec2::new(
                    creature.position.x + (creature.rotation + MOUTH_OPEN_ANGLE).sin() * MOUTH_POSITION, 
                    creature.position.y - (creature.rotation + MOUTH_OPEN_ANGLE).cos() * MOUTH_POSITION),
                MOUTH_SIZE,
                0.1,
                mouth_color,
            )?;
            canvas.draw(&circle, DrawParam::default());
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("evolution_sim", "Bowen")
        .window_setup(WindowSetup::default().title("Evolution Simulation"))
        .window_mode(WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT));
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new(&ctx)?;
    event::run(ctx, event_loop, state)
}
