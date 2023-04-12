use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;


//---------------------------------------------------------------------
//Définition of a Node structure
//---------------------------------------------------------------------
struct Node 
{
    name : String,
    ip : String,
}
//---------------------------------------------------------------------



//---------------------------------------------------------------------
//Définition of getters
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
//Definition of Node methods
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
// This methods create a pseudo-random subset of nodes named S.
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

    //Print of N
    //for x in 0.._n.len() {
    //    _n[x].get_infos();
    //}  

    let s0: u64 = 1234560;                               // signature (seed)
    let mut _s :Vec<&Node> = create_services(s0, &_n);   // subset of N named S

    //Print of S
    for x in 0.._s.len() {
        _s[x].get_infos();
    } 
    
}
