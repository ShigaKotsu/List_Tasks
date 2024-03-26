use std::io::{self, Write};

struct Lista {
    tasks: Vec<String>,
}
impl Lista {
    fn new() -> Lista {
        Lista { tasks: Vec::new() }
    }

    //Adicionar uma tarefa
    fn add_task(&mut self, task: String) {
        self.tasks.push(task);
    }

    //Função para ver as tarefas
    fn list_tasks(&self) {
        println!("Estas são as suas listas de tarefas:");
        for (i, task) in self.tasks.iter().enumerate() {
            println!("Tarefa {}: {}", i + 1, task);
        }
    }

    // Função pra remover uma tarefa
    //fn list_remove(&mut self, index: usize) {
    // if index < self.tasks.len() {
    //    self.tasks.remove(index);
    //} else {
    //    println!("Índice inválido. Tarefa não encontrada.");
    //}

    // Função para marcar como concluida
    fn list_completed(&mut self, index: usize) {
        if index < self.tasks.len() {
            println!("Tarefa marcada como concluída: {}", self.tasks[index]);
        } else {
            println!("Índice inválido. Tarefa não encontrada.");
        }
    }
}

fn main() {
    let mut lista = Lista::new();
    let mut input = String::new();

    let menu_aplicacao = vec![
        "1. Adicionar tarefa",
        "2. Marcar a tarefa como feita",
        "3. Mostrar as tarefas",
    ];

    for opcao in &menu_aplicacao {
        println!("{}", opcao);
    }

    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let escolha: usize = input.trim().parse().unwrap();

    if escolha == 1 {
        print!("Digite a tarefa que deseja adicionar: ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        lista.add_task(input.trim().to_string());
    } else if escolha == 2 {
        input.clear();
        print!("Digite o índice da tarefa concluída: ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let index: usize = input.trim().parse().unwrap();
        lista.list_completed(index - 1);
    } else if escolha == 3 {
        lista.list_tasks();
    }
}
