use std::io::Read;

fn main() {
    let mut memoire = vec![10, 15];
    // let instructions = vec![Boucle(vec![Moins, Droite, Plus, Gauche])];
    let instructions = parse(&"+-->>>>><+[++++[+-[-]><.]]".to_string()).unwrap_or(vec![]);

    println!("MEMOIRE DEPART : {:?}", memoire);

    interpreteur(&mut memoire, &instructions, 0);
    println!("RESULTAT FINAL : {:?}", memoire)

}

// +-->>>>><+[++++[+-[-]><.]]

fn parse_v2(s: &String) -> Result<Vec<Instruction>, String>{
    let mut vecInstruction = vec![];
    let mut charIndex = 0;
    let mut newString = String::new();
    for c in s.chars() {
        if c != '[' && c != ']' {
            newString += &c.to_string();
        }else if c == '['{
            println!("Nouvelle boucle {}", newString);
            let newText = &s[0..charIndex];
            vecInstruction.push((parse_v2(&newText.to_string()).unwrap_or(vec![])));
        }
        charIndex+=1;
    }
    Ok(vecInstruction)
}

fn parse(s: &String) -> Result<Vec<Instruction>, String>{

    let mut instructionsBoucles = vec![s.clone()];
    let mut nBoucle = s.matches("[").count();
    println!("s length : {}", s.len());
    println!("nBoucle : {}", nBoucle);


    for i in 0..nBoucle {
        println!("Recherche de la boucle {}", i);

        let mut index_de_la_boucle_trouve = 0;
        let mut indexChar = 0;
        let mut index_debut_boucle = 0;
        // je pars du début pour récupérer l'index du début de la boucle
        for c in s.chars() {
            if c == '[' {
                if index_de_la_boucle_trouve == i {
                    index_debut_boucle = indexChar;
                    break;
                }
                index_de_la_boucle_trouve += 1;
            }
            indexChar+=1;
        }
        // println!("index_debut_boucle : {}", index_debut_boucle);
        // je pars de la fin pour récupérer l'index de la fin de la boucle
        let reversed = String::from(s.chars().rev().collect::<String>());
        // println!("reversed : {}", reversed);
        let mut index_de_la_boucle_trouve_r = 0;
        let mut indexChar_r = 0;
        let mut index_fin_boucle = 0;
        // je pars du début pour récupérer l'index du début de la boucle
        for c in reversed.chars() {
            if c == ']' {
                if index_de_la_boucle_trouve_r == i {
                    index_fin_boucle = indexChar_r;
                    break;
                }
                index_de_la_boucle_trouve_r += 1;
            }
            indexChar_r+=1;
        }
        // println!("index_fin_boucle : {}", index_fin_boucle);
        index_fin_boucle = s.len() - index_fin_boucle;
        // Créer les vecteurs grace aux index des boucles
        let boucle = &s[index_debut_boucle..index_fin_boucle];
        instructionsBoucles.push(boucle.to_string());
    }

    println!("BOUCLES : {:?}", instructionsBoucles);

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

    let mut plus_moins_value = 0;
    let mut droite_gauche_value = 0;
    
    for i in 00..instructions.len() {

        let instruction_courante = instructions[i].clone();
        println!("instruction_courante : {:?}", instruction_courante);
        let mut prochaine_instruction = Instruction::Fin;

        if (i+1) < instructions.len() {
            prochaine_instruction = instructions[i+1].clone();

        }else{
            prochaine_instruction = Instruction::Fin
        }

        match instruction_courante{
            Instruction::Plus => {
                plus_moins_value += 1;
                match prochaine_instruction {
                    Instruction::Plus | Instruction::Moins => {
                        // Rien faire
                    }
                    _ => {
                        println!("----Push instru opti argVal +- : {}", plus_moins_value);
                        instructions_opti.push(Instruction::PlusMoins(plus_moins_value));
                        plus_moins_value = 0;
                    }
                }
            }
            Instruction::Moins => {
                plus_moins_value -= 1;
                match prochaine_instruction {
                    Instruction::Plus | Instruction::Moins => {
                        // Rien faire
                    }
                    _ => {
                        println!("----Push instru opti argVal +- : {}", plus_moins_value);
                        instructions_opti.push(Instruction::PlusMoins(plus_moins_value));
                        plus_moins_value = 0;
                    }
                }
            }
            Instruction::Droite => {
                droite_gauche_value += 1;
                match prochaine_instruction {
                    Instruction::Droite | Instruction::Gauche => {
                        // Rien faire
                    }
                    _ => {
                        println!("----Push instru opti argVal +- : {}", droite_gauche_value);
                        instructions_opti.push(Instruction::DroiteGauche(droite_gauche_value));
                        droite_gauche_value = 0;
                    }
                }
            }
            Instruction::Gauche => {
                droite_gauche_value -= 1;
                match prochaine_instruction {
                    Instruction::Droite | Instruction::Gauche => {
                        // Rien faire
                    }
                    _ => {
                        println!("----Push instru opti argVal +- : {}", droite_gauche_value);
                        instructions_opti.push(Instruction::DroiteGauche(droite_gauche_value));
                        droite_gauche_value = 0;
                    }
                }
            }
            Instruction::Lis => instructions_opti.push(Instruction::Lis),
            Instruction::Affiche => instructions_opti.push(Instruction::Affiche),
            _ => println!("Erreur"),
        }
        
        println!("\ndroite_gauche_value : {}", droite_gauche_value);
        println!("plus_moins_value : {}", plus_moins_value);

    }

    println!("INSTRUCTIONS BASE : {:?}", instructions);
    println!("INSTRUCTIONS OPTI : {:?}", instructions_opti);

    
    let mut mem_index = index as i32;
    for instructions in 00..instructions_opti.len() {
        let instruction = instructions_opti[instructions].clone();
        println!("  mem avant {:?}", memoire);
        match instruction {
            Instruction::PlusMoins(nb) => {
                println!("PlusMoins : {} indx {}", nb, mem_index);
                memoire[mem_index as usize] += nb
            }
            Instruction::DroiteGauche(nb) => {
                println!("DroiteGauche : {} indx {}", nb, mem_index);
                let new_index = mem_index + nb;
                if new_index >= memoire.len() as i32{
                    let diff = new_index - memoire.len() as i32 +1;
                    (0..diff).for_each(|_| memoire.push(0));
                }
                mem_index = new_index
            }
            Instruction::Affiche => println!("mem[{}] : {:?}", mem_index, memoire[mem_index as usize]),
            Instruction::Lis => {
                println!("Entrez un nombre : ");
                // input int from console
                let mut input = String::new();
                std::io::stdin().read_line(&mut input);
                println!("input : {}",   input.trim().parse::<i32>().unwrap());
                memoire[mem_index as usize] = input.trim().parse::<i32>().unwrap();
            }
            Instruction::Boucle(les_instructions) => {
                while memoire[mem_index as usize] != 0
                {
                    println!("lesInstructions : {:?}", &les_instructions);
                    interpreteur(memoire, &les_instructions, mem_index as usize);
                }
            }
            Instruction::Fin => (),
            _ => println!("Erreur"),
        }
    }
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
    PlusMoins(i32),
    DroiteGauche(i32),
    Fin,
}
