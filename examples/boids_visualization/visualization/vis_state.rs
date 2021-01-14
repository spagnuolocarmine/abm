use crate::model::bird::Bird;
use crate::model::boids_state::{BoidsState, DISCRETIZATION, HEIGHT, TOROIDAL, WIDTH};
use crate::NUM_AGENT;
use abm::location::Real2D;
use abm::visualization::on_state_init::OnStateInit;
use abm::visualization::renderable::{Render, SpriteType};
use abm::visualization::sprite_render_factory::SpriteRenderFactory;
use abm::Schedule;
use amethyst::prelude::{World, WorldExt};
use rand::Rng;

pub struct VisState;

impl OnStateInit for VisState {
    fn on_init(&self, world: &mut World, sprite_render_factory: &mut SpriteRenderFactory) {
        let state = BoidsState::new(WIDTH, HEIGHT, DISCRETIZATION, TOROIDAL);
        let schedule: Schedule<Bird> = Schedule::new();
        let mut rng = rand::thread_rng();
        world.register::<Bird>();
        for bird_id in 0..NUM_AGENT {
            let r1: f64 = rng.gen();
            let r2: f64 = rng.gen();
            let last_d = Real2D { x: 0., y: 0. };
            let pos = Real2D {
                x: WIDTH * r1,
                y: HEIGHT * r2,
            };
            let bird = Bird::new(bird_id, pos, last_d);
            state.field1.set_object_location(bird, pos);

            schedule.schedule_repeating(bird, 0., 0);
            let SpriteType::Emoji(emoji_code) = bird.sprite();
            let sprite_render = sprite_render_factory.get_emoji_loader(emoji_code, world);
            bird.setup_graphics(sprite_render, world, &state);
        }
        world.insert(state);
        world.insert(schedule);
    }
}