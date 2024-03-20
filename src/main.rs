
use std::fs;
use std::io;
use serde::{Serialize,Deserialize};

    #[derive(Debug,Deserialize)]
struct AllList{
    tasks:Vec<String>
}

impl AllList {
    //responsavel por criar uma nova instância a struct
    fn new() -> AllList{
        AllList{ tasks: Vec::new()}
    }
  
    fn add_task(&mut self, task:String){
        self.tasks.push(task);
    }

    fn check_task(&mut self,task_index:usize){
        if task_index < self.tasks_index.len(){
            self.tasks[task_index] = format!("{} (feito)",self.tasks[task_index]);
            println!("Tarefa marcada como feita!");
        }else{
            println!("indice de tarefa inválido.")
    }
 }

    fn list_tasks(&self){
        for (i,task) in self.tasks.iter().enumerate(){
            println!("essas são as suas listas: ");
            println!("{}: {}", i +1,task)
        }
    }
    fn remove_task(&mut self,index:usize){
        self.tasks.remove(index - 1);
    } 
}




//created task
fn main(){
   
     loop {
         println!("escolha umas dessas opções com o seu perspectivo numero");
        println!("'1' adicionar uma tarefa na sua lista");
        println!("'2' mudar a tarefa para feita");
        println!("'3' atualizar sua lista de tarefas");
        println!("'4' remover a lista de tarefas");
        println!("'5'sair");

        let mut opções = String::new();
        io::stdin().read_line(&mut opções).expect("failed to read line");
        println!("{}",opções); 

        opções = opções.trim().to_string();

    if opções == "1"{
        println!("deu certo");
        


    }
    else if opções == "5"{
        break
    }



 }
}

