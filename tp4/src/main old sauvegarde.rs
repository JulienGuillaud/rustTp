use crate::Instruction::*;

fn main() {
    let code = vec![Plus, Plus, Plus, Droite, Plus, Droite, Moins, Affiche];
    let mut memoire = vec![0, 0, 0];

    println!("{:?}", memoire);

    interpreteur(&mut memoire, &code)
}


fn interpreteur(memoire: &mut Vec<i32>, instructions: &Vec<Instruction>){
    let mut fini = false;
    let mut instructionIndex = 0;
    let mut memIndex = 0;
    while !fini {
        if instructionIndex == instructions.len(){
            println!("Fini !");
            fini=true;
        }else{
            let instruction = instructions[instructionIndex].clone();
            println!("instruction : {:?}", instruction);
            match instruction {
                Plus => memoire[memIndex] += 1,
                Moins => memoire[memIndex] -= 1,
                Droite => memIndex += 1,
                Gauche => memIndex -= 1,
                Affiche => println!("RESULTAT FINAL : {:?}", memoire),
            }
            instructionIndex+=1;
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Plus,
    Moins,
    Droite,
    Gauche,
    Affiche,
}
