use std::io::{self, Write};

fn main(){
    let mut todos: Vec<String> = Vec::new();

    loop{
        print!("Enter todo (or type 'list' or 'quit'): ");
        io::stdout().flush().unwrap();
        
        let mut input= String::new();
        io::stdin().read_line(&mut input).unwrap();
        let cmd=input.trim();
        
        if cmd=="quit"{
            break;
        }else if cmd == "list"{
            println!("do these work stypid: ");
            for (i,todo) in todos.iter().enumerate(){
                println!("{}. {}", i+1, todo);
            }
        }else{
            todos.push(cmd.to_string());
            println!("Added: {}", cmd);
        }
    }

    println!("Goodbye");
}
