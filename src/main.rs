use std::mem;
mod snh;
mod pm;

use std::rc::Rc;
use std::sync::{Mutex,Arc};
use std::thread;

struct Person
{
    name: Arc<String>,
    state: Arc<Mutex<String>>
}

impl Person
{
    fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Person
    {
        Person { name: name, state: state }
    }

    fn greet(&self)
    {
        let mut state = self.state.lock().unwrap();

        state.clear();
        state.push_str("excited");

        println!("Hi, my name is {} and I am {}", self.name, state.as_str());
    }
}

fn rc_demo()
{
    //Arc
    let name = Arc::new("John".to_string());
    let state = Arc::new(Mutex::new("bored".to_string()));
    let person = Person::new(name.clone(), state.clone());


    let t = thread::spawn(move || {
        person.greet();
    });
    println!("Name = {}, state = {}", name, state.lock().unwrap().as_str());


    t.join().unwrap();







 /*
    //Rc
    let name = Rc::new("John".to_string());
    println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
    {
        let person = Person::new(name.clone());
        println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));

        person.greet();
    }
    println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));

    println!("Name = {}", name);*/

}

fn main()
{
    rc_demo();
}
















/*struct Person
{
    name: String

}

impl Person
{
    fn get_ref_name(&self) -> &String
    {
        &self.name
    }
}

struct Company<'z>
{
    name: String,
    ceo: &'z Person
}

fn main()
{
    let mut z: &String;
    {
        let p = Person {name: String::from("John")};
        z = p.get_ref_name();

    }
}*/























/*fn main() {

    let print_vector = |x:&Vec<i32>|
        {
            println!("x[0] = {}",x[0]);
            //x.push(123);
        };

    let v = vec![3,2,1];
    print_vector(&v);

    let mut a = 40;

    {
        let b = &mut a;
        *b += 2;
    }
    println!("a = {}", a);

    let mut z = vec![3,2,1];

    for i in &z
    {
        println!("i = {}", i);
    }
}*/










/*let v = vec![1,2,3];

    //let v2 = v;

    println!("{:?}", v);

    let foo = |v:Vec<i32>| ();
    foo(v);

    //println!("{:?}", v);

    let u = 1; // i32
    let u2 = u;
    println!("u = {}", u);
    let u = Box::new(1);
    let u2 = u;

    //println!("u = {}", *u);*/






/*struct Circle{radius: f64}
struct Square {side: f64}

trait Shape{
    fn area(&self) -> f64;
}

impl Shape for Square {
    fn area(&self) -> f64
    {
        self.side * self.side
    }
}

impl Shape for Circle {
    fn area(&self) -> f64
    {
        self.radius * self.radius * std::f64::consts::PI
    }
}

fn main() {

    let shapes:[&Shape; 4] = [
        &Circle{radius: 1.0},
        &Square {side: 3.0},
        &Circle{radius: 2.0},
        &Square {side: 4.0}
    ];



    for (i, shape) in shapes.iter().enumerate()
    {
        println!("Shape #{} has area {}", i, shape.area());
        area(shape);
    }



}*/



















/*trait Printable
{
    fn format(&self) -> String;
}

impl Printable for i32
{
    fn format(&self) -> String
    {
        format!("i32: {}", *self)
    }
}

impl Printable for String
{
    fn format(&self) -> String
    {
        format!("string: {}", *self)
    }
}

fn print_it_too(z: &Printable)
{
    println!("{}", z.format());
}

/*fn print_it<T: Printable>(z: T)
{
    println!("{}", z.format());
}*/*/
























/*#[derive(Debug)]
struct Point
{
    x: f64,
    y: f64
}

use std::ops::Add;

impl Add<Vector> for Point
{
    type Output = Point;

    fn add(self, other:Point) -> Point
    {
        Point {x: self.x + other.x, y: self.y+other.y }
    }
}
*/





/*trait Animal
{
    fn create(name: &'static str) -> Self;

    fn name(&self) -> &'static str;


    fn talk(&self)
    {
        println!("{} cannot talk", self.name());
    }

}

trait Summable<T>
{
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32>
{
    fn sum(&self) -> i32
    {
        let mut result:i32 =0;
        for x in self { result+= *x;}
        return result;
    }
}

struct Cat
{
    name: &'static str
}

impl Animal for Cat
{
    fn create(name:&'static str) -> Cat
    {
        Cat{name:name}
    }


    fn name(&self) -> &'static str
    {
        self.name
    }

    fn talk(&self)
    {
        println!("{} says meow", self.name());
    }

}

struct Human
{
    name: &'static str
}

impl Animal for Human
{

    fn create(name:&'static str) -> Human
    {
        Human{name: name}
    }

    fn name(&self) -> &'static str
    {
        self.name
    }

    fn talk(&self)
    {
        println!("{} says hello", self.name());
    }
}

fn traits()
{
    let h = Human::create("John");

    h.name();
    h.talk();

    let c = Cat{name:"Misty"};
    c.talk();

    let a = vec![1,2,3];
    println!("sum = {}", a.sum());
}*/





















