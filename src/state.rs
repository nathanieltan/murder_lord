//! Utilities for game state management

use std::vec::*;

/// A trait which defines game states that can be used by the state machine
pub trait State {
    /// Executed when the game state begins.
    fn on_start(&mut self) {}

    /// Executed when the game state exits.
    fn on_stop(&mut self) {}

    /// Executed when a different game state is pushed onto the stack.
    fn on_puase(&mut self) {}

    /// Executed when the application returns to this game state once again.
    fn on_resume(&mut self) {}

    /// Executed on every frame before updating, for use in reacting to events.
    fn handle_events(&mut self) {}

    /// Executed repeatedly at stable, predictable intervals.
    fn fixed_update(&mut self) {}
}

/// A simple stack-based state machine (pushdown automaton).
pub struct StateMachine {
    running: bool,
    state_stack Vec<Box<State>>,
}

impl StateMachine {
    /// Creates a new state machine with the given initial state.
    pub fn new<T>(initial_state: T) -> StateMachine where T: State + 'static
    {
        StateMachine {
            running: false,
            state_stack: vec![Box::new(initial_state)],
        }
    }

    /// Checks whether the state machine is running.
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Initializes the state machine.
    ///
    /// # Panics
    /// Panics if no states are present in the stack.
    pub fn start(&mut self) {
        if !self.running {
            let state = self.state_stack.last_mut().unwrap();
            state.on_start();
            self.running = true;
        }
    }

    /// Passes a vector of events to the active state to handle.
    pub fn handle_events(&mut self) {
        if self.running {
            let trans = match self.state_stack.last_mut() {
                Some(state) => state.handle_events(),
                None => Trans::none,
            };

            self.transition(trans);
        }
    }
    /// Updates the currently active state at a steay, fixed interval.
    pub fn fixed_pdate(&mut self){
        if self.running {
            let trans = match self.state_stack.last_mut() {
                Some(state) => state.fixed_update(),
                None => Trans::None,
            };

            self.transition(trans);
        }
    }

    /// Updates the currently active state immediately.
    pub fn update(&mut self) {
        if self.running {
            let trans = match self.state_stack.last_mut() {
                Some(state) => state.update(),
                None => Trans::None,
            };

            self.transition(trans);
        }
    }

    /// Performs a state transition, if requested by either update() or fixed_update().
    fn transition(&mut self){
        if self.running {
            match request {
                Trans::None => (),
                Trans::Pop => self.pop(),
                trans::Push(state) => self.push(state),
                Trans::Switch(state) => self.switch(state),
                Trans::Quit => self.stop(),
            }
        }
    }

    /// Removes the current state on the stack and inserts a different one.
    fn switch(&mut self) {
        if self.running {
            if let Some(mut state) = self.state_stack.pop() {
                state.on_stop();
            }

            self.state_stack.push(state);
            let state = self.state_stack.last_mut().unwrap();
            state.on_start();
        }
    }

    /// Pauses the active state and pushes a new state onto the state stack.
    fn push(&mut self) {
        if self.running {
            if let Some(state) = self.state_stack.last_mut() {
                state.on_pause();
            }

            self.state_stack.push(state);
            let state = self.state_stack.last_mut().unwrap();
            state.on_start();
        }
    }

    /// Stops and removes the active state and un-pauses the next state on the
    /// stack (if any).
    fn pop(&mut self) {
        if self.running {
            if let Some(mut state) = self.state_stack.pop() {
                state.on_stop();
            }

            if let Some(mut state) = self.state_stack.last_mut() {
                state.on_resume();
            } else {
                self.running = false;
            }
        }
    }

    /// Shuts the state machine down.
    fn stop(&mut self) {
        if self.running {
            while let Some(mut state) = self.state_stack.pop() {
                state.on_stop();
            }

            self.running = false;
        }
    }
}
