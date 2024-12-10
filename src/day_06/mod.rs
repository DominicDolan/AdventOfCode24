mod track_runner;

use crate::day_06::track_runner::TrackRunners;
use crate::utils;
use crate::utils::char_grid::CharGrid;
use crate::utils::grid_cursor::GridCursor;
use crate::utils::ivector2::IVector2;
use std::hash::Hash;

const SAMPLE_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

pub fn main() {
    let input = utils::read_input("day_06");
    // let part_01_answer = part_01(input.as_str());
    // assert_eq!(part_01_answer, 5131);

    let part_02_answer = part_02(input.as_str());
    println!("Day 06 Part 2: {}", part_02_answer);
}

enum GuardStopReason {
    Bounds,
    Obstacle,
    LoopPoint
}

enum TrackStopReason {
    NearbyObstacle,
    Obstacle,
    CrossedTrack,
    Bounds
}

fn part_01(input: &str) -> usize {
    let grid = CharGrid::from(input);

    let guard_coord = grid.find_coord('^').expect("No value found for '^'");

    let mut cursor = GridCursor::new(guard_coord);

    cursor.set_velocity(IVector2::new(0, -1));
    let mut has_finished = false;

    while !has_finished {
         let reason = cursor.inc_until_some(|coord | {
            let c = grid.get(coord);
            return if c.is_some() && c.unwrap() == '#' {
                Some(GuardStopReason::Obstacle)
            } else if c.is_none() {
                Some(GuardStopReason::Bounds)
            } else {
                None
            };
        });
        
        match reason {
            GuardStopReason::Bounds => has_finished = true,
            GuardStopReason::Obstacle => {
                cursor.set_velocity(cursor.current_velocity().turn_right());
            },
            other => {}
        }
    }

    cursor.track_len()
}

fn part_02(input: &str) -> usize {
    let grid = CharGrid::from(input);

    let guard_coord = grid.find_coord('^').expect("No value found for '^'");

    let mut guard = GridCursor::new(guard_coord);
    guard.set_velocity(IVector2::new(0, -1));
    
    let mut track_runners = TrackRunners::new();
    
    let mut has_finished = false;
    let mut loop_point_count = 0;

    fn is_left_hand_track(track_runners: &TrackRunners, coord: IVector2, current_velocity: IVector2) -> bool {
        let left_hand_velocity = current_velocity.turn_left();
        if current_velocity.y == 0 {
            track_runners.get_all_by_x(coord.x).iter().any(|runner| {
                runner.contains_track_with_velocity(coord, left_hand_velocity)
            })
        } else {
            track_runners.get_all_by_y(coord.y).iter().any(|runner| {
                runner.contains_track_with_velocity(coord, left_hand_velocity)
            })
        }
    }
    
    while !has_finished {
        let current_velocity = guard.current_velocity();
        let reason = guard.inc_until_some(|next| {
            let c = grid.get(next);
            return if c.is_some() && c.unwrap() == '#' {
                Some(GuardStopReason::Obstacle)
            } else if is_left_hand_track(&track_runners, next, current_velocity) {
                Some(GuardStopReason::LoopPoint)
            } else if c.is_none() {
                Some(GuardStopReason::Bounds)
            } else {
                None
            };
        });

        match reason {
            GuardStopReason::Bounds => has_finished = true,
            GuardStopReason::Obstacle => {
                let mut track_runner = GridCursor::new(guard.current_position());
                track_runner.set_velocity(guard.current_velocity().reverse());
                
                track_runners = run_track(track_runners, &grid, track_runner);
                
                guard.set_velocity(guard.current_velocity().turn_right());
            },
            GuardStopReason::LoopPoint => {
                loop_point_count += 1;
                guard.inc()
            }
        }
    }
    
    loop_point_count
}

fn run_track(mut track_runners: TrackRunners, grid: &CharGrid, mut track_runner: GridCursor) -> TrackRunners {

    let mut has_finished = false;
    
    while !has_finished {
        let current_velocity = track_runner.current_velocity();
        let reason = track_runner.inc_until_some(|coord | {
            let c = grid.get(coord);
            let nearby_c = grid.get(coord.plus(current_velocity.turn_right()));
            return if c.is_some() && c.unwrap() == '#' {
                Some(TrackStopReason::Obstacle)
            } else if track_runners.contains_track_with_velocity(coord, current_velocity) {
                Some(TrackStopReason::CrossedTrack)
            } else if nearby_c.is_some() && nearby_c.unwrap() == '#' {
                Some(TrackStopReason::NearbyObstacle)
            } else if c.is_none() {
                Some(TrackStopReason::Bounds)
            } else {
                None
            };
        });

        match reason {
            TrackStopReason::Bounds => has_finished = true,
            TrackStopReason::Obstacle => has_finished = true,
            TrackStopReason::CrossedTrack => has_finished = true,
            TrackStopReason::NearbyObstacle => {
                track_runner.inc();
                let mut new_track_runner = GridCursor::new(track_runner.current_position());
                new_track_runner.set_velocity(track_runner.current_velocity().turn_left());
                
                track_runners = run_track(track_runners, &grid, new_track_runner);
            }
        }
    }
    
    track_runners.insert(track_runner);
    track_runners
}

fn print_grid(grid: &CharGrid, cursor: &GridCursor, track_runners: &Vec<GridCursor>) {
    println!(
        "\n\n{}",
        grid.as_transformed_string(|c, coord| {
            let track_runner_contains_coords = track_runners
                .iter()
                .any(|runner| { runner.contains_track(coord) });
            
            return if track_runner_contains_coords {
                '-'
            } else if cursor.has_passed(coord) {
                'X'
            } else {
                c
            };
        })
    );
}

