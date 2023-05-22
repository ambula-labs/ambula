use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use rand::{Rng, SeedableRng, thread_rng};
use rand::rngs::{StdRng, ThreadRng};
use rand_distr::{Normal, Distribution};

//---------------------------------------------------------------------
// Definition of a Node structure
//---------------------------------------------------------------------
struct Node {
    name: String,
    ip: String,
    public_key: String,
}

//---------------------------------------------------------------------
// Definition of getters
//---------------------------------------------------------------------
impl Node {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_ip(&self) -> &str {
        &self.ip
    }
    pub fn get_public_key(&self) -> &str {
        &self.public_key
    }
}

//---------------------------------------------------------------------
// Definition of Node methods
//---------------------------------------------------------------------
trait NodeInfo {
    fn get_infos(&self);
}

impl NodeInfo for Node {
    fn get_infos(&self) {
        println!("Name: \"{}\"", &self.get_name());
        println!("IP: {}", &self.get_ip());
        println!("Public Key: {}\n", &self.get_public_key());
    }
}
//---------------------------------------------------------------------


//---------------------------------------------------------------------
// Implementation of the algorithm createServices of the paper.
// This function creates a pseudo-random subset of nodes named S.
//
// @param seed: seed to create an RNG and generate a random size for S
// @param network_nodes: set of nodes
//
// @return Vec<&Node>: a subset of _n
//---------------------------------------------------------------------
fn create_services(seed: u64, network_nodes: &Vec<Node>) -> Vec<&Node> {
    let mut rng: StdRng = initialize_rng(seed);
    let network_size = network_nodes.len() as u64;
    let subset_size: u64 = 20.min(network_size / 2);
    let mut services: Vec<&Node> = Vec::new();

    println!("Size of the set N: {}", network_size);
    println!("s0: {}", seed);
    println!("Size of the subset S: {}\n", subset_size);

    let mut x: u64 = 0;
    let mut check_state: i32 = 0;
    let mut random_number: u64;
    
    loop {
        random_number = rng.gen::<u64>() % network_size;
        let node_tmp: &Node = &network_nodes[random_number as usize];
        let mut y: usize = 0;
        loop {
            if services.len() == 0 {
                break;
            }
            if node_tmp.get_name() == services[y].get_name() {
                check_state = 1;
                break;
            } else {
                y += 1;
            }
            if y == services.len() {
                break;
            }
        }
        if check_state == 0 {
            services.push(node_tmp);
            x += 1;
        }
        check_state = 0;
        if x == subset_size {
            break;
        }
    }
    services
}


//---------------------------------------------------------------------
// Initializases the RNG with the provided seed.
// 
// @param seed: seed to create the RNG
// 
// @return StdRng: the initialized RNG
//---------------------------------------------------------------------
fn initialize_rng(seed: u64) -> StdRng {
    StdRng::seed_from_u64(seed)
}


//---------------------------------------------------------------------
// The function get_tour_length_distribution is a random number generator,
// seeded with s0 that generates a number according to a probabilistic
// distribution. This number represents the number of signatures required
// to validate and push the current block.
//
// Probabilistic distribution: Normal distribution
//
// @param distribution: normal distribution
// @param seed: seed to create an RNG
//
// @return f64: the random length
//---------------------------------------------------------------------
fn get_tour_length_distribution(distribution: &Normal<f64>, seed: u64) -> f64 {
    let mut rng: StdRng = initialize_rng(seed);
    let value: f64 = distribution.sample(&mut rng);
    value
}


//---------------------------------------------------------------------
// The function tour_length is a random number generator, seeded with s0,
// that generates a number according to a probabilistic distribution. This
// number represents the number of signatures required to validate and push
// the current block.
//
// Probabilistic distribution: Normal distribution
//
// @param min_length: minimum length of the tour
// @param difficulty: current difficulty of the network
// @param standard_deviation: chosen standard deviation
// @param seed: seed to create an RNG
//
// @return u64: the random length
//---------------------------------------------------------------------
fn tour_length(min_length: u64, difficulty: f64, standard_deviation: f64, seed: u64) -> u64 {
    let distribution_result: Result<Normal<f64>, rand_distr::NormalError> = Normal::new(difficulty, standard_deviation);
    let distribution: Normal<f64> = distribution_result.unwrap();
    let value: u64 = get_tour_length_distribution(&distribution, seed).round() as u64;
    let clamped_value: u64 = value.max(min_length);
    clamped_value
}


