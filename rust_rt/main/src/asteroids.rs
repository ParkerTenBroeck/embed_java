extern crate alloc;

use core::ops::Sub;

use alloc::boxed::Box;
use alloc::format;
use alloc::vec::Vec;
use num_traits::Zero;
use rlib::io::screen::ScreeenPoint;
use rlib::io::screen::Screen;
use rlib::io::screen::ScreenCommand;

use crate::asteroids::util::*;

#[derive(Default, Clone, Copy)]
struct FrameInfo {
    time_nano: u64,
    instructions_ran: u64,
}

struct RollingTracker {
    pos: usize,
    frames: [FrameInfo; 256],
}

impl RollingTracker {
    pub fn new() -> Self {
        Self {
            pos: 0,
            frames: [Default::default(); 256],
        }
    }

    pub fn average_frame_time(&self) -> f32 {
        let now = self.frames[self.pos];
        let oldest = self.frames[(self.pos as isize).sub(1) as usize & 0xFF];
        (oldest.time_nano.wrapping_sub(now.time_nano)) as f32 / 256.0 / 1000000000.0
    }

    pub fn average_instructions_per_frame(&self) -> f32 {
        let now = self.frames[self.pos];
        let oldest = self.frames[(self.pos as isize).sub(1) as usize & 0xFF];
        (oldest.instructions_ran.wrapping_sub(now.instructions_ran)) as f32 / 256.0
    }

    pub fn new_frame(&mut self, time_mill: u64, instructions_ran: u64) {
        self.frames[self.pos] = FrameInfo {
            time_nano: time_mill,
            instructions_ran,
        };
        self.pos += 1;
        self.pos &= 0xFF;
    }
}

