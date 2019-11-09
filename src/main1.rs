fn add (a: i32, b: i32) -> i32 {
    a + b
}

enum GameType {
    SinglePlayer,
    MultiPlayer(u32),
}

fn choose_game (game: GameType) {
    match game {
        GameType::SinglePlayer => println!("How about solitaire?"),
        GameType::MultiPlayer(2) => println!("How about checkers?"),
        GameType::MultiPlayer(4) => println!("How about bridge?"),
        GameType::MultiPlayer(num) => {
            println!("How about {}-player tag?", num)
        },
    }
}

fn fizz(num: u32) -> String {
    if num % 3 == 0 {
        String::from("Fizz")
    } else {
        num.to_string()
    }
}

fn main() {
    let name = "Sílvia";
    println!("Hello, world {}!", name);

    let mut apples = 100;
    apples += 50;
    println!("I have {} apples", apples);

    println!("{}", add(1, 2));

    let color = [0, 255, 255];
    let index = 2;
    println!("The 10th element is {:?}", color[index]);

    let hour = 14;
    match hour {
        0..=6 => println!("It's dawn"),
        7..=11 => println!("It's the morning"),
        12..=18 => println!("It's the afternoon"),
        17..=24 => println!("It's the evening"),
        _ => {
            panic!("Time is broken!");
        }
    }

    let mut prices = vec![30, 100, 2];
    prices[0] = 25;
    prices.push(40);
    println!("All the prices are: {:?}", prices);

    let names = vec!["Carol", "Jake", "Marylou", "Bruce"];
    for name in names {
        println!("Hi {}!", name);
    }

    let mut i = 0;
    while i < 10 {
        println!("i = {}", i);
        i += 1;
    }

    loop {
        println!("i = {}", i);
        i += 1;
        if i >= 10 {
            break;
        }
    }

    for i in (0..10).map(|x| x * x ) {
        println!("i = {}", i);
    }

    let sum = (0..10).fold(0, |acc, x| {
        acc + x
    });
    println!("sum = {}", sum);

    choose_game(GameType::SinglePlayer);
    choose_game(GameType::MultiPlayer(6));

    println!("{}", fizz(3));
    println!("{}", fizz(5));

    let mut instructors = vec!["Carol"];
    let a = instructors.pop();
    println!("a is {:?}", a);
    let b = instructors.pop();
    println!("b is {:?}", b);

    let c = Some("Carol");
    let name = c.expect("No name present");
    println!("Name is {} bytes long", name.len());
}
