mod test_reqwest;
mod guessing_game;

fn main() {
    println!("Hello, world!");

    guessing_game::guessing_game::main();

    test_reqwest::test_reqwest::main_test_get();

    test_reqwest::test_reqwest::main_test_post();
}