pub struct Game {
    screen: Screen,
    frame: u32,
    ship: Ship,
    reset: bool,
    score: u32,
    level: u16,
    lives: u8,
    last_time: u64,
    alian_ship: Option<AlianShip>,
    asteroids: Vec<Asteroid>,
    bullets: Vec<Bullet>,
    particles: Vec<Particle>,
    debug_str: u16,
    t_time: f32,
    show_debug: bool,
    new_level_cooldown: Option<f32>,
    tracker: RollingTracker,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            screen: Default::default(),
            frame: 0,
            ship: Default::default(),
            reset: true,
            score: 0,
            last_time: 0,
            alian_ship: None,
            asteroids: Vec::new(),
            bullets: Vec::new(),
            level: 0,
            debug_str: 0,
            t_time: 0.0,
            lives: 3,
            particles: Vec::new(),
            show_debug: false,
            new_level_cooldown: None,
            tracker: RollingTracker::new(),
        }
    }

    pub fn draw_debug_str(&mut self, str: &str) {
        self.debug_str += 1;
        self.screen
            .push_command(ScreenCommand::SetColor([255, 255, 0, 255]));
        self.screen.push_command(ScreenCommand::Text(
            str,
            (0i16, self.debug_str as i16 * 15).into(),
        ));
    }

    pub fn run_frame(&mut self) {
        let instructions_ran = rlib::arch::get_instructions_ran();

        if self.reset {
            self.alian_ship = None;
            self.bullets.clear();
            self.asteroids.clear();
            self.particles.clear();
            self.reset = false;
            self.score = 0;
            self.level = 0;
            self.ship.after_death(&mut self.screen);
            self.lives = 3;
            self.last_time = rlib::arch::current_time_nanos();

            self.spawn_asteroids();
        }

        let now = rlib::arch::current_time_nanos();
        self.tracker.new_frame(now, instructions_ran);

        if rlib::arch::is_key_pressed('g') {
            self.spawn_asteroid();
        }

        if rlib::arch::is_key_pressed('q') {
            while rlib::arch::is_key_pressed('q') {}
            self.show_debug = !self.show_debug;
        }

        self.screen
            .push_command(ScreenCommand::SetColor([0, 0, 0, 255]));
        self.screen.push_command(ScreenCommand::Clear);

        // it would be more usful if uh we actaually calculated this but im lazy
        // and the sleep_d_ms does an *ok* job at making this true
        let d_time = (now - self.last_time) as f32 / 1_000_000_000.0; //0.025;

        self.screen
            .push_command(ScreenCommand::SetColor([255, 255, 255, 255]));

        self.bullets.retain_mut(|bullet| {
            bullet.pos.x += bullet.vel.x * d_time;
            bullet.pos.y += bullet.vel.y * d_time;
            bullet.pos.fit_to_screen(&mut self.screen);
            bullet.life -= d_time;

            let i = self.asteroids.iter().position(|asteroid| {
                let tmpx = asteroid.pos.x - bullet.pos.x;
                let tmpy = asteroid.pos.y - bullet.pos.y;
                let distance = tmpx * tmpx + tmpy * tmpy;
                distance < asteroid.max_rad * asteroid.max_rad
            });
            if let Some(i) = i {
                let ast = self.asteroids.remove(i);
                self.score += match ast.size {
                    3 => 20,
                    2 => 50,
                    1 => 100,
                    _ => 10_000, //wow you did the impossible
                };
                if ast.size != 1 {
                    let new = ast.split();
                    self.asteroids.push(new[0]);
                    self.asteroids.push(new[1]);
                }
                for _ in 0..20 {
                    let mut particle = Particle::new_point(ast.pos);
                    particle.vel.x += ast.vel.x;
                    particle.vel.y += ast.vel.y;
                    self.particles.push(particle);
                }
                return false;
            }

            let mut tmp = bullet.pos;
            tmp.x -= bullet.vel.x * d_time;
            tmp.y -= bullet.vel.y * d_time;
            self.screen
                .push_command(ScreenCommand::Line(bullet.pos.into(), tmp.into()));
            bullet.life > 0.0
        });

        self.screen
            .push_command(ScreenCommand::SetColor([255, 255, 255, 255]));
        self.particles.retain_mut(|particle| {
            particle.pos.x += particle.vel.x * d_time;
            particle.pos.y += particle.vel.y * d_time;
            particle.life -= d_time;
            if let Some(c) = &particle.color_fn {
                self.screen
                    .push_command(ScreenCommand::SetColor(c(particle)));
            }
            if let Some(ls) = particle.ls.as_mut() {
                ls.0.x += particle.vel.x * d_time;
                ls.0.y += particle.vel.y * d_time;
                //ls.0.fit_to_screen(&mut self.screen);
                ls.1 += ls.2 * d_time;
                let mut center = Vector::new(
                    (ls.0.x + particle.pos.x) / 2.0,
                    (ls.0.y + particle.pos.y) / 2.0,
                );

                let p1 = Vector::new(ls.0.x - center.x, ls.0.y - center.y);
                let p2 = Vector::new(particle.pos.x - center.x, particle.pos.y - center.y);

                center.fit_to_screen(&mut self.screen);

                let (sin, cos) = libm::sincosf(ls.1);
                let t = calculate_screen_points([p1, p2], sin, cos, center);

                self.screen
                    .push_command_with_wrap(ScreenCommand::Line(t[0], t[1]));
            } else {
                particle.pos.fit_to_screen(&mut self.screen);
                self.screen
                    .push_command(ScreenCommand::Pixel(particle.pos.into()));
            }
            if particle.color_fn.is_some() {
                self.screen
                    .push_command(ScreenCommand::SetColor([255, 255, 255, 255]));
            }
            particle.life > 0.0
        });


        self.screen
            .push_command(ScreenCommand::SetColor([255, 255, 255, 255]));
        for asteroid in &mut self.asteroids {
            asteroid.pos.x += asteroid.vel.x * d_time;
            asteroid.pos.y += asteroid.vel.y * d_time;
            asteroid.pos.fit_to_screen(&mut self.screen);
            let x = asteroid.pos.x as i16;
            let y = asteroid.pos.y as i16;
            let mut d_p = [ScreeenPoint::default(); 12];
            for (d_p, a_p) in d_p.iter_mut().zip(asteroid.points.iter()) {
                d_p.x = a_p.0 + x;
                d_p.y = a_p.1 + y;
            }
            if self.show_debug {
                self.screen
                    .push_command(ScreenCommand::SetColor([0, 0, 255, 255]));
                self.screen.push_command_with_wrap(ScreenCommand::Oval(
                    (x, y).into(),
                    (asteroid.max_rad as i16, asteroid.max_rad as i16).into(),
                ));
                self.screen
                    .push_command(ScreenCommand::SetColor([255, 255, 255, 255]));
            }
            self.screen
                .push_command_with_wrap(ScreenCommand::Polygon(&d_p));
        }

        if self.show_debug {
            self.screen
                .push_command(ScreenCommand::SetColor([0, 0, 255, 255]));
            self.screen.push_command_with_wrap(ScreenCommand::Oval(
                self.ship.pos.into(),
                (SHIP_MAX_RADIUS as i16, SHIP_MAX_RADIUS as i16).into(),
            ));
        }

        let hit = Ship::update(self, d_time);

        let hit = match hit {
            ShipCollision::Asteroid(index) => {
                let ast = self.asteroids.remove(index);
                if ast.size != 1 {
                    let new = ast.split();
                    self.asteroids.push(new[0]);
                    self.asteroids.push(new[1]);
                }
                for _ in 0..20 {
                    let mut particle = Particle::new_point(ast.pos);
                    particle.vel.x += ast.vel.x;
                    particle.vel.y += ast.vel.y;
                    self.particles.push(particle);
                }
                true
            }
            ShipCollision::Bullet(index) => {
                self.bullets.remove(index);
                true
            }
            ShipCollision::AlianShip => {
                self.alian_ship = None;
                true
            }
            ShipCollision::None => false,
        };
        if hit {
            if self.lives == 0 {
                self.reset = true;
            } else {
                for _ in 0..20 {
                    let mut particle = Particle::new_point(self.ship.pos);
                    particle.vel.x += self.ship.vel.x;
                    particle.vel.y += self.ship.vel.y;
                    self.particles.push(particle);
                }
                self.ship.after_death(&mut self.screen);
                self.lives -= 1;
            }
        }

        if rlib::arch::is_key_pressed('r') {
            rlib::process::exit(0);
        }

        let spawn_new = if let Some(cool_down) = self.new_level_cooldown.as_mut() {
            *cool_down -= d_time;
            *cool_down <= 0.0
        } else {
            false
        };
        if spawn_new {
            self.spawn_asteroids();
            self.new_level_cooldown = None;
        }
        if self.asteroids.is_empty() && self.new_level_cooldown.is_none() {
            self.new_level_cooldown = Some(2.0);
            self.level += 1;
        }


        self.screen
            .push_command(ScreenCommand::SetColor([255, 255, 255, 255]));

        let str = format!("score: {}", self.score);
        self.screen.push_command(ScreenCommand::Text(
            &str,
            (
                self.screen.get_width() / 2 - str.len() as i16 * 5 / 2,
                10i16,
            )
                .into(),
        ));

        let str = format!("level: {}", self.level + 1);
        self.screen.push_command(ScreenCommand::Text(
            &str,
            (
                self.screen.get_width() / 2 - str.len() as i16 * 5 / 2,
                25i16,
            )
                .into(),
        ));

        let str = format!("lives: {}", self.lives);
        self.screen.push_command(ScreenCommand::Text(
            &str,
            (
                self.screen.get_width() / 2 - str.len() as i16 * 5 / 2,
                40i16,
            )
                .into(),
        ));

        let str = format!("fps: {:.0}", 1.0 / self.tracker.average_frame_time());
        self.screen.push_command(ScreenCommand::Text(
            &str,
            (
                self.screen.get_width() / 2 - str.len() as i16 * 5 / 2,
                55i16,
            )
                .into(),
        ));
        let str = format!("ipf: {:.0}", self.tracker.average_instructions_per_frame());
        self.screen.push_command(ScreenCommand::Text(
            &str,
            (
                self.screen.get_width() / 2 - str.len() as i16 * 5 / 2,
                70i16,
            )
                .into(),
        ));

        if self.show_debug {
            self.draw_debug_str(&format!("particles: {:.2}", self.particles.len()));
            self.draw_debug_str(&format!("asteroids: {:.2}", self.asteroids.len()));
            self.draw_debug_str(&format!("bullets: {:.2}", self.bullets.len()));
            self.draw_debug_str(&format!("d_time: {:.4}s", d_time));
            self.draw_debug_str(&format!("t_time: {:.2}s", self.t_time));
            self.draw_debug_str(&format!("frames: {:.2}", self.frame));
            self.draw_debug_str(&format!("fps: {:.2}", self.frame as f32 / self.t_time));
            self.draw_debug_str(&format!("draw_call_size: {}B", self.screen.get_call_len()));
        }

        self.screen.send_draw_call();
        self.frame += 1;
        self.t_time += d_time;
        self.debug_str = 0;
        self.last_time = now;

        if !rlib::arch::is_key_pressed('e') {
            rlib::arch::sleep_d_ms(16);
        }

    }

    fn spawn_asteroids(&mut self) {
        while self.asteroids.len() < (self.level + 5) as usize {
            self.spawn_asteroid()
        }
    }

    fn spawn_asteroid(&mut self){
        let x = rlib::arch::rand_range(0, self.screen.get_width() as i32) as f32;
        let y = rlib::arch::rand_range(0, self.screen.get_height() as i32) as f32;
        let asteroid = Asteroid::new(Vector::new(x, y), 3);
        let d_x = asteroid.pos.x - self.ship.pos.x;
        let d_y = asteroid.pos.y - self.ship.pos.y;
        let dis = d_x * d_x + d_y * d_y;
        if dis > 10000.0 {
            self.asteroids.push(asteroid);
        }
    }
}