//---------------------------------------------------------------------
// The function concat_u64_as_u128 is a function that concatenates
// u64s to create a u128.
//
// @param nums: list of u64s
//
// @return u64: the concatenation of the numbers
//---------------------------------------------------------------------
fn concat_u64_as_u128(nums: &[u64]) -> u128 {
    let mut result = String::new();
    for (i, &num) in nums.iter().enumerate() {
        if i > 0 {
            result.push('0');
        }
        result.push_str(&num.to_string());
    }
    result.parse::<u128>().expect("Overflow occurred")
}

//---------------------------------------------------------------------
// The function verify_signature is a function that verifies that
// `signature` is a signature of `dependency` by `u`.
//
// @param u: the public key of the node that signed
// @param signature: the signature to verify
// @param dependency: the dependency to verify
//
// @return bool: true if the signature is valid, false otherwise
//---------------------------------------------------------------------
fn verify_signature(_u: &str, _signature: u128, _dependency: u128) -> bool {
    // TODO once we have a signature mechanism set up
    true
}


//---------------------------------------------------------------------
// The function hash is a function that hashes a string.
//
// @param value: the value to hash
//
// @return u64: the hash of the value
//---------------------------------------------------------------------
fn hash(value: String) -> u64 {
    let mut h: DefaultHasher = DefaultHasher::new();
    value.hash(&mut h);
    h.finish()
}


//---------------------------------------------------------------------
// The function check_poi is a function that verifies that the proof
// of interaction is valid.
//
// @param proof: the proof of interaction
// @param u: the public key of the node that signed
// @param dependency: the dependency to verify
// @param message_root: the root of the message
// @param difficulty: the difficulty of the proof of interaction
// @param network_nodes: the set of nodes
//
// @return bool: true if the proof of interaction is valid, false otherwise
//---------------------------------------------------------------------
fn check_poi(proof: &Vec<u64>, signer_key: &str, dependency: u64, message_root: u64, difficulty: f64, network_nodes: &Vec<Node>) -> bool {
    if !verify_signature(signer_key, proof[0] as u128, dependency as u128) {
        return false;
    }
    let network_size: u64 = network_nodes.len() as u64;
    let std_deviation_coefficient: f64 = 0.1;
    let services: Vec<&Node> = create_services(proof[0], network_nodes);
    let length: u64 = tour_length(network_size, difficulty, network_size as f64 * std_deviation_coefficient, proof[0]);

    if 2 * length + 1 != proof.len() as u64 {
        return false;
    }

    let mut data_to_hash: u128 = concat_u64_as_u128(&[proof[0], message_root]);
    let mut current_hash: u64 = hash(data_to_hash.to_string());
    for i in 0..length as usize {
        let next_hop: usize = (current_hash % (services.len() as u64)) as usize;
        let next_node_key: &str = services[next_hop].get_public_key();
        let to_check: u128 = concat_u64_as_u128(&[current_hash, dependency, message_root]);
        if !verify_signature(next_node_key, proof[2 * i + 1] as u128, to_check) {
            return false;
        }
        if !verify_signature(signer_key, proof[2 * i + 2] as u128, proof[2 * i + 1] as u128) {
            return false;
        }
        data_to_hash = concat_u64_as_u128(&[proof[2 * i + 2]]);
        current_hash = hash(data_to_hash.to_string())
    }
    true
}


//---------------------------------------------------------------------
fn sign(_n: &Node, _d: u64) -> u64 {
    // TODO process of signature
    let mut rng: ThreadRng = thread_rng();
    let random_number: u64 = rng.gen_range(1..=1000);
    random_number
}


//---------------------------------------------------------------------
fn send(_receiver: u64, _h: u64, _d: u64, _m: u64) -> u64 {
    // TODO send a request to _receiver to have a signature
    let mut rng: ThreadRng = thread_rng();
    let random_number: u64 = rng.gen_range(1..=1000);
    random_number
}


