use std::ops::Range;
use bracket_lib::prelude::*;

const FRAME_DURATION: f32 = 75.0;
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

enum GameMode {
    Menu,
    Playing,
    End,
}

struct State {
    mode: GameMode,
    player: Player,
    frame_time: f32,
    obstacle: Obstacle,
    score: i32,
}

impl State {
    fn new() -> Self {
        Self {
            mode: GameMode::Menu,
            player: Player::new(5, 25.0),
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            score: 0,
        }
    }

    pub(crate) fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_color_centered(8, YELLOW, BLACK, "Welcome to flappy");
        ctx.print_color_centered(12, YELLOW, BLACK, "(P) Play Game");
        ctx.print_color_centered(16, YELLOW, BLACK, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    pub(crate) fn restart(&mut self) {
        self.mode = GameMode::Playing;
        self.player = Player::new(5, 25.0);
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.frame_time = 0.0;
        self.score = 0;
    }

    pub(crate) fn playing(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        ctx.print(1, 1, "Press Space to Flap");
        ctx.print_color_centered(1, YELLOW, NAVY, &format!("Score {}", self.score));

        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        self.player.render(ctx);
        self.obstacle.render(self.player.x, ctx);

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }

        if self.player.y as i32 > SCREEN_HEIGHT || self.obstacle.hit_obstracle(&self.player) {
            self.mode = GameMode::End;
        }
    }

    pub(crate) fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_color_centered(8, YELLOW, BLACK, "You are dead");
        ctx.print_color_centered(12, YELLOW, BLACK, &format!("You earned {} points", self.score));
        ctx.print_color_centered(16, YELLOW, BLACK, "(P) Play Game");
        ctx.print_color_centered(20, YELLOW, BLACK, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.playing(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}


fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy bird")
        .build()?;

    main_loop(context, State::new())
}

const PLAYER_START_POS_X:i32 = 5;

struct Player {
    x: i32,
    y: f32,
    velocity: f32,
}

impl Player {
    fn new(x: i32, y: f32) -> Self {
        Self {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(PLAYER_START_POS_X, self.y as i32, YELLOW, BLACK, to_cp437('@'))
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.1;
        }

        self.y += self.velocity;

        if self.y < 0.0 {
            self.y = 0.0;
        }

        self.x += 1;
    }

    fn flap(&mut self) {
        self.velocity = -1.0;
    }
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Self {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score),
        }
    }

    fn render(&mut self, player_x: i32, ctx: &mut BTerm) {
        let screen_x = self.x + PLAYER_START_POS_X - player_x;

        let (above_gap, below_gap) = self.above_below_gap();
        self.draw_obstacle(ctx, screen_x, 0..above_gap);
        self.draw_obstacle(ctx, screen_x, below_gap..SCREEN_HEIGHT);
    }

    fn above_below_gap(&mut self) -> (i32, i32) {
        let half_size = self.size / 2;
        (self.gap_y - half_size, self.gap_y + half_size)
    }

    fn draw_obstacle(&mut self, ctx: &mut BTerm, screen_x: i32, range: Range<i32>) {
        for y in range {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('#'))
        }
    }

    fn hit_obstracle(&mut self, player: &Player) -> bool {
        let does_x_match = player.x == self.x;
        let (above_gap, below_gap) = self.above_below_gap();

        let player_above_gap = (player.y as i32) < above_gap;
        let player_below_gap = (player.y as i32) > below_gap;
        does_x_match && (player_above_gap || player_below_gap)
    }
}