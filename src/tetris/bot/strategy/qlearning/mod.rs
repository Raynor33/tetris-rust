use tch::{Device, nn, Tensor};
use tch::nn::{Module, Optimizer, OptimizerConfig, Sequential, VarStore};
use crate::tetris::bot::strategy::Strategy;
use crate::tetris::{Action, Tetris};
use crate::tetris::ActionResult::GameOver;
use crate::tetris::bot::analysis::analyse;
use crate::tetris::bot::strategy::random::Random;

const ANALYSIS_ARRAY_SIZE: usize = 4;

// Q-learning constants
const GAME_OVER_PENALTY: f64 = 1.0;
const DISCOUNT_RATE: f64 = 0.99;

// ANN constants
const LEARNING_RATE: f64 = 0.001;
const EPOCH: i64 = 100;
const INPUT_NODES: i64 = 4;
const NET_HIDDEN_NODES: i64 = 6;
const OUTPUT_NODES: i64 = 1;

pub struct QLearning {
    random_strategy: Random,
    training_input_count: usize,
    training_buffer_capacity: usize,
    training_inputs_buffer: Vec<[f64; ANALYSIS_ARRAY_SIZE]>,
    training_outputs_buffer: Vec<[f64; 1]>,
    training_frequency: usize,
    net: Sequential,
    pub optimiser: Optimizer,
}

struct TrainingItem {
    input: Tensor,
    output: Tensor,
}

impl QLearning {
    pub fn new() -> QLearning {
        let vs= nn::VarStore::new(Device::Cpu);
        let vs_root = &vs.root();
        let net = nn::seq()
            .add(nn::linear(
                vs_root / "layer1",
                i64::from(INPUT_NODES),
                NET_HIDDEN_NODES,
                Default::default(),
            ))
            .add_fn(|xs| xs.relu())
            .add(nn::linear(vs_root, NET_HIDDEN_NODES, 1, Default::default()));
        let mut optimiser = nn::Adam::default().build(&vs, LEARNING_RATE).unwrap();
        QLearning {
            random_strategy: Random::new(),
            training_input_count: 0,
            training_buffer_capacity: 10000,
            training_inputs_buffer: vec![],
            training_outputs_buffer: vec![],
            training_frequency: 10,
            optimiser,
            net,
        }
    }

    fn should_experiment(&self) -> bool {
        false
    }

    fn train(&mut self, tetris: &Tetris, best_action_score: f64) {
        let inputs = Self::inputs(tetris);
        let output = [best_action_score];
        if self.training_inputs_buffer.len() >= self.training_buffer_capacity {
            self.training_inputs_buffer[self.training_input_count % self.training_buffer_capacity] = inputs;
            self.training_outputs_buffer[self.training_input_count % self.training_buffer_capacity] = output;
        } else {
            self.training_inputs_buffer.push(inputs);
            self.training_outputs_buffer.push(output);
        }

        if self.training_input_count % self.training_frequency == 0 {
            // Tensor::stack({}, 0);
            let input_tensor = Tensor::of_slice(&self.training_inputs_buffer[0][..]);
            let output_tensor = Tensor::of_slice(&self.training_outputs_buffer[0][..]);
            for _ in 0..EPOCH {
                let loss = self.net
                    .forward(&input_tensor)
                    .cross_entropy_for_logits(&output_tensor);
                self.optimiser.backward_step(&loss);
            }
        }
        self.training_input_count = self.training_input_count + 1;
    }

    fn inputs(tetris: &Tetris) -> [f64; ANALYSIS_ARRAY_SIZE] {
        let analysis = analyse(tetris);
        let inputs = [
            analysis.low_edges as f64 / 200.0,
            analysis.gaps as f64 / 200.0,
            analysis.central_columns_max_height as f64 / 200.0,
            analysis.total_neighbour_diff as f64 / 200.0,
        ];
        inputs
    }
}

impl Strategy for QLearning {
    fn choose_actions(&mut self, tetris: &Tetris) -> Vec<Action> {
        let best_actions = self.best_actions(tetris);
        if best_actions.result == GameOver {
            self.train(tetris, GAME_OVER_PENALTY);
        } else {
            self.train(tetris, best_actions.score * DISCOUNT_RATE);
        }
        if self.should_experiment() {
            self.random_strategy.choose_actions(tetris)
        } else {
            best_actions.actions
        }
    }

    fn score(&self, outcome: &Tetris) -> f64 {
        let output = self.net
            .forward(&Tensor::of_slice(&QLearning::inputs(outcome)));
        0.0
    }
}



// use std::sync::mpsc;
// use std::{process, thread, time};
// use tetris_rust::tetris::gui::Gui;
// use anyhow::Result;
// use tch::{nn, nn::Module, nn::OptimizerConfig, Device};

// const IMAGE_DIM: i64 = 784;
// const HIDDEN_NODES: i64 = 128;
// const LABELS: i64 = 10;
//
// fn net(vs: &nn::Path) -> impl Module {
//     nn::seq()
//         .add(nn::linear(
//             vs / "layer1",
//             IMAGE_DIM,
//             HIDDEN_NODES,
//             Default::default(),
//         ))
//         .add_fn(|xs| xs.relu())
//         .add(nn::linear(vs, HIDDEN_NODES, LABELS, Default::default()))
// }
//
// pub fn run() {
//     let m = tch::vision::mnist::load_dir("data")?;
//     let vs = nn::VarStore::new(Device::Cpu);
//     let net = net(&vs.root());
//     let mut opt = nn::Adam::default().build(&vs, 1e-3)?;
//     for epoch in 1..200 {
//         let loss = net
//             .forward(&m.train_images)
//             .cross_entropy_for_logits(&m.train_labels);
//         opt.backward_step(&loss);
//         let test_accuracy = net
//             .forward(&m.test_images)
//             .accuracy_for_logits(&m.test_labels);
//         println!(
//             "epoch: {:4} train loss: {:8.5} test acc: {:5.2}%",
//             epoch,
//             f64::from(&loss),
//             100. * f64::from(&test_accuracy),
//         );
//     }
// }



