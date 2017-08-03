//! Fight Logic

use Player;
use Enemy;
use Stats;
use Visuals;

/// Stores fight information
pub struct Fight<'a> {
    /// Enemy in fight.
    pub enemy_visuals: &'a Visuals,
    /// Enemy stats in fight.
    pub enemy_stats: Stats,
    /// Player in fight.
    pub player_visuals: &'a Visuals,
    /// Player stats in fight.
    pub player_stats: Stats,
}

impl<'a> Fight<'a> {
    /// Creates a new fight
    pub fn new(player_visuals: &'a Visuals, enemy_visuals: &'a Visuals, player_stats: &Stats, enemy_stats: &Stats,) -> Fight<'a> {
        Fight{
            enemy_visuals: enemy_visuals,
            player_visuals: player_visuals,
            enemy_stats: enemy_stats.clone(),
            player_stats: player_stats.clone(),
        }
    }
}
