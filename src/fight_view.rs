//! Fight view

use graphics::types::Color;
use graphics::{Context, Graphics};

use FightController;

/// Stores fight view settings.
pub struct FightViewSettings {
    /// Background color.
    pub background_color: Color,
    /// Position
    pub position: [f64; 2],
    /// Size X
    pub size_x: f64,
    /// Size y
    pub size_y: f64,
}

impl FightViewSettings {
    /// Creates new fight view settings.
    pub fn new() -> FightViewSettings {
        FightViewSettings {
            background_color: [0.8, 0.8, 1.0, 1.0],
            position: [10.0;2],
            size_x: 1280.0,
            size_y: 720.0,
        }
    }
}

/// Stores visual information about a fight.
pub struct FightView {
    /// Stores fight view settings.
    pub settings: FightViewSettings,
}

impl FightView {
    /// Creates a new fight view.
    pub fn new(settings: FightViewSettings) -> FightView {
        FightView {
            settings: settings,
        }
    }

    /// Draw fight.
    pub fn draw<G: Graphics>(&self, controller: &FightController, c: &Context, g: &mut G){
        use graphics::{Line, Rectangle};

        let ref settings = self.settings;
        let board_rect = [
            0.0, 0.0, settings.size_x, settings.size_y,
        ];

        Rectangle::new(settings.background_color).draw(board_rect, &c.draw_state, c.transform, g);

    }
}
