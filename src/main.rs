use std::f64::consts::E;

fn print_help(err: &str) -> ! {
    println!(
        "{err}\nUsage:
    random <...function>

        functions get applied one after another.

        functions are:

        floor
        round
        ceil
        between float float
        log
        pow float
            "
    );
    std::process::exit(1);
}
fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    if args.contains(&String::from("--help")) {
        print_help("");
    }
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    let value: f64 = rand::random();
    let value = apply(&args, value);

    println!("{value}");
}

fn apply(a: &[&str], value: f64) -> f64 {
    if a.is_empty() {
        return value;
    }

    match a[0] {
        "floor" => apply(&a[1..], value.floor()),
        "ceil" => apply(&a[1..], value.ceil()),
        "round" => apply(&a[1..], value.round()),
        "between" => {
            let start = a
                .get(1)
                .unwrap_or_else(|| print_help("between requires 2 arguments"))
                .parse::<f64>()
                .unwrap_or_else(|_| print_help("between needs arguments to be floats"));
            let end = a
                .get(2)
                .unwrap_or_else(|| print_help("between requires 2 arguments"))
                .parse::<f64>()
                .unwrap_or_else(|_| print_help("between needs arguments to be floats"));
            let value = value * (end - start) + start;
            apply(&a[3..], value)
        }
        "pow" => {
            let power = a
                .get(1)
                .unwrap_or_else(|| print_help("pow requires 2 arguments"))
                .parse::<f64>()
                .unwrap_or_else(|_| print_help("pow needs arguments to be floats"));

            let value = value.powf(power);
            apply(&a[2..], value)
        }
        "log" => apply(&a[1..], value.log(E)),
        x => print_help(&format!("unknown operation {x}")),
    }
}
