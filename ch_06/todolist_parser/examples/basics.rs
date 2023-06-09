use todolist_parser::TodoList;

fn main() {
    let todos = TodoList::get_todos("examples/todos");
    match todos {
        Ok(list) => println!("{:?}", list),
        Err(e) => {
            println!("{}", e.description());
            println!("{:?}", e)
        }
    }
}
