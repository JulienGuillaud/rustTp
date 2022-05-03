use crate::Instruction::*;
use std::io;

fn main() {
    let code = vec![Lis, Plus, Affiche, Lis, Plus, Affiche, Lis, Plus, Lis, Plus, Affiche];
    let mut memoire = vec![0];

    println!("MEMOIRE DEPART : {:?}", memoire);

    interpreteur(&mut memoire, &code);
    println!("RESULTAT FINAL : {:?}", memoire)

}


fn interpreteur(memoire: &mut Vec<i32>, instructions: &Vec<Instruction>){
    let mut mem_index = 0;
    for instruction_index in 00..instructions.len() {
        let instruction = instructions[instruction_index].clone();
        match instruction {
            Plus => memoire[mem_index] += 1,
            Moins => memoire[mem_index] -= 1,
            Droite => move_side(memoire, &mut mem_index, instruction),
            Gauche => move_side(memoire, &mut mem_index, instruction),
            Affiche => println!("mem[{}] : {:?}", mem_index, memoire[mem_index]),
            Lis => {
                let my_int = stdin.read(guess);
                memoire[mem_index] = my_int;
            }
        }
    }
}

fn move_side(memoire: &mut Vec<i32>, mem_index: &mut usize, side: Instruction) {
    match side {
        Droite => {
            *mem_index += 1;
            if *mem_index >= memoire.len(){
                memoire.push(0)
            }
        },
        Gauche => {
            if *mem_index != 0 {
                *mem_index -= 1
            }else{
                println!("Dépassement de la limite de la memoire (coté gauche, ne pas se déplacer)")
            }
        }
        Plus => {}
        Moins => {}
        Affiche => {}
        Lis => {}
    }
}


#[derive(Clone)]
enum Instruction {
    Plus,
    Moins,
    Droite,
    Gauche,
    Affiche,
    Lis,
}
