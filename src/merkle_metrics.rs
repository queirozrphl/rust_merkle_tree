use rand::Rng;
use std::time::{Duration, Instant};
use crate::merkle_tree;

fn generate_random_numbers(n: usize) -> Vec<u8> {
  let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
  let mut numbers = Vec::with_capacity(n);

  for _ in 0..n {
      numbers.push(rng.gen_range(0..=255));
  }

  numbers
}


fn testing_creation_time() {
  // testando 10000 iterações
  let mut total_duration = Duration::new(0, 0);
  let number_of_measurements = 1000;
  for i in 0..(number_of_measurements + 1) {
    let random_numbers = generate_random_numbers(100000); // Gera um vetor com 10 números aleatórios
    let start = Instant::now();
    let merkle_tree = merkle_tree::MerkleTree::new((&random_numbers).clone());
    let duration = start.elapsed();
    total_duration = total_duration.checked_add(duration).expect("Soma de tempo excedeu capacidade");
    // println!("Interação {} tempo de execução: {:?}", i, duration);
  }

  let average_duration = total_duration / number_of_measurements as u32;
  println!("Duração média: {:?}", average_duration);
}


fn testing_proof_gen_time() {
  // testando 10000 iterações
  let mut total_duration = Duration::new(0, 0);
  let number_of_measurements = 1000;
  let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

  for i in 0..(number_of_measurements + 1) {
    let random_numbers = generate_random_numbers(100000); // Gera um vetor com 10 números aleatórios
    let merkle_tree = merkle_tree::MerkleTree::new((&random_numbers).clone());
    let random_index = rng.gen_range(0..(&random_numbers).len());

    let start = Instant::now();
    let proof: Vec<(Vec<u8>, bool)> = merkle_tree.generate_proof(random_index as u8);
    let duration = start.elapsed();
    total_duration = total_duration.checked_add(duration).expect("Soma de tempo excedeu capacidade");
    // println!("Interação {} tempo de execução: {:?}", i, duration);
  }

  let average_duration = total_duration / number_of_measurements as u32;
  println!("Duração média: {:?}", average_duration);
}

fn testing_verify_proof_time() {
  // testando 10000 iterações
  let mut total_duration = Duration::new(0, 0);
  let number_of_measurements = 1000;
  let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

  for i in 0..(number_of_measurements + 1) {
    let random_numbers = generate_random_numbers(100000); // Gera um vetor com 10 números aleatórios
    let merkle_tree = merkle_tree::MerkleTree::new((&random_numbers).clone());
    let random_index = rng.gen_range(0..(&random_numbers).len());
    let data_value = random_numbers[random_index];
    let proof: Vec<(Vec<u8>, bool)> = merkle_tree.generate_proof(random_index as u8);
    let root_hash = merkle_tree.nodes[0].clone();

    let start = Instant::now();
    let is_valid = merkle_tree.verify_proof(proof, data_value, root_hash);
    let duration = start.elapsed();
    total_duration = total_duration.checked_add(duration).expect("Soma de tempo excedeu capacidade");
    // println!("Interação {} tempo de execução: {:?}", i, duration);
  }

  let average_duration = total_duration / number_of_measurements as u32;
  println!("Duração média: {:?}", average_duration);
}


pub fn run_metrics() {
  // testing_creation_time();
  // testing_proof_gen_time();
  testing_verify_proof_time();
}
