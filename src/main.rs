mod test_reqwest;

fn main() {
    println!("Hello, world!");

    test_reqwest::test_reqwest::main_test_get();

    test_reqwest::test_reqwest::main_test_post();
}