//------------------------------------------------------------------------------------------

#[derive(Default)]
struct Ship {
    pos: Vector,
    vel: Vector,
    acc: Vector,
    angle: f32,
    invincible: f32,
    cant_shoot: f32,
    god: bool,
}

enum ShipCollision {
    Asteroid(usize),
    #[allow(unused)]
    Bullet(usize),
    #[allow(unused)]
    AlianShip,
    None,
}

const SHIP_MAX_RADIUS: f32 = 15.0;
const SHIP_POINTS: [Vector; 4] = [
    Vector::new(12.0, 0.0),
    Vector::new(-12.0, -9.0),
    Vector::new(-9.0, 0.0),
    Vector::new(-12.0, 9.0),
];
const SHIP_FIRE_INNER: [Vector; 3] = [
    Vector::new(-10.0, 2.0),
    Vector::new(-17.3333, 0.0),
    Vector::new(-10.0, -2.0),
];
const SHIP_FIRE_OUTER: [Vector; 3] = [
    Vector::new(-10.5, 4.5),
    Vector::new(-22.5, 0.0),
    Vector::new(-10.5, -4.5),
];

impl Ship {
    pub fn update(game: &mut Game, d_time: f32) -> ShipCollision {
        if game.show_debug {
            game.draw_debug_str(&format!("acc_x: {:.2}", game.ship.acc.x));
            game.draw_debug_str(&format!("acc_y: {:.2}", game.ship.acc.y));
            game.draw_debug_str(&format!("vel_x: {:.2}", game.ship.vel.x));
            game.draw_debug_str(&format!("vel_y: {:.2}", game.ship.vel.y));
            game.draw_debug_str(&format!("pos_x: {:.2}", game.ship.pos.x));
            game.draw_debug_str(&format!("pos_y: {:.2}", game.ship.pos.y));
        }

        let Game {
            screen,
            ship,
            //alian_ship,
            particles,
            asteroids,
            t_time,
            bullets,
            ..
        } = game;

        let invincible = ship.invincible > 0.0;
        let should_draw = if invincible {
            if ship.invincible < 1.0 {
                libm::fmodf(*t_time, 0.1) > 0.05
            } else {
                libm::fmodf(*t_time, 0.2) > 0.1
            }
        } else {
            true
        };

        if rlib::arch::is_key_pressed('a') {
            ship.angle -= 4.0 * d_time;
        }
        if rlib::arch::is_key_pressed('d') {
            ship.angle += 4.0 * d_time;
        }
        if rlib::arch::is_key_pressed('i') {
            ship.god = !ship.god;
        }

        let (sin, cos) = libm::sincosf(ship.angle);

        let d_points = calculate_screen_points(SHIP_POINTS, sin, cos, ship.pos);
        if should_draw {
            screen.push_command(ScreenCommand::SetColor([255, 255, 255, 255]));
            screen.push_command_with_wrap(ScreenCommand::Polygon(&d_points));
        }

        if rlib::arch::is_key_pressed(' ') {
            if ship.cant_shoot == 0.0 {
                let bullet = Bullet {
                    pos: d_points[0].into(),
                    vel: Vector {
                        x: cos * 350.0 + ship.vel.x,
                        y: sin * 350.0 + ship.vel.y,
                    },
                    can_hurt_player: false,
                    life: 1.1,
                };
                if ship.god {
                    //ship.cant_shoot = 0.003;
                } else {
                    ship.cant_shoot = 0.4;
                }
                bullets.push(bullet);
            } else {
                ship.cant_shoot -= d_time;
                if ship.cant_shoot <= 0.0 {
                    ship.cant_shoot = 0.0;
                }
            }
        } else {
            ship.cant_shoot = 0.0;
        }

        if rlib::arch::is_key_pressed('w') {
            ship.acc.x = cos * 250.0;
            ship.acc.y = sin * 250.0;

            if should_draw {
                screen.push_command(ScreenCommand::SetColor([255, 255, 0, 255]));
                let mut p1 = calculate_screen_points(SHIP_FIRE_INNER, sin, cos, ship.pos);
                p1[1].x += rlib::arch::rand_range(-1, 1) as i16;
                p1[1].y += rlib::arch::rand_range(-1, 1) as i16;
                screen.push_command_with_wrap(ScreenCommand::PolygonLine(&p1));

                screen.push_command(ScreenCommand::SetColor([255, 0, 0, 255]));
                let mut p2 = calculate_screen_points(SHIP_FIRE_OUTER, sin, cos, ship.pos);
                p2[1].x += rlib::arch::rand_range(-1, 1) as i16;
                p2[1].y += rlib::arch::rand_range(-1, 1) as i16;

                for i in 0..(libm::ceilf(200.0 * d_time) as i32) {
                    let mut particle =
                        Particle::new_point(Vector::new(p1[1].x as f32, p1[1].y as f32));
                    particle.pos.x += i as f32 * cos;
                    particle.pos.y += i as f32 * sin;
                    particle.vel.x /= 2.0;
                    particle.vel.y /= 2.0;
                    particle.life /= 6.0;
                    let life = particle.life;
                    particle.vel.x += ship.vel.x + 260.0 * -cos;
                    particle.vel.y += ship.vel.y + 260.0 * -sin;
                    particle.color_fn = Some(Box::new(move |particle| {
                        let life = (particle.life / life * 255.0) as u8;
                        [life, life, life, 255]
                    }));
                    particles.push(particle);
                }
                screen.push_command_with_wrap(ScreenCommand::PolygonLine(&p2));
            }
        } else {
            ship.acc.x = 0.0;
            ship.acc.y = 0.0;
        }

        {
            let dfc = 0.128 * 0.025;

            let full_s = ship.vel.x * ship.vel.x + ship.vel.y * ship.vel.y;
            let full = libm::sqrtf(full_s);

            if !full.is_zero() {
                let xp = ship.vel.x / full;
                let yp = ship.vel.y / full;

                //actual drag calculation
                let tmp_ad = dfc * full_s;
                ship.acc.x -= xp * tmp_ad;
                ship.acc.y -= yp * tmp_ad;
            }

            //idk what this is irl but it makes it feel better??
            {
                let fony_balony_drag_cof = 20.0;
                if !full.is_zero() {
                    let abs_x = abs(ship.vel.x);

                    if abs_x < 0.5 {
                        ship.vel.x = 0.0;
                    } else {
                        ship.acc.x -= ship.vel.x / full * fony_balony_drag_cof;
                    }

                    let abs_y = abs(ship.vel.y);
                    if abs_y < 0.5 {
                        ship.vel.y = 0.0;
                    } else {
                        ship.acc.y -= ship.vel.y / full * fony_balony_drag_cof;
                    }
                }
            }

            let half_d_time = d_time / 2.0;

            ship.vel.x += ship.acc.x * half_d_time; // OoOOOOoO proper acceleration integration OOOooOooOOo
            ship.vel.y += ship.acc.y * half_d_time;
            ship.pos.x += ship.vel.x * d_time;
            ship.pos.y += ship.vel.y * d_time;
            ship.vel.x += ship.acc.x * half_d_time;
            ship.vel.y += ship.acc.y * half_d_time;
        }

        //interface::println!("{:#?}", ship.acc);

        ship.pos.fit_to_screen(screen);
        ship.invincible -= d_time;
        if ship.invincible < 0.0 {
            ship.invincible = 0.0;
        }

        if !invincible && !ship.god {
            let t = 'thing: {
                for (i, asteroid) in asteroids.iter_mut().enumerate() {
                    let tmpx = asteroid.pos.x - ship.pos.x;
                    let tmpy = asteroid.pos.y - ship.pos.y;
                    let distance = tmpx * tmpx + tmpy * tmpy;
                    let tmp = SHIP_MAX_RADIUS + asteroid.max_rad;
                    if distance < tmp * tmp {
                        break 'thing ShipCollision::Asteroid(i);
                    }
                }

                for (_i, bullet) in bullets.iter_mut().enumerate() {
                    if bullet.can_hurt_player {}
                }
                ShipCollision::None
            };
            if !matches!(t, ShipCollision::None) {
                let mut l_v = d_points[3];
                for p in d_points {
                    let mut particle = Particle::new_line(
                        Vector::new(p.x as f32, p.y as f32),
                        Vector::new(l_v.x as f32, l_v.y as f32),
                    );
                    particle.vel.x += ship.vel.x;
                    particle.vel.y += ship.vel.y;
                    particles.push(particle);
                    l_v.x = p.x;
                    l_v.y = p.y;
                }
            }
            return t;
        }

