use std::{
    io::{self, Write},
    usize,
};

fn main() {
    let mut contacts: Vec<Vec<String>> = vec![];

    loop {
        show_menu();
        let choice = get_choice();

        match choice {
            1 => {
                contacts.push(create_contact());
                println!("Contato cadastrado com sucesso!");
            }
            2 => show_contacts(&contacts),
            3 => delete_contact(&mut contacts),
            0 => {
                println!("Até mais, e obrigado pelos peixes!");
                break;
            }
            _ => println!("Opção inválida"),
        }

        println!();
    }
}

fn show_menu() {
    println!("======= MENU =======");
    println!("1. Novo contato");
    println!("2. Listar contatos");
    println!("3. Remover contato");
    println!("0. Sair");
}

fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().expect("Falha ao limpar o buffer");
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler entrada");
    input.trim().to_string()
}

fn get_choice() -> i32 {
    read_input("> ").parse().unwrap_or(-1)
}

fn create_contact() -> Vec<String> {
    vec![
        read_input("Nome: "),
        read_input("Email: "),
        read_input("Telefone: "),
    ]
}

fn show_contacts(contacts: &Vec<Vec<String>>) {
    if contacts.is_empty() {
        println!("Nenhum contato cadastrado");
        return;
    }
    for (index, contact) in contacts.iter().enumerate() {
        println!("{} - {}, {}, {}", index, contact[0], contact[1], contact[2]);
    }
}

fn delete_contact(contacts: &mut Vec<Vec<String>>) {
    if contacts.is_empty() {
        println!("Nenhum contato cadastrado");
        return;
    }

    let index = read_input("Qual o índice do contato que deseja remover? ")
        .parse()
        .unwrap_or(usize::MAX);

    if index < contacts.len() {
        contacts.remove(index);
        println!("Contato removido com sucesso!");
        return;
    }

    println!("Índice inválido!");
}
