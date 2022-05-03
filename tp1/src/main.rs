fn mad(a:i32, b:i32, c:i32) -> i32 {
    a + b + c
}

fn sum_from_to_while(mut a:i32, mut b:i32) -> i32 {
    let mut resultat = 1;
    if a > b { (a, b) = (b, a); }
    while b > a {
        resultat += b;
        b = b - 1;
    }
    resultat
}

fn sum_from_to_for(a:i32, b:i32) -> i32 {
    let mut sum = 0;
    for i in a..b+1 {
        sum += i;
    }
    sum
}

fn sum_from_to_recu(current:i32, to:i32) -> i32 {
    if current == to {
        return current;
    }else{
        return current + sum_from_to_recu(current+1, to);
    }
}

fn sum_from_to_rust(a:i32, b:i32) -> i32 {
    (a..b+1).sum()
}

fn sum_from_to_rust_v2(a:i32, b:i32) -> i32 {
    (a..b+1).fold(0, |a, b| a + b)
}

#[derive(Debug, Clone)]
struct Livre {
    titre: String,
    annee_publication: u32,
    genre: Genre,
}

fn age_livre(livre: Livre) -> u32 {
    let now = 2022;
    now - livre.annee_publication
}

#[derive(Debug, Clone)]
enum Genre{
    Fiction,
    Histoire,
    Fantasy,
    Informatique
}

fn note_livre(livre: Livre) -> u32 {
    match livre.genre {
        Genre::Fiction => ((livre.titre.to_string().len() as u32) + (livre.annee_publication)) * 12,
        Genre::Histoire => ((livre.titre.to_string().len() as u32) + (livre.annee_publication)) * 2,
        Genre::Fantasy => ((livre.titre.to_string().len() as u32) + (livre.annee_publication)) * 36,
        Genre::Informatique => ((livre.titre.to_string().len() as u32) + (livre.annee_publication)) * 41,
    }
}

#[derive(Debug)]
enum ResultatDivision{
    DivisionParZero,
    DivisionCorrecte(f32)
}

fn division(a:f32, b:f32) -> ResultatDivision {
    if b == 0.00 {
        ResultatDivision::DivisionParZero
    }else{
        ResultatDivision::DivisionCorrecte(a as f32 / b as f32)
    }
}

fn divisionOption(a:f32, b:f32) -> Option<f32> {
    if b == 0.00 {
        None
    }else{
        Some(a / b)
    }
}


fn main() {
    println!("Hello, world!");
    println!("mad(a, b, c) : {}", mad(1, 2, 3));
    println!("\n--------- sum_from_to_for --------\n");
    println!("sum_from_to_for(1,20) : {}", sum_from_to_for(1, 20));
    println!("sum_from_to_while(1,20) : {}", sum_from_to_while(1, 20));
    println!("sum_from_to_while(20,1) parametres inversés : {}", sum_from_to_while(20, 1));
    println!("sum_from_to_recu(1,20) : {}", sum_from_to_recu(1, 20));
    println!("sum_from_to_rust(1,20) : {}", sum_from_to_rust(1, 20));
    println!("sum_from_to_rust_v2(1,20) : {}", sum_from_to_rust_v2(1, 20));
    
    println!("\n--------- struct Livre --------\n");
    let livre1 = Livre {
        titre: String::from("Le Petit Prince"),
        annee_publication: 1943,
        genre: Genre::Fiction,
    };
    let livre2 = Livre {
        titre: String::from("Livre 2"),
        annee_publication: 2000,
        genre: Genre::Histoire,
    };
    println!("livre 1 : '{}' publié le : {} du genre : {:?}", livre1.titre, livre1.annee_publication.to_string(), livre1.genre);
    println!("age_livre 2 : {}", age_livre(livre1.clone()));
    println!("livre 1 : '{}' publié le : {} du genre : {:?}", livre2.titre, livre2.annee_publication.to_string(), livre2.genre);
    println!("age_livre 2 : {}", age_livre(livre2.clone()));
    
    println!("\n--------- Notes --------\n");
    
    println!("note_livre 1 : {}", note_livre(livre1.clone()));
    println!("note_livre 2 : {}", note_livre(livre2.clone()));
    
    
    println!("\n--------- enum avancé --------\n");
    
    println!("division 10 par 2 : {:?}", division(10.0, 2.0));
    println!("division 10 par 0 : {:?}", division(10.0, 0.0));

    println!("\ndivision 10 par 2 : {:?}", divisionOption(10.0, 2.0));
    println!("division 10 par 0 : {:?}", divisionOption(10.0, 0.0));


    println!("\n--------- TESTS i64 to i32 --------\n");
    pub const MAXI32: i32 = i32::MAX;
    pub const MAXI: i64 = i64::MAX;
    let i32Test: i32 = MAXI as i32;
    println!(" MAX i64 = {}", MAXI);
    println!(" MAX i32 = {}", MAXI32);
    println!(" i64 max to i32 max = {}", i32Test);
    println!("\n--------- TESTS u64 to u32 --------\n");
    pub const MAXU32: u32 = u32::MAX;
    pub const MAXU: u64 = u64::MAX;
    let i32Test: u32 = MAXU as u32;
    println!(" MAX u64 = {}", MAXU);
    println!(" MAX u32 = {}", MAXU32);
    println!(" u64 max to u32 max = {}", i32Test);
    
}
