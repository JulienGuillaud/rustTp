use std::io::Read;

fn main() {
    let mut memoire = vec![10, 15];
    // let instructions = vec![Boucle(vec![Moins, Droite, Plus, Gauche])];
    let instructions = parse(&"+--<>>>>><>.".to_string()).unwrap_or(vec![]);

    println!("MEMOIRE DEPART : {:?}", memoire);

    interpreteur(&mut memoire, &instructions, 0);
    println!("RESULTAT FINAL : {:?}", memoire)

}


fn parse(s: &String) -> Result<Vec<Instruction>, String>{
    // Boucle each char of string
    let mut instructions = vec![];
    for c in s.chars() {
        match c {
            '+' => instructions.push(Instruction::Plus),
            '-' => instructions.push(Instruction::Moins),
            '<' => instructions.push(Instruction::Gauche),
            '>' => instructions.push(Instruction::Droite),
            ',' => instructions.push(Instruction::Lis),
            '.' => instructions.push(Instruction::Affiche),
            _ => return Err(format!("Caractère non reconnu : {}", c)),
        }
    }
    Ok(instructions)
}

fn interpreteur(memoire: &mut Vec<i32>, instructions: &Vec<Instruction>, index: usize) {

    let mut instructions_opti = vec![];

    let mut same_in_row = 0;
    for i in 00..instructions.len() {
        let mut instruction_courante = instructions[i].clone();

        if i > 0 && instructions[i] == instructions[i-1]{
            same_in_row += 1;
        }else{
            match instruction_courante {
                Instruction::Plus => instructions_opti.push(Instruction::PlusOpti(same_in_row)),
                Instruction::Moins => instructions_opti.push(Instruction::MoinsOpti(same_in_row)),
                Instruction::Gauche => instructions_opti.push(Instruction::GaucheOpti(same_in_row)),
                Instruction::Droite => instructions_opti.push(Instruction::DroiteOpti(same_in_row)),
                Instruction::Lis => instructions_opti.push(Instruction::LisOpti(same_in_row)),
                Instruction::Affiche => instructions_opti.push(Instruction::AfficheOpti(same_in_row)),
                _ => println!("Erreur"),
            };
            same_in_row = 1;

        }
    }

    println!("INSTRUCTIONS BASE : {:?}", instructions);
    println!("INSTRUCTIONS OPTI : {:?}", instructions_opti);

    let mut mem_index = index;
    for instruction_index in 00..instructions.len() {
        let instruction = instructions[instruction_index].clone();
        println!("  mem avant {:?}", memoire);
        match instruction {
            Instruction::Plus => {
                println!("+");
                memoire[mem_index] += 1
            }
            Instruction::Moins => {
                println!("-");
                memoire[mem_index] -= 1
            }
            Instruction::Droite => {
                println!("D");
                let new_index = mem_index + 1;
                if new_index >= memoire.len(){
                    memoire.push(0)
                }
                mem_index = new_index

            }
            Instruction::Gauche => {
                println!("G");
                let mut new_index = mem_index;

                if new_index != 0 {
                    new_index -= 1;
                    mem_index = new_index;
                }else{
                    println!("Dépassement de la limite de la memoire (coté gauche, ne pas se déplacer)")
                }
            }
            Instruction::Affiche => println!("mem[{}] : {:?}", mem_index, memoire[mem_index]),
            Instruction::Lis => {
                println!("Entrez un nombre : ");
                // input int from console
                let mut input = String::new();
                std::io::stdin().read_line(&mut input);
                println!("input : {}",   input.trim().parse::<i32>().unwrap());
                memoire[mem_index] = input.trim().parse::<i32>().unwrap();
            }
            Instruction::Boucle(les_instructions) => {
                while memoire[mem_index] != 0
                {
                    println!("lesInstructions : {:?}", &les_instructions);
                    interpreteur(memoire, &les_instructions, mem_index);
                }
            }
            Instruction::PlusOpti(nb) => {
                println!("+{}", nb);
                memoire[mem_index] += nb
            }
            Instruction::MoinsOpti(nb) => {
                println!("+{}", nb);
                memoire[mem_index] -= nb
            }
            Instruction::DroiteOpti(nb) => {
                let new_index = mem_index + nb as usize;
                for i in 0..nb{
                    if new_index >= memoire.len(){
                        memoire.push(0)
                    }
                }
                mem_index = new_index
            }
            Instruction::GaucheOpti(nb) => {
                let mut new_index = mem_index;

                if new_index != 0 {
                    new_index -= 1;
                    mem_index = new_index;
                }else{
                    println!("Dépassement de la limite de la memoire (coté gauche, ne pas se déplacer)")
                }
            }
            Instruction::AfficheOpti(nb) => {}
            Instruction::LisOpti(nb) => {}
        }
    }
}

fn move_side(memoire: &mut Vec<i32>, mem_index: usize, side: Instruction) -> usize{
    let mut new_index = mem_index;
    match side {
        Instruction::Droite => {
            new_index += 1;
            if new_index >= memoire.len(){
                memoire.push(0)
            }
        },
        Instruction::Gauche => {
            if new_index != 0 {
                new_index -= 1
            }else{
                println!("Dépassement de la limite de la memoire (coté gauche, ne pas se déplacer)")
            }
        }
        _ => println!("Erreur"),
    }
    new_index
}


#[derive(Clone, Debug, PartialEq)]
enum Instruction {
    Plus,
    Moins,
    Droite,
    Gauche,
    Affiche,
    Lis,
    Boucle(Vec<Instruction>),
    PlusOpti(i32),
    MoinsOpti(i32),
    DroiteOpti(i32),
    GaucheOpti(i32),
    AfficheOpti(i32),
    LisOpti(i32),
}
