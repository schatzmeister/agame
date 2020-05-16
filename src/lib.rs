use std::io;

pub fn repl() {
    startup();

    loop {
        let input = {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => input,
                Err(e) => panic!("Error occurred: {}", e),
            }
        };
        if input == "exit\n" {
            break;
        } else {
            println!("Status: {}", input);
        }
    }
}

fn startup() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    println!("Welcome to a game v{}", VERSION);
}
