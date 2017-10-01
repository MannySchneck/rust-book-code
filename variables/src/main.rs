fn main() {
    let mut x = 6;

    println!("This is silly: {}", x);

    x = 4;

    println!("I am a mutant. Yay?: {}", x);

    so_much_fun(1);

    println!("so am programmer {}", now_i_return_fear_me());

    println!("print me added: {}", add_me(4));

    if x < 12{
        println!("me");
    } else {
        println!("you!");
    }

    for num in (1..5).rev() {
        println!("{}!", num);
    }

    println!("LIFTFOFFFF!!!!");
}

fn so_much_fun(wow_a_param: u64) {
    println!(
        "All the lulz with as big a number as you want babe {}",
        wow_a_param
    );
}

fn now_i_return_fear_me() -> i32 {
    5
}

fn add_me(num: i32) -> i32 {
    num + 1
}
