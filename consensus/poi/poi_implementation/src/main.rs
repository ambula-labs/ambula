use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// use rand::prelude::*;
use rand::{Rng, SeedableRng, thread_rng};
use rand::rngs::{StdRng, ThreadRng};
use rand_distr::{Normal, Distribution};

//---------------------------------------------------------------------
// Définition of a Node structure
//---------------------------------------------------------------------
struct Node 
{
    name : String,
    ip : String,
    public_key : String,
}
//---------------------------------------------------------------------



//---------------------------------------------------------------------
// Définition of getters
//---------------------------------------------------------------------
impl Node
{
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



//---------------------------------------------------------------------
// Definition of Node methods
//---------------------------------------------------------------------
trait NodeInfo 
{
    fn get_infos(&self);
}

impl NodeInfo for Node 
{
    fn get_infos(&self) {
        println!("Name : \"{}\"", &self.get_name());
        println!("IP : {}", &self.get_ip());
        println!("Public Key : {}\n", &self.get_public_key());
    }
}
//---------------------------------------------------------------------


//---------------------------------------------------------------------
// Implemntation of the algorithm createServices of the paper.
// This function create a pseudo-random subset of nodes named S.
//
// @param seed : seed to create a RNG and gererate a random size for S
// @param _n : set of nodes
//
// @return Vec<&Node> : a subset of _n
//---------------------------------------------------------------------
fn create_services(seed: u64, _n: &Vec<Node>) -> Vec<&Node>
{
    let mut rng: StdRng = StdRng::seed_from_u64(seed);          // RNG
    let mut random_number: u64;                              // number to use for random index of N
    let s_size: usize = 20.min(_n.len()/2);                    // size of the subset S
    let mut _s :Vec<&Node> = Vec::new();                // the subset S

    println!("Size of the set N : {}", _n.len());
    println!("s0 : {}", seed);
    println!("Size of the subset S : {} \n", s_size);

    let mut x: usize = 0;                                      //iterator of the loop
    let mut check_state: i32 = 0;                            //variable to know if the current element of N is already present in S (yes: 1 | no: 0)
    loop {

        //Take a random element of N
        random_number = rng.gen::<u64>() % (_n.len() as u64);
        let node_tmp: &Node = &_n[random_number as usize];

        //Check if this random element is not already present in S
        let mut y: usize = 0;                                  //index to browse S
        loop {
            
            if _s.len() == 0 { break; }

            if node_tmp.get_name() == _s[y].get_name() {
                check_state = 1;
                break;
            } 
            else { y = y+1; }

            if y == _s.len() { break; }
        }

        //If not, it's added in S
        if check_state == 0 { 
            _s.push(node_tmp); 
            x = x+1;
        }

        check_state = 0;

        //Stop the processus when S is completed
        if x == s_size { break; }
    } 

    return _s;
}
//---------------------------------------------------------------------


//---------------------------------------------------------------------
// The function get_tour_length_distribution is a random number generator,
// seeded with s0 that generates a number according to probabilistic
// distribution. This number represent the number of signature required
// to validate and push the current block.
//
// Probabilistic distribution : Normal distribution
//
// @param distribution : normal distribution
// @param seed : seed to create a RNG
// 
// @return f64 : the random length
//---------------------------------------------------------------------
fn get_tour_length_distribution(distribution: &Normal<f64>, seed: u64) -> f64 {
    let mut rng: StdRng = StdRng::seed_from_u64(seed);
    let value: f64 = distribution.sample(&mut rng);
    value
}


//---------------------------------------------------------------------
// The function tour_length is a random number generator, seeded with s0
// that generates a number according to probabilistic distribution. This
// number represent the number of signature required to validate and push
// the current block.
//
// Probabilistic distribution : Normal distribution
//
// @param min_length : minimum length of the tour
// @param difficulty : current difficulty of the network
// @param standard_deviation : chosen standard deviation
// @param seed : seed to create a RNG
//
// @return u64 : the random length
//---------------------------------------------------------------------
fn tour_length(min_length: u64, difficulty: f64, standard_deviation: f64, seed: u64) -> u64 {
    let distribution_result: Result<Normal<f64>, rand_distr::NormalError> = Normal::new(difficulty, standard_deviation);
    let distribution: Normal<f64> = distribution_result.unwrap(); // Extract the value or panic on error
    let value: u64 = get_tour_length_distribution(&distribution, seed).round() as u64;
    let clamped_value: u64 = value.max(min_length);
    clamped_value
}
//---------------------------------------------------------------------


//---------------------------------------------------------------------
// The function concat_u64_as_u128 is a function that concatenate 
// u64s to create a u128.
// 
// @param nums : list of u64s
//
// @return u64 : the concatenation of the numbers
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


//---------------------------------------------------------------------
// The function verify_signature is a function that verifies that
// `signature` is a signature of `dependency` by `u`.
// 
// @param u : the public key of the node that signed
// @param signature : the signature to verify
// @param dependency : the dependency to verify
//
// @return bool : true if the signature is valid, false otherwise
//---------------------------------------------------------------------
fn verify_signature(u: &str, signature: u128, dependency: u128) -> bool
{
    return true; // TODO once we have a signature mecanism setup
}
//---------------------------------------------------------------------


//---------------------------------------------------------------------
// The function hash is a function that hashes a string.
//
// @param value : the value to hash
//
// @return u64 : the hash of the value
//---------------------------------------------------------------------
fn hash(value: String) -> u64 {
    let mut h: DefaultHasher = DefaultHasher::new();
    value.hash(&mut h);
    return h.finish();
}
//---------------------------------------------------------------------


//---------------------------------------------------------------------
// The function check_poi is a function that verifies that the proof
// of interaction is valid.
//
// @param proof : the proof of interaction
// @param u : the public key of the node that signed
// @param dependency : the dependency to verify
// @param message_root : the root of the message
// @param difficulty : the difficulty of the proof of interaction
// @param _n : the set of nodes
//
// @return bool : true if the proof of interaction is valid, false otherwise
//---------------------------------------------------------------------
fn check_poi(proof: &Vec<u64>, u: &str, dependency: u64, message_root: u64, difficulty: f64, _n: &Vec<Node>) -> bool {
    if !verify_signature(u, proof[0] as u128, dependency as u128) {
        return false;
    }
    
    let network_size: u64 = _n.len() as u64;
    let std_deviation_coefficient: f64 = 0.1;

    let s: Vec<&Node> = create_services(proof[0], _n);
    let l: u64 = tour_length(network_size, difficulty, network_size as f64 * std_deviation_coefficient, proof[0]);
    if 2*l + 1 != proof.len() as u64 {
        return false;
    }

    let mut data_to_hash: u128 = concat_u64_as_u128(&[proof[0], message_root]);
    let mut current_hash: u64 = hash(data_to_hash.to_string());

    for i in 0..l as usize {
        let next_hop: usize = (current_hash % (s.len() as u64)) as usize;
        let next_node_key: &str = s[next_hop].get_public_key();
        let to_check: u128 = concat_u64_as_u128(&[current_hash, dependency, message_root]);
        if !verify_signature(next_node_key, proof[2*i+1] as u128, to_check) {
            return false;
        }
        if !verify_signature(u, proof[2*i+2] as u128, proof[2*i+1] as u128) {
            return false;
        }
        data_to_hash = concat_u64_as_u128(&[proof[2*i+2]]);
        current_hash = hash(data_to_hash.to_string())
    }
    return true;
}
//---------------------------------------------------------------------


//---------------------------------------------------------------------
fn sign(_n: &Node, _d: u64) -> u64
{
    //TODO process of signature

    let mut rng: ThreadRng = thread_rng();
    let random_number : u64 = rng.gen_range(1..=1000);

    return random_number;
}
//---------------------------------------------------------------------


//---------------------------------------------------------------------
fn send(_receiver: u64, _h: u64, _d: u64, _m: u64) -> u64
{
    //TODO send a request to _receiver to have a signature

    let mut rng: ThreadRng = thread_rng();
    let random_number: u64 = rng.gen_range(1..=1000);
    
    return random_number
}
//---------------------------------------------------------------------


//---------------------------------------------------------------------
// This function is executed by u0 to generate the PoI
//  
// @param u0 : the node wich wants to push _m
// @param _d : dependency (hash of last block of the blockchain)
// @param _m : the message : the new block to push in the blockchain -> hash of this block
// @param d1 : first parameter of the difficulty of the PoI
// @param d2 : second parameter of the diffculty of the PoI
// @param _n : the set of nodes in the network
//
// @return  : P, the PoI, a list of signatures {s0, s1, s1', .., sk, sk'}
//---------------------------------------------------------------------
fn generate_poi(u0: &Node, _d: u64, _m: u64, difficulty: f64, _n: &Vec<Node>) -> Vec<u64>
{
    let mut _p :Vec<u64> = Vec::new();                                  //the list of signatures                               
    let s0 : u64 = sign(&u0, _d);                                       //the signature of u0 (the node which wants to push _m)
    let mut _s : Vec<&Node>  = create_services(s0, &_n);             

    let network_size: u64 = _n.len() as u64;
    let std_deviation_coefficient: f64 = 0.1;

       //the subset of _n, use to get the differents signatures
    let _l: u64 = tour_length(network_size, difficulty, network_size as f64 * std_deviation_coefficient, s0);
                          //number of signatures required to push _m

    //Print S
    for _x in 0.._s.len() {
        _s[_x].get_infos();
    } 
    println!("{} signatures required to validate and push the current block.", _l);

    _p.push(s0);

    let data_to_hash : u128 = concat_u64_as_u128(&[s0, _m]);
    let mut current_hash : u64 = hash(data_to_hash.to_string());
    
    let mut next_hop : u64;
    let mut sk : u64;
    for _k in 0.._l {

        next_hop = current_hash % (_s.len() as u64);
        sk = send(next_hop, current_hash, _d, _m);
        _p.push(sk);
        sk = sign(&u0, sk);
        _p.push(sk);
        current_hash = hash(sk.to_string());
    }

    return _p;
}
//---------------------------------------------------------------------


//---------------------------------------------------------------------
//MAIN
//---------------------------------------------------------------------
fn main() {

    //Déclaration node n°1
    let node_1 = Node {
        name : String::from("Node n°1"),
        ip : String::from("127.0.0.1"),
        public_key : String::from("abcdefga")
    };

    //Déclaration node n°2
    let node_2 = Node {
        name : String::from("Node n°2"),
        ip : String::from("127.0.0.2"),
        public_key : String::from("abcdefgb")
    };

    //Déclaration node n°3
    let node_3 = Node {
        name : String::from("Node n°3"),
        ip : String::from("127.0.0.3"),
        public_key : String::from("abcdefgc")
    };

    //Déclaration node n°4
    let node_4 = Node {
        name : String::from("Node n°4"),
        ip : String::from("127.0.0.4"),
        public_key : String::from("abcdefgd")
    };

    //Déclaration node n°5
    let node_5 = Node {
        name : String::from("Node n°5"),
        ip : String::from("127.0.0.5"),
        public_key : String::from("abcdefge")
    };

    //Déclaration node n°6
    let node_6 = Node {
        name : String::from("Node n°6"),
        ip : String::from("127.0.0.6"),
        public_key : String::from("abcdefgf")
    };

    //Déclaration node n°7
    let node_7 = Node {
        name : String::from("Node n°7"),
        ip : String::from("127.0.0.7"),
        public_key : String::from("abcdefgg")
    };

    //Déclaration node n°8
    let node_8 = Node {
        name : String::from("Node n°8"),
        ip : String::from("127.0.0.8"),
        public_key : String::from("abcdefgh")
    };

    //Déclaration node n°9
    let node_9 = Node {
        name : String::from("Node n°9"),
        ip : String::from("127.0.0.9"),
        public_key : String::from("abcdefgi")
    };

    //Déclaration node n°10
    let node_10 = Node {
        name : String::from("Node n°10"),
        ip : String::from("127.0.0.10"),
        public_key : String::from("abcdefgj")
    };
    

    //Déclaration of the Node set N
    let mut _n :Vec<Node> = Vec::new();
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

    let last_block_hash : u64 = 54321;
    let block1 : u64 = 999;
    let difficulty: f64 = 20.0;
    let _p : Vec<u64> = generate_poi(&_n[0],last_block_hash, block1, difficulty, &_n);

    //Print the PoI :
    let mut iterator = 0;
    let mut index = 1;
    loop {

        if iterator == _p.len() { break; }

        if iterator == 0 { println!("s0 : {}", _p[iterator]); }

        else {

            if iterator > 1 && iterator%2 == 1 { index += 1; }

            if (iterator % 2) == 1 { println!("s{} : {}", index,_p[iterator]); }
            
            else { println!("s{}' : {}", index, _p[iterator]); }
        }

        iterator = iterator + 1;
    }  

    //Print N
    //for _x in 0.._n.len() {
    //    _n[_x].get_infos();
    //}  
    
}