        ShipCollision::None
    }

    pub fn after_death(&mut self, screen: &mut Screen) {
        self.pos.x = screen.get_width() as f32 / 2.0;
        self.pos.y = screen.get_height() as f32 / 2.0;
        self.vel = Vector::new(0.0, 0.0);
        self.acc = Vector::new(0.0, 0.0);
        self.angle = -core::f32::consts::FRAC_PI_2;
        self.invincible = 2.6;
    }
}

//------------------------------------------------------------------------------------------

struct AlianShip {}

//------------------------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct Asteroid {
    pos: Vector,
    vel: Vector,
    max_rad: f32,
    points: [(i16, i16); 12],
    size: u8,
}

impl Asteroid {
    pub fn new(pos: Vector, size: u8) -> Self {
        let mut points: [(i16, i16); 12] = [(0, 0); 12];

        let mut max_rad = 0.0;

        for (i, point) in points.iter_mut().enumerate() {
            let angle = i as f32 / 12.0 * core::f32::consts::TAU;
            let (sin, cos) = libm::sincosf(angle);
            let mut circ = size as f32 * 13.0;
            circ += rlib::arch::rand_range((circ / -4.0) as i32, (circ / 4.0) as i32) as f32;
            if circ > max_rad {
                max_rad = circ;
            }
            point.0 = (circ * sin) as i16;
            point.1 = (circ * cos) as i16;
        }

        Self {
            pos,
            vel: Vector::random_vel((4 - size) as i32 * 10, (4 - size) as i32 * 50),
            points,
            size,
            max_rad,
        }
    }

