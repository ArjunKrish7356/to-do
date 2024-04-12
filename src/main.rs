use std::io;

struct Task{
    name : String,
    stat : bool,
}

struct Todolist{
    items : Vec<Task>,
}

impl Todolist{

    fn New()->Todolist{
        Todolist{
            items: Vec::new(),
        }
    }

    fn add(&mut self, name: String) {
        let task = Task {
            name: name,
            stat: false,
        };
        self.items.push(task);
    }

    fn show(&self){
        let mut i=1;
        let mut formatted_name:String ;
        println!("Todolist is :");
        for task in &self.items {
            let task_name = if !task.stat { &task.name } else { 
                formatted_name = format!("\x1B[9m{}\x1B[0m", task.name);
                &formatted_name
            };
            println!("{}. {} ", i, task_name);
            i += 1;
        }
    }
    fn comp(&mut self){
        println!("Enter the Task no :");
        let mut task_no=String::new();
        let _ = io::stdin().read_line(&mut task_no);
        let task_no :usize =task_no.trim().parse().unwrap();
        self.items[task_no-1].stat=true;
        self.show() 
    }
    fn del(&mut self){
        println!("Enter the Task no :");
        let mut task_no=String::new();
        let _ = io::stdin().read_line(&mut task_no);
        let task_no :usize =task_no.trim().parse().unwrap();
        self.items.remove(task_no-1);
        self.show();
    }

}




fn main(){
    let mut todo_list=Todolist::New();
    loop{
        println!("Enter task : 1.Add 2.Show 3.Comp 4.Del 5.Exit");
        let mut choice=String::new();
        let _ =io::stdin().read_line(&mut choice);
        let choice: u32 = choice.trim().parse().unwrap();
        if choice==1{
            println!("Enter the Task name :");
            let mut task_name=String::new();
            let _ = io::stdin().read_line(&mut task_name);
            todo_list.add(task_name.trim().to_string());
        }
        else if choice==2 {
            todo_list.show();
        }
        else if choice==3 {
            todo_list.comp();
        }
        else if choice ==4 {
            todo_list.del();
        }
        else if choice == 5{
            break;
        }
    }    
}


