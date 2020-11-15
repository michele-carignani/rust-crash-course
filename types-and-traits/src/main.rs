fn main() {
    try_for_loop();
}

// Exercise 1.7.3

fn try_for_loop(){
	for i in 1..11 {
		println!("i == {}", i)
	}
}

// Exercise 1.7.1

fn try_loop() {
	let mut i = 1;

	loop {
		println!("i == {}", i);

		if i >= 10 {
			break;
		} else {
			i += 1;
		}
	}
}

// Exercise 1.6

fn fix_printer_1() {
    let val: String = "Hello".to_string();
    printer(val.clone());
    printer(val);
}

fn fix_printer_2() {
    let val = "Hello";
    printer_ref(&val);
    printer_ref(&val);
}

fn printer(v: String) {
    println!("with clone {}", v);
}

fn printer_ref(v: &str) {
    println!("with ref {}", v);
}