    pub fn split(self) -> [Self; 2] {
        let Self { pos, size, .. } = self;
        [Self::new(pos, size - 1), Self::new(pos, size - 1)]
    }
}

//------------------------------------------------------------------------------------------

struct Bullet {
    pos: Vector,
    vel: Vector,
    life: f32,
    can_hurt_player: bool,
}

//------------------------------------------------------------------------------------------

mod util {
    use rlib::io::screen::{ScreeenPoint, Screen};

    #[derive(Debug, Default, Clone, Copy)]
    pub struct Vector {
        pub x: f32,
        pub y: f32,
    }

    impl From<(f32, f32)> for Vector {
        fn from(val: (f32, f32)) -> Self {
            Self { x: val.0, y: val.1 }
        }
    }

    impl From<Vector> for ScreeenPoint {
        fn from(vec: Vector) -> Self {
            Self {
                x: vec.x as i16,
                y: vec.y as i16,
            }
        }
    }

    impl From<ScreeenPoint> for Vector {
        fn from(vec: ScreeenPoint) -> Self {
            Self {
                x: vec.x as f32,
                y: vec.y as f32,
            }
        }
    }

    impl Vector {
        pub fn fit_to_screen(&mut self, screen: &mut Screen) {
            if self.x >= screen.get_width() as f32 {
                self.x -= screen.get_width() as f32;
            }
            if self.x >= screen.get_width() as f32 {
                self.x %= screen.get_width() as f32;
            }

            if self.y >= screen.get_height() as f32 {
                self.y -= screen.get_height() as f32;
            }
            if self.y >= screen.get_height() as f32 {
                self.y %= screen.get_height() as f32;
            }

            if self.x < 0.0 {
                self.x += screen.get_width() as f32;
            }
            if self.x < 0.0 {
                self.x %= screen.get_width() as f32;
            }

            if self.y < 0.0 {
                self.y += screen.get_height() as f32;
            }
            if self.y < 0.0 {
                self.y %= screen.get_height() as f32;
            }
        }