//---------------------------------------------------------------------
// This function is executed by u0 to generate the PoI
//
// @param u0: the node which wants to push _m
// @param last_block_hash: dependency (hash of the last block of the blockchain)
// @param new_block_hash: the message: the new block to push in the blockchain -> hash of this block
// @param difficulty: first parameter of the difficulty of the PoI
// @param network_nodes: the set of nodes in the network
//
// @return: P, the PoI, a list of signatures {s0, s1, s1', .., sk, sk'}
//---------------------------------------------------------------------
fn generate_poi(u0: &Node, last_block_hash: u64, new_block_hash: u64, difficulty: f64, network_nodes: &Vec<Node>) -> Vec<u64> {
    let mut proofs: Vec<u64> = Vec::new();
    let s0: u64 = sign(&u0, last_block_hash);
    let mut services: Vec<&Node> = create_services(s0, &network_nodes);
    let network_size: u64 = network_nodes.len() as u64;
    let std_deviation_coefficient: f64 = 0.1;
    let length: u64 = tour_length(network_size, difficulty, network_size as f64 * std_deviation_coefficient, s0);
    for node in 0..services.len() {
        services[node].get_infos();
    }
    println!("{} signatures required to validate and push the current block.", length);
    proofs.push(s0);
    let data_to_hash: u128 = concat_u64_as_u128(&[s0, new_block_hash]);

    let mut sk: u64;
    let mut next_hop: u64;
    let mut current_hash: u64 = hash(data_to_hash.to_string());
    for _k in 0..length {
        next_hop = current_hash % (services.len() as u64);
        sk = send(next_hop, current_hash, last_block_hash, new_block_hash);
        proofs.push(sk);
        sk = sign(&u0, sk);
        proofs.push(sk);
        current_hash = hash(sk.to_string());
    }
    proofs
}


//---------------------------------------------------------------------
// MAIN
//---------------------------------------------------------------------
fn main() {
    // Declaration node n°1
    let node_1 = Node {
        name: String::from("Node n°1"),
        ip: String::from("127.0.0.1"),
        public_key: String::from("abcdefga"),
    };
    // Declaration node n°2
    let node_2 = Node {
        name: String::from("Node n°2"),
        ip: String::from("127.0.0.2"),
        public_key: String::from("abcdefgb"),
    };
    // Declaration node n°3
    let node_3 = Node {
        name: String::from("Node n°3"),
        ip: String::from("127.0.0.3"),
        public_key: String::from("abcdefgc"),
    };
    // Declaration node n°4
    let node_4 = Node {
        name: String::from("Node n°4"),
        ip: String::from("127.0.0.4"),
        public_key: String::from("abcdefgd"),
    };
    // Declaration node n°5
    let node_5 = Node {
        name: String::from("Node n°5"),
        ip: String::from("127.0.0.5"),
        public_key: String::from("abcdefge"),
    };
    // Declaration node n°6
    let node_6 = Node {
        name: String::from("Node n°6"),
        ip: String::from("127.0.0.6"),
        public_key: String::from("abcdefgf"),
    };
    // Declaration node n°7
    let node_7 = Node {
        name: String::from("Node n°7"),
        ip: String::from("127.0.0.7"),
        public_key: String::from("abcdefgg"),
    };
    // Declaration node n°8
    let node_8 = Node {
        name: String::from("Node n°8"),
        ip: String::from("127.0.0.8"),
        public_key: String::from("abcdefgh"),
    };
    // Declaration node n°9
    let node_9 = Node {
        name: String::from("Node n°9"),
        ip: String::from("127.0.0.9"),
        public_key: String::from("abcdefgi"),
    };
    // Declaration node n°10
    let node_10 = Node {
        name: String::from("Node n°10"),
        ip: String::from("127.0.0.10"),
        public_key: String::from("abcdefgj"),
    };
    // Declaration of the Node set N
    let mut _n: Vec<Node> = Vec::new();
    _n.push(node_1);
    _n.push(node_2);
    _n.push(node_3);
    _n.push(node_4);
    _n.push(node_5);
    _n.push(node_6);
    _n.push(node_7);
    _n.push(node_8);
    _n.push(node_9);
    _n.push(node_10);
    let last_block_hash: u64 = 54321;
    let block1: u64 = 999;
    let difficulty: f64 = 20.0;
    let _p: Vec<u64> = generate_poi(&_n[0], last_block_hash, block1, difficulty, &_n);
    let mut iterator = 0;
    let mut index = 1;
    loop {
        if iterator == _p.len() {
            break;
        }
        if iterator == 0 {
            println!("s0: {}", _p[iterator]);
        } else {
            if iterator > 1 && iterator % 2 == 1 {
                index += 1;
            }
            if iterator % 2 == 1 {
                println!("s{}: {}", index, _p[iterator]);
            } else {
                println!("s{}': {}", index, _p[iterator]);
            }
        }
        iterator = iterator + 1;
    }
}
