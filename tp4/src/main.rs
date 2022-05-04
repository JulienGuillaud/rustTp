use std::io::Read;

fn main() {
    let mut memoire = vec![10, 15];
    // let instructions = vec![Boucle(vec![Moins, Droite, Plus, Gauche])];
    let instructions = parse(&"+-<>><><>.,".to_string()).unwrap_or(vec![]);

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
                mem_index = move_side(memoire, mem_index, instruction);

            }
            Instruction::Gauche => {
                println!("G");
                mem_index = move_side(memoire, mem_index, instruction);
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
        }
    }
}

fn move_side(memoire: &mut Vec<i32>, mem_index: usize, side: Instruction) -> usize{
    let mut newIndex = mem_index;
    match side {
        Instruction::Droite => {
            newIndex += 1;
            if newIndex >= memoire.len(){
                memoire.push(0)
            }
        },
        Instruction::Gauche => {
            if newIndex != 0 {
                newIndex -= 1
            }else{
                println!("Dépassement de la limite de la memoire (coté gauche, ne pas se déplacer)")
            }
        }
        Instruction::Plus => {}
        Instruction::Moins => {}
        Instruction::Affiche => {}
        Instruction::Lis => {}
        Instruction::Boucle(les_instructions) => {}
    }
    newIndex
}


#[derive(Clone, Debug)]
enum Instruction {
    Plus,
    Moins,
    Droite,
    Gauche,
    Affiche,
    Lis,
    Boucle(Vec<Instruction>)
}
