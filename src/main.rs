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
        sqrt
        add float
        mul float
        sub float
        div float
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

    print!("{value}");
}

fn get_number(op: &str, a: &[&str]) -> f64 {
    a.get(1)
        .unwrap_or_else(|| {
            println!("{op} requires arguments");
            print_help("")
        })
        .parse::<f64>()
        .unwrap_or_else(|_| {
            println!("{op} requires arguments to be floats");
            print_help("")
        })
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
            let start = get_number("between", a);
            let end = get_number("between", &a[1..]);
            let value = value * (end - start) + start;
            apply(&a[3..], value)
        }
        "pow" => {
            let power = get_number("pow", a);
            let value = value.powf(power);
            apply(&a[2..], value)
        }
        "add" => {
            let operand = get_number("add", a);
            let value = value + operand;
            apply(&a[2..], value)
        }
        "sub" => {
            let operand = get_number("sub", a);
            let value = value - operand;
            apply(&a[2..], value)
        }
        "mul" => {
            let operand = get_number("mul", a);
            let value = value * operand;
            apply(&a[2..], value)
        }
        "div" => {
            let operand = get_number("div", a);
            let value = value / operand;
            apply(&a[2..], value)
        }
        "log" => apply(&a[1..], value.log(E)),
        "sqrt" => apply(&a[1..], value.sqrt()),
        x => print_help(&format!("unknown operation {x}")),
    }
}
