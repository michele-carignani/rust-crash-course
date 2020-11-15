fn main() {

	println!("--- 2.10-3 ---");	
	exercise2_10_4();

	println!("--- 2.10-3 ---");	
	exercise2_10_3();

    println!("--- 2.6 ---");
    foobars();

    println!("--- 2.4 ---");
    let b : i32 = 4;
    my_drop(b);

}

// Exercise 2.10, 4 

fn double(y: Asdlol) -> Asdlol {
	Asdlol(y.0*2)
}

fn exercise2_10_4(){
	let x = Asdlol(1);
	let y = double(x);
	println!("{}", y.0);
}

// Exercise 2.10, 3 

#[derive(Debug,Copy,Clone)]
struct Asdlol(i32);

fn uses_asdlol(asd: Asdlol) {
	println!("{:?}", asd);
}

fn exercise2_10_3(){
	let x = Asdlol(1);
	uses_asdlol(x);
	uses_asdlol(x);
}

// Exercise 2.6

#[derive(Debug)]
struct Foobar(i32);

impl Drop for Foobar {
	fn drop(&mut self){
		println!("Dropping a Foobar: {:?}", self);
	}
}

impl Foobar {
	fn use_me(&self){
		println!("I consumed a Foobar as a method {:?}", self);	
	}
}

fn uses_foobar(foobar: &Foobar) {
	println!("I consumed a Foobar as a function {:?}", foobar);
}

fn foobars(){
	let x = Foobar(1);
	println!("Before uses_foobar");
	uses_foobar(&x);
	uses_foobar(&x);
	x.use_me();
	x.use_me();
	println!("After uses_foobar");
}

// Exercise 2.4

 fn my_drop<T>(_: T){
 }