/*fn is_even(x: i32) -> bool
{
    x % 2 == 0
}



fn hof()
{
    let limit = 500;
    let mut sum = 0;
    for i in 0..
    {
        let isq = i*i;
        if isq > limit {break;}
        else if is_even(isq) {sum += isq;}

    }

    println!("loop sum = {}", sum);

    let sum2 =
        (0..).map(|x| x*x)
            .take_while(|&x| x <= limit)
            .filter(|x| is_even(*x))
            .fold(0, |sum, x| sum +x);
    println!("hof sum = {}", sum2)

}*/















/*fn say_hello()
{
    println!("hello");
}


fn closures()
{
    let sh = say_hello;
    sh();

    let plus_one = |x:i32| -> i32 {x +1};
    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

    let mut two = 2;
    {
        let plus_two = |x|
            {
                let mut z = x;
                z += two;
                z
            };

        println!("{} + 2 = {}", 3, plus_two(3));
    }

    let borrow_two = &mut two;

    // T: by value
    // T&
    // &mut &
    let plus_three = |x:&mut i32| *x +=3;
    let mut f = 12;
    plus_three(&mut f);
    println!("f = {}", f);

}*/





























/*struct Point
{
    x: f64,
    y: f64
}

struct Line
{
    start: Point,
    end: Point
}

impl Line
{
    fn len(&self) -> f64
    {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx*dx+dy*dy).sqrt()
    }
}





fn methods()
{
    let p = Point { x:3.0, y:4.0};
    let p2 = Point {x: 6.0, y:10.0};

    let myline = Line {start: p, end: p2};
    println!("length = {}", myline.len())
}*/



















/*fn increase(x: &mut i32)
{
    *x +=1;
}

fn product(x: i32, y: i32) -> i32
{
    x*y
}

fn functions()
{
    print_value(33);

    let mut z:i32 = 1;
    increase(&mut z);
    print_value(z);

    let a = 3;

    let b = 5;
    let p = product(a, b);
    print_value(p);
}

fn print_value(x: i32)
{
    println!("value = {}", x);
}
*/







/*struct Point<T>
{
    x: T,
    y: T
}

fn generics()
{
    let a:Point<u32> = Point {x: 0, y: 0};
    let b:Point<f64>  = Point { x:1.2, y: 3.4};
}*/






/*fn sum_and_product(x:i32, y:i32) -> (i32, i32)
{
    (x+y, x*y)
}

fn tuples()
{

    let x = 3;
    let y = 4;
    let sp = sum_and_product(x,y);
    println!("sp = {:?}",sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x,y, sp.0, sp.1);

    //destructuring

    let (a,b) = sp;
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x,y, a, b);

    let sp2 = sum_and_product(4,7);
    let combined = (sp, sp2);
    println!("{:?}", combined);
    println!("last elem = {}", (combined.1).1);

    let ((c,d),(e,f)) = combined;
    let foo = (true, 42.0, -1i8);
    println!("{:?}", foo);

    let meaning = (42,);
    println!("{:?}", meaning);


}*/







/*fn strings()
{
    let s:&'static str = "hello there!";

    for c in s.chars().rev()
    {
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0)
    {
        println!("first letter is {}", first_char)
    }

    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <=('z' as u8)
    {
        letters.push(a as char);
        letters.push_str(",");
        a+=1;
    }

    println!("{}",letters);

     &str <> String

    let u:&str = &letters;

    let z = letters + "abc";
    //let z = letters + &letters;
    let mut abc = String::from("hello world");

    let mut abc = "hello world".to_string();
    abc.remove(0);
    abc.push_str("!!!");
    println!("{}",abc.replace("ello", "goodbye"));
}*/

















/*fn use_slice(slice: &mut[i32])
{
    println!("first elem = {}, len ={}", slice[0],slice.len());
    slice[0] = 4321;
}


fn slices()
{
    let mut data = [1,2,3,4,5];
    use_slice(&mut data[1..4]);
    println!("{:?}", data);
}*/





















//Same as lists
/*fn vectors()
{
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);
    a.push(44);
    println!("a = {:?}", a);

    let idx:usize = 0;

    println!("a[0] = {}", a[idx]);


    match a.get(6)
    {
        Some(x) => println!("a[6] = {}",x),
        None => println!("Error, no such element")
    }

    for x in &a { println!("{}", x); }

    a.push(77);
    println!("{:?}", a);
    let last_elem = a.pop();
    println!("Last elem is {:?}, a is {:?}", last_elem, a);

    let last_value = a.pop();

    while let Some(x) = a.pop()
    {
        println!("{}", x);
    }
}*/









/*fn array()
{
    let mut a:[i32;5] = [1,2,3,4,5];
    println!("a has {} elements, first is {}", a.len(), a[0]);
    a[0] = 1;
    println!("a[0] = {}", a[0]);

    println!("{:?}",a);
    if a != [1,2,3,4,5]
    {
        println!("does not match");
    }

    let b = [1; 10];
    for i in 0..b.len()
    {
        println!("{}",b[i])
    }

    println!("b took up {} bytes", mem::size_of_val(&b));

    let mtx:[[f32;3]; 2] =
    [
      [2.0,0.0,0.0],
      [3.0,5.0,0.0]
    ];

    println!("{:?}", mtx);

    for i in 0..mtx.len()
    {
        for j in 0..mtx[i].len()
        {
            if i == j
            {
                println!("mtx[{}][{}] = {}",i,j, mtx[i][j]);
            }
        }
    }
}
*/












