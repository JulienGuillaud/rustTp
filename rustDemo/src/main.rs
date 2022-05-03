fn factorial(i: i64) -> i64 {
    if i == 0
    {
        1
    }
    else
    {
        i * factorial(i - 1)
    }
}

fn factorial_while(mut i:i64) -> i64{
    let mut resultat =1;
    while i > 0
    {
        resultat *=i;
        i = i-1;
    }

    resultat
}

fn factorial_rust(i:i64) -> i64{
    (1..(i+1)).fold(1, |acc, x| acc * x)
}


#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

fn distance(p1: Point, p2: Point, p3: Point) -> i64 {
    let x_diff = p1.x - p2.x;
    let y_diff = p1.y - p2.y;
    let z_diff = p1.z - p2.z;
    x_diff * x_diff + y_diff * y_diff + z_diff * z_diff
}
/*
enum Color{
    Red, Green, Blue
}

fn color_to_string(c:Color){
    match c{
        Color::Red => "red",
        Color::Green => "green",
        Color::Blue => "blue",
    };
    println!("{}", c);
}
*/
fn shape_area(s: Shape) -> f64 {
    let PI = 3.14159;
    match s {
        Shape::Circle(r) => {
            println!("Circle area: {}", PI * r * r);
            PI * r * r
        }
        Shape::Rectangle(w, h) => {
            println!("Rectangle area: {}", w * h);
            w * h
        }
        Shape::Square(w) => {
            println!("Square area: {}", w * h);
            w * h
        }
    }
}

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Square(f64),
}

fn main() {
    println!("Hello, world!");
    for i in 1..10 {
        println!("fact({}) = {}", i, factorial_while(i));
    }

    let s0 = Shape::Rectangle(3.0, 4.0);
    let s1 = Shape::Square(10.0);
    let s2 = Shape::Circle(5.0);

    shape_area(s0);
    shape_area(s1);
    shape_area(s2);
    
}
