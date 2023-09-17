use rand::Rng;

pub fn gen_value() -> String {
    let mut rng = rand::thread_rng();
    let numbers: Vec<u8> = vec![
        rng.gen_range(10..99),
        7,
        rng.gen_range(1..7),
        4,
        8, //rng.gen_range(5..8),
        rng.gen_range(0..1),
        200,
        9, // rng.gen_range(1..4),
        7,
        2, // rng.gen_range(0..3),
        1, // rng.gen_range(1..2),
        2, // rng.gen_range(2..3),
        8,
    ];
    numbers.iter().map(|x| format!("{}", x)).collect()
}