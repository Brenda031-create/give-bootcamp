use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
}

#[contract]
pub struct TodoList;

const TODOS: Symbol = symbol_short!("TODOS");

const NEXT_ID: Symbol = symbol_short!("NEXT_ID");

#[contractimpl]
impl TodoList {
    pub fn create_todo(env: &Env, title: String, description: String) -> Todo {
        let mut current_id = env.storage().temporary().get(&NEXT_ID).unwrap_or(1);

        let mut todos = env
            .storage()
            .temporary()
            .get(&TODOS)
            .unwrap_or(Vec::new(env));

        let new_todo = Todo {
            id: current_id,
            title,
            description,
            is_completed: false,
        };

        todos.push_back(new_todo.clone());

        env.storage().temporary().set(&TODOS, &todos);

        current_id += 1;

        env.storage().temporary().set(&NEXT_ID, &current_id);

        new_todo
    }

    pub fn update_todo(env: &Env, id: u32, new_title: String, new_description: String) -> bool {
        let mut todos = Self::get_todos(env);

        for i in 0..todos.len() {
            let mut todo = todos.get(i).unwrap();

            if todo.id == id {
                todo.title = new_title.clone();
                todo.description = new_description.clone();
                todo.is_completed = false;

                todos.set(i, todo);

                env.storage().temporary().set(&TODOS, &todos);

                return true;
            }
        }

        false
    }

    pub fn get_todos(env: &Env) -> Vec<Todo> {
        env.storage()
            .temporary()
            .get(&TODOS)
            .unwrap_or(Vec::new(env))
    }

    pub fn get_next_id(env: &Env) -> u32 {
        env.storage().temporary().get(&NEXT_ID).unwrap_or(1)
    }

    // pub fn mark_is_completed()-> bool;
    pub fn mark_is_completed(env: &Env, id: u32) -> bool {
        let mut todos = Self::get_todos(env);

        for i in 0..todos.len() {
            let mut todo = todos.get(i).unwrap();

            if todo.id == id {
                todo.is_completed = true;

                todos.set(i, todo);

                env.storage().temporary().set(&TODOS, &todos);

                return true;
            }
        }

        false
    }
    // pub delete_todo()->bool;
}
