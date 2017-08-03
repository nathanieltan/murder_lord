//! Fight controller.

use piston::input::GenericEvent;

use Fight;
use FightView;

///Handles events for fights.
pub struct FightController<'a> {
    /// Stores the fight state.
    pub fight: Fight<'a>,
    cursor_pos: [f64;2],
}

impl<'a> FightController<'a> {
    /// Creats a new fight controller.
    pub fn new(fight: Fight) -> FightController {
        FightController{
            fight: fight,
            cursor_pos: [0.0; 2],
        }
    }

    /// Handles events.
    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], e:&E) {
        if let Some(pos) = e.mouse_cursor_args() {

        }
    }

    /// Fight Loop
    pub fn fight_loop(&mut self) {
        // state0: player turn begin
        // state1: player action in process
        // state2: enemy turn begin
        // state3: enemy action begin
        let mut state = 0;

        while self.fight.player_stats.health > 0 && self.fight.enemy_stats.health > 0 {
            if state == 0 {

            }
            else if state == 1 {

            }
            else if state == 2 {

            }
            else if state == 3 {

            }
        }
    }
}