/*fn option()
{
    // Option<T>

    let x = 3.0;
    let y = 0.0;

    let result:Option<f64> =
        if y != 0.0 { Some(x/y) } else { None };
    println!("{:?}", result);

    match result {
        Some(z) => println!("{}/{} = {}", x, y, z),
        None => println!("cannot divide {} by {}", x, y)
    }

    if let Some(z) = result { println!("z = {}", z); }
}*/
































/*union IntOrFloat
{
    i: i32,
    f: f32
}

fn process_value(iof:IntOrFloat)
{
    unsafe
        {
            match iof
            {
                IntOrFloat {i: 42} => {println!("meaning of life");}
                IntOrFloat {f} => {println!("f32 = {}", f)}

            }
        }
}


fn unions()
{
    let mut iof = IntOrFloat {i: 123};

    unsafe { iof.i = 42; }
    let value = unsafe {iof.i};
    process_value(iof);
    process_value(IntOrFloat{f: 1.23});
}*/












/*enum Color
{
    Red,
    Green,
    Blue,
    RgbColor(u8,u8,u8),
    CmykColor{
        cyan:u8, magenta:u8, yellow:u8, black:u8
    }
}*/


/*fn enums()
{
    let c:Color = Color::CmykColor{cyan: 0, magenta:128, yellow: 0, black: 255};
    match c
    {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        //Two dots means that we need to know only about black:)
        Color::CmykColor {black:255,..} => println!("black"),
        Color::RgbColor(0,0,0) => println!("Black"),
        Color::RgbColor(r,g,b) => println!("rgb({}, {}, {})",r,g,b),
        _ => ()
    }
}*/



/*struct Point
{
    x: f64,
    y: f64
}

struct Line
{
    start: Point,
    end: Point
}

fn structures()
{
    let p = Point {x: 3.0, y: 4.0};
    println!("point p is at ({}, {})", p.x, p.y);

    let p2 = Point {x:5.0,y:10.0};
    let myLine = Line {start:p,end:p2};
}*/

/*fn match_statement()
{
    let country_code = 7; // 1 999

    let country = match country_code
    {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1...999 => "unknown",
        _ => "invalid"
    };
    println!("the country with code {} is {}",country_code,country);
}*/

/*fn for_loop()
{
    for x in 1..11
    {
        println!("x = {}", x);
    }

    for (pos, y) in (30..41).enumerate()
    {
        println!("{}: {}", pos, y);
    }
}*/





/*fn while_and_loop()
{
    let mut x =1;
    while x < 1000
    {
        x*=2;

        if x == 64 {continue;}

        println!("x = {}", x);
    }

    let mut y = 1;
    loop {
        y*=2;
        println!("y = {}", y);

        if y == 1 << 10 {break;}
    }

}*/








/*fn if_statement()
{
    let temp = 35;
    if temp > 30
    {
        println!("really hot outside");
    }
    else if temp < 10
    {
        println!("really cold");
    }
    else
    {
        println!("temp is okay");
    }

    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("Today is {}", day);
    println!("it is {}",
    if temp > 20 {if temp > 30 {"very hot"} else {"hot"}}
    else if temp < 10 {"cold"} else {"OK"});
}*/

/*fn scope_and_shadowing()
{
    let a = 123;

    {
        let b = 456;
        println!("inside, b = {}", b);
    }

    println!(" a = {}", a);

}*/

/*fn operators()
{
    let mut a = 2+3*4;
    println!("{}", a);
    a = a+1;
    a -= 2;
    let a_cubed = i32::pow(a,3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);

    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    let c = 1 | 2;
    println ! ("1|2 = {}", c);
    let two_to_10 = 1 << 10;
    println!("2^10 = {}", two_to_10);

    //logical
    let pi_less4 =  std::f64:consts::PI < 4.0;
    // > <= >= ==
    let x = 5;
    let x_is5 = x == 5;

}*/

fn variables()
{
    /* //
        let a:i8 = -123;
        println!("a = {}", a);

        let mut b:i8 = 0;
        println!("b = {}", b);

        b = 42;
        println!("b = {}", b);

        let mut c = 123456789;
        println!("c = {}, size = {} bytes", c,  mem::size_of_val(&c) );
        c = -1;
        println!("c = {} after modification", c );

        let z:isize = 123;
        let size_of_z = mem::size_of_val(&c);
        println!("z = {}, takes up {} bytes, {}-bit OS",
            z, size_of_z, size_of_z*8);

        let d:char = 'x';

        println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));
        let e:f32 = 2.5;
        println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

        // true false
        let g = false;
        println!("e = {}, size = {} bytes", g, mem::size_of_val(&e));

        let f = 4 > 0;
        println!("e = {}, size = {} bytes", f, mem::size_of_val(&e));*/
}