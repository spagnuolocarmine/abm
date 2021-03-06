use std::hash::Hash;

use crate::engine::location::Int2D;
use crate::utils::dbdashmap::DBDashMap;

/// A crude implementation of a grid that wraps a HashMap, with agents as keys and their locations as values.
pub struct Grid2D<A: Eq + Hash + Clone + Copy> {
    pub locs: DBDashMap<A, Int2D>,
    pub locs_inversed: DBDashMap<Int2D, A>,
    pub width: i64,
    pub height: i64,
}

impl<A: Eq + Hash + Clone + Copy> Grid2D<A> {
    /// Initializes a Grid2D with a specied capacity of width * height.
    pub fn new(width: i64, height: i64) -> Grid2D<A> {
        Grid2D {
            locs: DBDashMap::with_capacity((width * height) as usize),
            locs_inversed: DBDashMap::with_capacity((width * height) as usize),
            width,
            height,
        }
    }

    /// Inserts the agent in the grid, with the agent itself as key and its position as value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_ab::engine::agent::Agent;
    /// use rust_ab::engine::field::object_grid_2d::Grid2D;
    /// use rust_ab::engine::location::Int2D;
    /// use rust_ab::engine::state::State;
    ///
    /// struct S {};
    /// impl State for S{}
    ///
    /// #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    /// struct A {};
    /// impl Agent for A {type SimState = S;
    ///
    /// fn step(&mut self, state: &S) {
    ///         println!("Stepping!");
    ///     }
    /// }
    /// let mut grid = Grid2D::new(10,10);
    /// let mut agent = A{};
    /// let loc = Int2D{x: 2, y: 2};
    /// grid.set_object_location(agent, &loc);
    /// grid.update();
    /// assert!(grid.get_object_location(agent) == Some(&loc));
    /// ```
    pub fn set_object_location(&self, agent: A, new_pos: &Int2D) {
        self.locs.insert(agent, *new_pos);
        self.locs_inversed.insert(*new_pos, agent);
    }

    /// Fetches the position of the agent in the grid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_ab::engine::agent::Agent;
    /// use rust_ab::engine::field::object_grid_2d::Grid2D;
    /// use rust_ab::engine::location::Int2D;
    /// use rust_ab::engine::state::State;
    ///
    /// struct S {};
    /// impl State for S{}
    ///
    /// #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    /// struct A {};
    /// impl Agent for A {type SimState = S;
    ///
    /// fn step(&mut self, state: &S) {
    ///         println!("Stepping!");
    ///     }
    /// }
    /// let mut grid = Grid2D::new(10,10);
    /// let mut agent = A{};
    /// let loc = Int2D{x: 2, y: 2};
    /// grid.set_object_location(agent, &loc);
    /// grid.update();
    /// assert!(grid.get_object_location(agent) == Some(&loc));
    /// ```
    pub fn get_object_location(&self, agent: A) -> Option<&Int2D> {
        self.locs.get(&agent)
    }

    pub fn get_object_at_location(&self, pos: &Int2D) -> Option<&A> {
        self.locs_inversed.get(pos)
    }

    pub fn update(&mut self) {
        self.locs.update();
        self.locs_inversed.update();
    }
}
