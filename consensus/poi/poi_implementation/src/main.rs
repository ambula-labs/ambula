use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use rand_distr::{Distribution, Uniform};


//---------------------------------------------------------------------
// Définition of a Node structure
//---------------------------------------------------------------------
struct Node 
{
    name : String,
    ip : String,
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
        println!("IP : {}\n", &self.get_ip());

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
    let mut rng = StdRng::seed_from_u64(seed);          // RNG
    let mut random_number;                              // number to use for random index of N
    let s_size = 20.min(_n.len()/2);                    // size of the subset S
    let mut _s :Vec<&Node> = Vec::new();                // the subset S

    println!("Size of the set N : {}", _n.len());
    println!("s0 : {}", seed);
    println!("Size of the subset S : {} \n", s_size);

    let mut x = 0;                                      //iterator of the loop
    let mut check_state = 0;                            //variable to know if the current element of N is already present in S (yes: 1 | no: 0)
    loop {

        //Take a random element of N
        random_number = rng.gen::<u64>() % (_n.len() as u64);
        let node_tmp = &_n[random_number as usize];

        //Check if this random element is not already present in S
        let mut y = 0;                                  //index to browse S
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



/*
//---------------------------------------------------------------------
// The function tour_length is a random number generator, seeded with s0
// that generates a number according to probabilistic distribution. This
// number represent the number of signature required to validate and push
// the current block.
//
// Probabilistic distribution : Normal distribution (not use)
//
// @param d1 : first parameter of normal distribution (mean)
// @param d2 : second parameter of normal distribution (std_dev)
// @param seed : seed to create a RNG 
//
// @return u64 : the random length
//---------------------------------------------------------------------
fn tour_length(d1: u64, d2: u64, seed: u64) -> u64
{
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    let mean = d1 as f64;
    let std_dev = d2 as f64;
    let normal = Normal::new(mean, std_dev).unwrap();   
    let value: f64 = normal.sample(&mut rng);

    return value as u64;
}
//---------------------------------------------------------------------
*/



//---------------------------------------------------------------------
// The function tour_length is a random number generator, seeded with s0
// that generates a number according to probabilistic distribution. This
// number represent the number of signature required to validate and push
// the current block.
//
// Probabilistic distribution : Uniform distribution
//
// @param d1 : first parameter of uniform distribution (lower range)
// @param d2 : second parameter of uniform distribution (upper range)
// @param seed : seed to create a RNG 
//
// @return u64 : the random length
//---------------------------------------------------------------------
fn tour_length(d1: u64, d2: u64, seed: u64) -> u64
{
    let mut rng = StdRng::seed_from_u64(seed);
    let range = Uniform::new_inclusive(d1, d2);
    let value: u64 = range.sample(&mut rng);

    return value;
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
    };

    //Déclaration node n°2
    let node_2 = Node {
        name : String::from("Node n°2"),
        ip : String::from("127.0.0.2"),
    };

    //Déclaration node n°3
    let node_3 = Node {
        name : String::from("Node n°3"),
        ip : String::from("127.0.0.3"),
    };

    //Déclaration node n°4
    let node_4 = Node {
        name : String::from("Node n°4"),
        ip : String::from("127.0.0.4"),
    };

    //Déclaration node n°5
    let node_5 = Node {
        name : String::from("Node n°5"),
        ip : String::from("127.0.0.5"),
    };

    //Déclaration node n°6
    let node_6 = Node {
        name : String::from("Node n°6"),
        ip : String::from("127.0.0.6"),
    };

    //Déclaration node n°7
    let node_7 = Node {
        name : String::from("Node n°7"),
        ip : String::from("127.0.0.7"),
    };

    //Déclaration node n°8
    let node_8 = Node {
        name : String::from("Node n°8"),
        ip : String::from("127.0.0.8"),
    };

    //Déclaration node n°9
    let node_9 = Node {
        name : String::from("Node n°9"),
        ip : String::from("127.0.0.9"),
    };

    //Déclaration node n°10
    let node_10 = Node {
        name : String::from("Node n°10"),
        ip : String::from("127.0.0.10"),
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

    //Print N
    //for x in 0.._n.len() {
    //    _n[x].get_infos();
    //}  

    let s0: u64 = 1234560;                                   // signature (seed)
    let mut _s : Vec<&Node> = create_services(s0, &_n);      // subset of N named S

    //Print S
    for _x in 0.._s.len() {
        _s[_x].get_infos();
    } 

    let length_tour = tour_length(1, 20, s0);                //number of signatures required
    println!("{} signatures required to validate and push the current block.", length_tour);

    
}
