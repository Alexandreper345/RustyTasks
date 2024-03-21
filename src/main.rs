use std::io::{self , Write};
use serde::{Deserialize};

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
        if task_index < self.tasks.len(){
            self.tasks[task_index] = format!("{} (feito)",self.tasks[task_index]);
            println!("Tarefa marcada como feita!");
        }else{
            println!("indice de tarefa inválido.")
    }
 }

    fn list_tasks(&self){
        if  !self.tasks.is_empty(){
             println!("essas são as suas listas: ");
        for (i,task) in self.tasks.iter().enumerate(){
             println!("{}: {}", i +1,task)
        }
   
     }else{
        println!("sua lista de tarefas está vazia.")
     }
}

    fn remove_task(&mut self,index:usize){
        self.tasks.remove(index - 1);
    } 
}




//created task
fn main(){
    let mut all_list = AllList::new();
    let mut input = String::new();


    
  
        
        println!("escolha umas dessas opções com o seu perspectivo numero");
        println!("'1' adicionar uma tarefa na sua lista");
        println!("'2' mudar a tarefa para feita");
        println!("'3' mostrar a lista de tarefas");
        println!("'4' remover a lista de tarefas");
        println!("'5'sair");
    loop{
        input.clear();
        print!("escolha uma opção: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        

       let  opções: u32 = input.trim().parse().unwrap();

    if opções == 1{
        input.clear();
        print!("Digite a tarefa: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        all_list.add_task(input.trim().to_string());
        
    }else  if opções == 2{
      input.clear();
      print!("digite o numero da tafera que você completou: ");
      io::stdout().flush().unwrap();
      io::stdin().read_line(&mut input).unwrap();
      let task_index: usize = input.trim().parse().unwrap();
      all_list.check_task(task_index - 1)
      }
    

    else  if opções == 3{
      all_list.list_tasks()
      }

    else if  opções == 4{
      input.clear();
      print!("digite o numero da tarefa: ");
      io::stdout().flush().unwrap();
      io::stdin().read_line(&mut input).unwrap();
      let index:usize = input.trim().parse().unwrap();
      all_list.remove_task(index);
      }
    
    else if opções == 5{
        break
    }

    else{
    println!("numero invalido que numero você quis dizer")
    }


 
    }
}
