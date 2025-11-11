mod task;
mod utils;

use crate::task::{Task, TaskList, TaskStatus};
use crate::utils::read_input;

fn main() -> std::io::Result<()> {
    println!("📋 Todo CLI — powered by Rust + serde_json");
    let file_path = "tasks.json";

    let mut todolist = TaskList::load_from_json(file_path)?;
    println!("Loaded {} tasks from file.", todolist.list.len());

    loop {
        println!(
            "\n1️⃣  Add task\n2️⃣  Remove task\n3️⃣  Mark task\n4️⃣  Show list\n5️⃣  Save & Exit\n"
        );
        let choice = read_input("Enter choice: ");

        match choice.as_str() {
            "1" => add_task(&mut todolist),
            "2" => remove_task(&mut todolist),
            "3" => mark_task(&mut todolist),
            "4" => println!("{}", todolist),
            "5" => {
                todolist.save_to_json(file_path)?;
                println!("✅ Saved to {}", file_path);
                break;
            }
            _ => println!("❌ Invalid choice!"),
        }
    }

    Ok(())
}

fn add_task(todolist: &mut TaskList) {
    let name = read_input("Enter task name: ");
    let description = read_input("Enter task description: ");
    todolist.add(Task {
        name,
        description,
        status: TaskStatus::Unfinished,
    });
    println!("✅ Task added!");
}

fn remove_task(todolist: &mut TaskList) {
    println!("{}", todolist);
    let idx = read_input("Enter task index to remove: ");
    if let Ok(num) = idx.parse::<usize>() {
        if let Err(e) = todolist.remove(num) {
            println!("❌ {}", e);
        } else {
            println!("🗑️ Task removed!");
        }
    } else {
        println!("❌ Invalid input.");
    }
}

fn mark_task(todolist: &mut TaskList) {
    println!("{}", todolist);
    let idx = read_input("Enter task index to mark: ");
    if let Ok(num) = idx.parse::<usize>() {
        let status_input = read_input("Enter 1 for Done, 0 for Not Done: ");
        let status = match status_input.as_str() {
            "1" => TaskStatus::Finished,
            _ => TaskStatus::Unfinished,
        };
        if let Err(e) = todolist.mark(num, status) {
            println!("❌ {}", e);
        } else {
            println!("✅ Task updated!");
        }
    } else {
        println!("❌ Invalid input.");
    }
}
