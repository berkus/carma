//
// Part of Roadkill Project.
//
// Copyright 2010, 2017, Stanislav Karchebnyy <berkus@madfire.net>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
use num::Num;

#[derive(Clone)]
pub enum LoopType {
    None,
    ForwardLoop,
    PingPongLoop,
}

#[derive(Clone)]
pub enum Direction {
    Forward,
    Backward,
}

#[derive(Builder)]
pub struct AnimatedParameter<T: Num + Ord> {
    value: T,
    // if min == max -> no constraint
    min: T,
    max: T,
    speed: T,
    loop_type: LoopType,
    direction: Direction,
}

impl<T: Num + Ord> AnimatedParameter<T> {
    /* For elapsed time in milliseconds and speed,
    calculate value change, set self.value to new result and return it. */
    pub fn animate(&mut self, elapsedTimeMs: T) -> T
    {
        // self.value = self.value + (self.speed / 1000) * elapsedTimeMs * match self.direction {
        //     Direction::Forward => 1,
        //     Direction::Backward => -1,
        // };

        if self.min != self.max {
            if self.value > self.max {
                match self.loop_type {
                    PingPongLoop => {
                        self.value = self.max;
                        self.direction = match self.direction {
                            Direction::Forward => Direction::Backward,
                            Direction::Backward => Direction::Forward,
                        };
                    }
                    ForwardLoop => self.value = self.min,
                    LoopType::None => self.value = self.max,
                }
            }
            if self.value < self.min {
                match self.loop_type {
                    PingPongLoop => {
                        self.value = self.min;
                        self.direction = match self.direction {
                            Direction::Forward => Direction::Backward,
                            Direction::Backward => Direction::Forward,
                        };
                    }
                    ForwardLoop => self.value = self.max,
                    LoopType::None => self.value = self.min,
                }
            }
        }

        self.value
    }

    pub fn get_value(&self) -> T {
        self.value
    }
}
