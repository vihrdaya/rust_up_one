use rand::Rng;


#[derive(Debug)]
struct Fighter {
    hp: i64,
    attack: i64,
    defense: i64,
}


fn main() {
    let mut rng = rand::thread_rng();

    let mut fighter1 = Fighter {
        hp: rng.gen_range(5..10),
        attack: rng.gen_range(2..4),
        defense: rng.gen_range(0..1),
    };
    let mut fighter2 = Fighter {
        hp: rng.gen_range(5..10),
        attack: rng.gen_range(2..4),
        defense: rng.gen_range(1..5),
    };

    println!("Fighter 1: {:?}", fighter1);
    println!("Fighter 2: {:?}", fighter2);
    let mut turn_count = 0;
    while fighter2.hp > 0 && fighter1.hp > 0 { // Changed >= to > to avoid negative hp battle
        // turn 1
        turn_count += 1;
        fighter1.hp -= fighter2.attack - fighter1.defense;
        println!("Turn number {}", turn_count);
        println!("Fighter 1: {:?}", fighter1);

        if fighter1.hp <= 0 {
            println!("Fighter 2 wins!");
            break;
        }
        // turn 2
        turn_count += 1;
        fighter2.hp -= fighter1.attack - fighter2.defense;
        println!("Turn number {}", turn_count);
        println!("Fighter 2: {:?}", fighter2);

        if fighter2.hp <= 0 {
            println!("Fighter 1 wins!");
            break;
        }
    }
}
