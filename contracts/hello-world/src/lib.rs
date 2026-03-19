#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Env, Symbol, Vec, Address
};

// Structure for a task
#[derive(Clone)]
#[contracttype]
pub struct Task {
    pub id: u32,
    pub content: Symbol,
    pub completed: bool,
}

#[contract]
pub struct TodoContract;

#[contractimpl]
impl TodoContract {

    // Add a new task
    pub fn add_task(env: Env, user: Address, content: Symbol) {
        user.require_auth();

        let mut tasks: Vec<Task> = env
            .storage()
            .instance()
            .get(&user)
            .unwrap_or(Vec::new(&env));

        let id = tasks.len() as u32 + 1;

        let task = Task {
            id,
            content,
            completed: false,
        };

        tasks.push_back(task);
        env.storage().instance().set(&user, &tasks);
    }

    // Get all tasks
    pub fn get_tasks(env: Env, user: Address) -> Vec<Task> {
        env.storage()
            .instance()
            .get(&user)
            .unwrap_or(Vec::new(&env))
    }

    // Mark task as completed
    pub fn complete_task(env: Env, user: Address, task_id: u32) {
        user.require_auth();

        let mut tasks: Vec<Task> = env
            .storage()
            .instance()
            .get(&user)
            .unwrap_or(Vec::new(&env));

        for i in 0..tasks.len() {
            let mut task = tasks.get(i).unwrap();
            if task.id == task_id {
                task.completed = true;
                tasks.set(i, task);
            }
        }

        env.storage().instance().set(&user, &tasks);
    }
}

mod test;