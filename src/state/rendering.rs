use super::{Action, State, StickEnd, RADIUS, TICK_DURATION};
use ggez::{
    graphics::{self, DrawMode, DrawParam, Mesh},
    Context, GameResult,
};
use glam::DVec2;

const STICK_WIDTH: f32 = 5.;

const BACKGROUND: (u8, u8, u8) = (61, 64, 112);
const POINT_COLOUR: (u8, u8, u8) = (255, 255, 255);
const LOCKED_COLOUR: (u8, u8, u8) = (255, 0, 0);
const STICK_COLOUR: (u8, u8, u8) = (203, 203, 212);

impl State {
    pub fn render(&self, ctx: &mut Context) -> GameResult {
        let draw_param = DrawParam::default();
        let camera = self.camera.as_dvec2();
        let t = self.accumulator / TICK_DURATION;

        let ropes = if self.simulating {
            &self.active
        } else {
            &self.saved
        };

        graphics::clear(ctx, BACKGROUND.into());

        for (a, b) in ropes.get_sticks(t) {
            let mesh = stick_mesh(ctx, a - camera, b - camera)?;
            graphics::draw(ctx, &mesh, draw_param)?;
        }

        if let Action::CreatingStick(start, end) = &self.action {
            let a = ropes.get_position(*start, t) - camera;
            let b = match end {
                StickEnd::Key(key) => ropes.get_position(*key, t) - camera,
                StickEnd::Mouse(pos) => (*pos).as_dvec2(),
            };
            if (a - b).length_squared() >= RADIUS * RADIUS {
                let mesh = stick_mesh(ctx, a, b)?;
                graphics::draw(ctx, &mesh, draw_param)?;
            }
        }

        if let Action::CreatingLine(Some((key, mouse))) = self.action {
            let a = ropes.get_position(key, t) - camera;
            let b = mouse.as_dvec2();
            if (a - b).length_squared() >= RADIUS * RADIUS {
                let mesh = stick_mesh(ctx, a, b)?;
                graphics::draw(ctx, &mesh, draw_param)?;
            }
        }

        let point = graphics::Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            [0., 0.],
            RADIUS as f32,
            0.4,
            (255, 255, 255).into(),
        )?;
        for (position, locked) in ropes.get_points(t) {
            let draw_param = draw_param
                .color(if locked { LOCKED_COLOUR } else { POINT_COLOUR }.into())
                .dest(as_point(position - camera));
            graphics::draw(ctx, &point, draw_param)?;
        }

        graphics::present(ctx)
    }
}

fn stick_mesh(ctx: &mut Context, a: DVec2, b: DVec2) -> GameResult<Mesh> {
    graphics::Mesh::new_line(
        ctx,
        &[as_point(a), as_point(b)],
        STICK_WIDTH,
        STICK_COLOUR.into(),
    )
}

fn as_point(v: DVec2) -> [f32; 2] {
    [v.x as f32, v.y as f32]
}