        pub const fn new(x: f32, y: f32) -> Self {
            Self { x, y }
        }

        pub(crate) fn random_vel(min: i32, max: i32) -> Vector {
            let rand = rlib::arch::rand_range(min, max) as f32;
            let angle =
                rlib::arch::rand_range(0, 720) as f32 * core::f32::consts::FRAC_PI_2 / 180.0;
            let (sin, cos) = libm::sincosf(angle);
            let x = sin * rand;
            let y = cos * rand;
            Self { x, y }
        }
    }

    pub fn abs(val: f32) -> f32 {
        if val.is_sign_negative() {
            -val
        } else {
            val
        }
    }

    pub fn calculate_screen_points<const T: usize>(
        points: [Vector; T],
        sin: f32,
        cos: f32,
        off: Vector,
    ) -> [ScreeenPoint; T] {
        let mut d_points: [ScreeenPoint; T] = [ScreeenPoint::default(); T];
        for (point, d_point) in points.iter().zip(d_points.iter_mut()) {
            let tmp = (point.x * cos + point.y * -sin + off.x) as i16;
            d_point.y = (point.x * sin + point.y * cos + off.y) as i16;
            d_point.x = tmp;
        }
        d_points
    }
}

//------------------------------------------------------------------------------------------

struct Particle {
    pos: Vector,
    vel: Vector,
    ls: Option<(Vector, f32, f32)>,
    life: f32,
    color_fn: Option<Box<dyn Fn(&Self) -> [u8; 4]>>,
}

impl Particle {
    pub fn new_point(pos: Vector) -> Self {
        Self {
            pos,
            vel: Vector::random_vel(10, 70),
            ls: None,
            life: rlib::arch::rand_range(10, 25) as f32 / 10.0,
            color_fn: None,
        }
    }

    pub fn new_line(pos1: Vector, pos2: Vector) -> Self {
        let mut tmp = Self::new_point(pos1);
        tmp.ls = Some((pos2, 0.0, rlib::arch::rand_range(-170, 170) as f32 / 180.0));
        tmp.vel.x /= 5.0;
        tmp.vel.y /= 5.0;
        tmp
    }
}
