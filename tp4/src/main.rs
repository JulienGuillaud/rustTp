use std::io::Read;

fn main() {
    let mut memoire = vec![0];
    // let instructions = vec![Boucle(vec![Moins, Droite, Plus, Gauche])];
    //let instructions = parse_v2(&"+++>+>-".to_string()).unwrap_or(vec![]);

    let mut texte = std::fs::read_to_string("src/hello.bf".to_string());
    let instructions = parse(&texte.unwrap_or_default());
    // let mut texte =  "+-->>>>><+[++++[+-[-]><.]]".to_string();
    // let instructions = parse(&texte);
    println!("MEMOIRE DEPART : {:?}", memoire);

    let instructions_opti = optimizeVector(&instructions);

    // autoComment println!("INSTRUCTIONS BASE : {:?}", instructions);
    // autoComment println!("INSTRUCTIONS OPTI : {:?}", instructions_opti);

    interpreteur(&mut memoire, &instructions_opti, 0, false);
    println!("RESULTAT FINAL : {:?}", memoire);

    let mut resultat = String::new();


    // autoComment println!("RESULTAT : {}", resultat);
}

fn parse(code: &str) -> Vec<Instruction> {
    parse_v2(&mut code.chars())
}
// +-->>>>><+[++++[+-[-]><.]]
fn parse_v2(chars: &mut std::str::Chars) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    while let Some(i) = chars.next() {
        match i {
            '+' => instructions.push(Instruction::Plus),
            '-' => instructions.push(Instruction::Moins),
            '>' => instructions.push(Instruction::Droite),
            '<' => instructions.push(Instruction::Gauche),
            '[' => instructions.push(Instruction::Boucle(parse_v2(chars))),
            ']' => { break; }
            ',' => instructions.push(Instruction::Lis),
            '.' => {
                instructions.push(Instruction::Affiche);
            }
            _ => (),
        };
    }
    instructions
}

fn optimizeVector(instructions: &Vec<Instruction>) -> Vec<Instruction> {
    let mut instructions_opti = vec![];

    let mut plus_moins_value = 0;
    let mut droite_gauche_value = 0;
    
    for i in 00..instructions.len() {

        let instruction_courante = instructions[i].clone();
        // autoComment println!("instruction_courante : {:?}", instruction_courante);
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
                        // autoComment println!("----Push instru opti argVal +- : {}", plus_moins_value);
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
                        // autoComment println!("----Push instru opti argVal +- : {}", plus_moins_value);
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
                        // autoComment println!("----Push instru opti argVal <> : {}", droite_gauche_value);
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
                        // autoComment println!("----Push instru opti argVal <> : {}", droite_gauche_value);
                        instructions_opti.push(Instruction::DroiteGauche(droite_gauche_value));
                        droite_gauche_value = 0;
                    }
                }
            }
            Instruction::Lis => instructions_opti.push(Instruction::Lis),
            Instruction::Affiche => instructions_opti.push(Instruction::Affiche),
            Instruction::Boucle(instructions_loop) => {
                if droite_gauche_value != 0 {
                    // autoComment println!("----Push instru opti argVal <> : {}", droite_gauche_value);
                    instructions_opti.push(Instruction::DroiteGauche(droite_gauche_value));
                    droite_gauche_value = 0;
                }else if plus_moins_value != 0 {
                    // autoComment println!("----Push instru opti argVal +- : {}", plus_moins_value);
                    instructions_opti.push(Instruction::DroiteGauche(plus_moins_value));
                    plus_moins_value = 0;
                }
                // let new_instructions_loop = instructions[0..i].to_vec();
                // autoComment println!("newVector (boucle) : {:?}", instructions_loop);
                instructions_opti.push(Instruction::Boucle(optimizeVector(&instructions_loop)));
            },
            Instruction::Fin => println!("FIN"),
            _ => println!("Erreur optimisation"),
        }
        
        // autoComment println!("\ndroite_gauche_value : {}", droite_gauche_value);
        // autoComment println!("plus_moins_value : {}", plus_moins_value);

    }

    instructions_opti

}


fn interpreteur(memoire_param: &mut Vec<i32>, instructions: &Vec<Instruction>, indexBoucle: usize, inBoucle: bool) {
  
    let mut memoire = memoire_param;
    // let mut mem_index = index as i32;
    let mut mem_index = 0;
    // println!("instructions : {:?}", instructions);
    for instructions_for in 00..instructions.len() {
        let instruction = instructions[instructions_for].clone();
        if(inBoucle && memoire[indexBoucle]==0){
            break;
        }
        println!("{:?}", memoire);
        match instruction {
            Instruction::PlusMoins(nb) => {
                // autoComment println!("PlusMoins : {} indx {}", nb, mem_index);
                if nb > 0 {
                    println!("+{} mem[{}]", nb, mem_index);
                }else{
                    println!("-{} mem[{}]", nb, mem_index);
                }
                memoire[mem_index as usize] += nb
            }
            Instruction::DroiteGauche(nb) => {
                if nb > 0 {
                    println!(">{}", nb);
                }else{
                    println!("<{}", nb);
                }
                // autoComment println!("DroiteGauche : {} indx {}", nb, mem_index);
                let new_index = mem_index + nb;
                if new_index >= memoire.len() as i32{
                    let diff = new_index - memoire.len() as i32 +1;
                    (0..diff).for_each(|_| memoire.push(0));
                }
                mem_index = new_index
            }
            Instruction::Affiche => {
                // // autoComment println!("mem[{}] : {:?}", mem_index, memoire[mem_index as usize]),
                let lettre = char::from_digit(memoire[mem_index as usize] as u32, 10).unwrap_or_default().to_string();
                // autoComment println!("-------------------------------------------------- {}", lettre);
            }
            Instruction::Lis => {
                // autoComment println!("Entrez un nombre : ");
                // input int from console
                let mut input = String::new();
                std::io::stdin().read_line(&mut input);
                // autoComment println!("input : {}",   input.trim().parse::<i32>().unwrap());
                memoire[mem_index as usize] = input.trim().parse::<i32>().unwrap();
            }
            Instruction::Boucle(les_instructions) => {
                while memoire[mem_index as usize] != 0
                {
                    // autoComment println!("lesInstructions : {:?}", &les_instructions);
                    //println!("Entrée dans boucle {:?}", &les_instructions);
                    interpreteur(memoire, &les_instructions, mem_index as usize, true);
                }
            }
            Instruction::Fin => println!("FIN"),
            Instruction::Plus => println!("PLUS"),
            Instruction::Droite => println!("DROITE"),
            Instruction::Gauche => println!("GAUCHE"),
            Instruction::Moins => println!("FIN"),
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
