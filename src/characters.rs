//! Characters

use std::rc::Rc;
use std::option::*;
use opengl_graphics::Texture;
use graphics::Image;

/// Statistics for characters.
#[derive(Debug, Copy, Clone)]
pub struct Stats {
    /// health.
    pub health: i64,
    /// attack.
    pub attack: i64,
    /// defense.
    pub defense: i64,
}

impl Stats {
    /// Creates stats.
    pub fn new() -> Stats {
        Stats{
            health: 100,
            attack: 20,
            defense: 30,
        }
    }
}

/// Visuals for characters.
pub struct Visuals{
    /// Texture.
    texture: Texture,
    ///image.
    image: Image,
}

impl Visuals {
    /// Creates visuals.
    pub fn new(texture: Texture, image: Image) -> Visuals {
        Visuals {
            texture: texture,
            image: image,
        }
    }

    /// Gives texture.
    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    /// Gives image.
    pub fn image(&self) -> &Image {
        &self.image
    }
}

/// Player struct. A player is broken up into two parts, stats and visuals
pub struct Player {
    /// Player visuals.
    visuals: Visuals,

    /// Player stats.
    stats: Stats,
}

impl Player {
    /// Creates new player
    pub fn new(texture: Texture, image: Image) -> Player {
        Player{
            visuals: Visuals::new(texture, image),
            stats: Stats::new(),
        }
    }

    /// Gets visuals
    pub fn visuals(&self) -> &Visuals {
        &self.visuals
    }

    /// Gets stats
    pub fn stats(&self) -> &Stats {
        &self.stats
    }
}

/// Enemy.
pub struct Enemy {
    /// Enemy stats.
    pub stats: Stats,
    /// Enemy visuals.
    pub visuals: Visuals,
}

impl Enemy {
    /// Creates new enemy
    pub fn new(texture: Texture, image: Image) -> Enemy {
        Enemy{
            stats: Stats::new(),
            visuals: Visuals::new(texture, image),
        }
    }

    /// Gets visuals
    pub fn visuals(&self) -> &Visuals {
        &self.visuals
    }

    /// Gets stats
    pub fn stats(&self) -> &Stats {
        &self.stats
    }
}
