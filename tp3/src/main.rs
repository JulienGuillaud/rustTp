fn main() {
    let test1 = String::from("Hello 1");
    let test2 = String::from("Hellooooo 2");
    
    println!("1er affichage \n");
    println!("{}", test1);
    println!("{}", test2);
    println!("\n2eme affichage \n");
    println!("{}", test1);
    println!("{}", test2);
    println!("\nIsbig 1 \n");
    println!("test1 : {}", isbig(&test1, 3));
    println!("test2 : {}", isbig(&test2, 30));
    println!("\nIsbig 2 \n");
    println!("test1 : {}", isbig(&test1, 3));
    println!("test2 : {}", isbig(&test2, 30));
    println!("\nInverse \n");
    println!("inverse(5) : {}", inverse(5));
    println!("inverse(0) : {}", inverse(0));
    println!("\nSafe \n");
    println!("safe_inverse(5) : {:?}", safe_inverse(5));
    println!("safe_inverse(0) : {:?}", safe_inverse(0));
    
    let v = vec![1, 2, 3, 4, 5];
    println!("\nVecteur \n");
    println!("v : {:?}", v);
    
    println!("\nBoucle");
    for i in v{
        println!("{}", i);
    }

    let mut vMut = vec![1, 2, 3, 4, 5];
    println!("\nVecteur mut \n");
    println!("v : {:?}", vMut);
    println!("Somme : {}", somme(&vMut));
    vMut.push(6);
    println!("v : {:?}", vMut);
    println!("Somme : {}", somme(&vMut));
    println!("\nVecteur max \n");
    println!("Max : {:?}", maximum(&vMut));



}

fn isbig(x: &String, longueur:i32) -> bool {
    x.len() as i32 > longueur
}

fn inverse(v:i32) -> i32 {
    1/v
}

fn safe_inverse(v:i32) -> Option<i32> {
    if v == 0 {
        None
    }else{
        Some(inverse(v))
    }
}

// Vector sum
fn somme(v: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in v{
        sum += i;
    }
    sum
}

fn maximum(v: &Vec<i32>) -> i32 {
    let mut max = v.first();
    for i in v{
        if i > max {
            max = i;
        }
    }
    max
}