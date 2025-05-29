use crate::move_annotation::MoveAnnotation;
use crate::engine_output::EngineOutput;
use crate::engine_eval::{EngineEval, EvalType};

use std::f32::consts;

const EVAL_MULTIPLIER: f32 = -0.00368208;

pub fn get_white_win_percentages(engine_outputs: &[EngineOutput]) -> Vec<f32> {
    engine_outputs.iter().map(|engine_output| {
        eval_to_win_percentage(engine_output.eval())
    }).collect()
}

pub fn get_move_annotations(white_win_percentages: &[f32]) -> Vec<MoveAnnotation> {
    let mut annotations = Vec::with_capacity(white_win_percentages.len());
    for i in 1..white_win_percentages.len() {
        let is_white = i % 2 == 0;
        let (last_percentage, new_percentage) = percentage_pair(white_win_percentages, i, is_white);

        let loss = new_percentage - last_percentage;
        annotations.push(MoveAnnotation::from_parameters(loss, new_percentage));
    }
    annotations
}

pub fn get_accuracy_scores(white_win_percentages: &[f32]) -> (u32, u32) {
    let mut white_accuracy_score = 0.0;
    let mut black_accuracy_score = 0.0;

    for i in 1..white_win_percentages.len() {
        let is_white = i % 2 == 0;
        let (last_percentage, new_percentage) = percentage_pair(white_win_percentages, i, is_white);

        let move_accuracy = 103.1668 * consts::E.powf(
            -0.04354 * (last_percentage - new_percentage)
        ) - 3.1669;
        let move_accuracy = move_accuracy.clamp(0.0, 100.0);
        if is_white {
            white_accuracy_score += move_accuracy;
        } else {
            black_accuracy_score += move_accuracy;
        }
    }

    let total_moves = white_win_percentages.len() - 1;
    let (num_white_moves, num_black_moves) = if total_moves % 2 == 0 {
        (total_moves / 2, total_moves / 2)
    } else {
        (total_moves / 2 + 1, total_moves / 2)
    };
    println!("num_white_moves, num_black_moves: {num_white_moves}, {num_black_moves}");

    white_accuracy_score /= num_white_moves as f32;
    black_accuracy_score /= num_black_moves as f32;

    let white_accuracy_score = white_accuracy_score.round() as u32;
    let black_accuracy_score = black_accuracy_score.round() as u32;

    (white_accuracy_score, black_accuracy_score)
}

fn eval_to_win_percentage(eval: EngineEval) -> f32 {
    match eval.eval_type() {
        EvalType::Mate => if eval.value() > 0 {100.0} else {0.0},
        EvalType::Centipawn => {
            let eval_value = eval.value() as f32;
            let win_chance = 2.0 / (
                1.0 + consts::E.powf(eval_value * EVAL_MULTIPLIER)
            ) - 1.0;
            let win_chance = win_chance.clamp(-1.0, 1.0);

            50.0 + (50.0 * win_chance)
        }
    }
}

fn percentage_pair(white_win_percentages: &[f32], i: usize, is_white: bool) -> (f32, f32) {
    let last_percentage = if is_white {
        white_win_percentages[i - 1]
    } else {
        100.0 - white_win_percentages[i - 1]
    };
    let new_percentage = if is_white {
        white_win_percentages[i]
    } else {
        100.0 - white_win_percentages[i]
    };
    (last_percentage, new_percentage)
}