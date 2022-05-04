use std::io::Read;

fn main() {
    let mut memoire = vec![0];
    // let instructions = vec![Boucle(vec![Moins, Droite, Plus, Gauche])];
    // test let instructions = parse_v2(&"+-->>>>><+[++++[+-[-]><.]]".to_string()).unwrap_or(vec![]);
    //let instructions = parse_v2(&"++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.".to_string()).unwrap_or(vec![]);
    //let instructions = parse_v2(&"+++>+>-".to_string()).unwrap_or(vec![]);
    let text = std::fs::read_to_string("src/hello.bf".to_string());
    let instructions = parse_v2(&text.unwrap_or_default()).unwrap_or(vec![]);
    println!("MEMOIRE DEPART : {:?}", memoire);

    interpreteur(&mut memoire, &instructions, 0);
    println!("RESULTAT FINAL : {:?}", memoire);

    let mut resultat = String::new();

    for i in memoire {
        resultat += char::from_u32(i as u32).unwrap().to_string().as_str();
        //resultat += char::from_digit(((i as u32) + 103), 10).unwrap_or_default().to_string().as_str();
    }
}

// +-->>>>><+[++++[+-[-]><.]]

fn parse_v2(s: &String) -> Result<Vec<Instruction>, String>{
    let mut vec_instruction = vec![];
    let mut char_index = 0;
    let mut new_string = String::new();
    for c in s.chars() {
        if c != '[' && c != ']' {
            println!("ajout : {}", c);
            new_string += &c.to_string();
            if(char_index == s.len() - 1){
                let new_simple_text_parsed = parse(&new_string.to_string()).unwrap_or(vec![]);
                for i in new_simple_text_parsed {
                    vec_instruction.push(i);
                }
            }
        }else if c == '['{
            println!("Nouvelle boucle {}", new_string);
            let new_text = &s[0..char_index];
            let new_simple_text_parsed = parse(&new_string.to_string()).unwrap_or(vec![]);
            for i in new_simple_text_parsed {
                vec_instruction.push(i);
            }

            let new_recursive_in_loop = parse_v2(&new_text.to_string()).unwrap_or(vec![]);
            for i in new_recursive_in_loop {
                vec_instruction.push(i);
            }
        }else if c == ']'{
            println!("Fin boucle {}", new_string);
            let new_boucle = Instruction::Boucle(parse(&new_string.to_string()).unwrap_or(vec![]));
            vec_instruction.push(new_boucle);
        }
        char_index+=1;
    }
    Ok(vec_instruction)
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
