use clap::{App, Arg};

fn main() {
    let required_opt: Arg = Arg::new("count").takes_value(true).required(true);

    let app: App = App::new("My Application")
        .author("authors name")
        .version("v1.0.0")
        .about("Application short description")
        .arg(required_opt);
    match app.try_get_matches() {
        Ok(m) => {
            let count = m.value_of("count").unwrap().parse::<i32>().unwrap();
            for n in 0..count {
                for _ in 0..((n + 1) * (n + 1)) {
                    print!("{}", "*");
                }
                print!("{}", "\n");
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
