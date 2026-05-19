#![cfg(test)]

use crate::todo::{TodoList, TodoListClient};

use soroban_sdk::{Env, String};

struct SetUpResult<'a> {
    env: Env,
    client: TodoListClient<'a>,
}

fn setup<'a>() -> SetUpResult<'a> {
    let env = Env::default();

    let contract_id = env.register(TodoList, ());

    let client = TodoListClient::new(&env, &contract_id);

    SetUpResult { env, client }
}

#[test]
fn test_create_todo() {
    let setup_result = setup();

    let title = String::from_str(&setup_result.env, "stellar impact bootcamp");

    let description = String::from_str(&setup_result.env, "soroban smartcontract");

    let todo_creation = setup_result.client.create_todo(&title, &description);

    assert_eq!(todo_creation.title, title);

    assert_eq!(todo_creation.description, description);
}

#[test]
fn test_update_todo() {
    let setup_result = setup();

    let title = String::from_str(&setup_result.env, "stellar impact bootcamp");

    let description = String::from_str(&setup_result.env, "soroban smartcontract");

    let todo_creation = setup_result.client.create_todo(&title, &description);

    let new_title = String::from_str(&setup_result.env, "stellar impact bootcamp wk2");

    let new_description = String::from_str(&setup_result.env, "soroban smartcontract wk2");

    let id = todo_creation.id;

    let update_todo_call = setup_result
        .client
        .update_todo(&id, &new_title, &new_description);

    assert!(update_todo_call, "failed to update");

    let todos = setup_result.client.get_todos();

    let updated_todo = todos.get(0).unwrap();

    assert_eq!(updated_todo.title, new_title);
    assert_eq!(updated_todo.description, new_description);
}

// #[test]
// fn test_mark_completed
#[test]
fn test_mark_is_completed(){
   let setup_result = setup();

    let title = String::from_str(&setup_result.env, "stellar impact bootcamp");

    let description = String::from_str(&setup_result.env, "soroban smartcontract");

    let todo_creation = setup_result.client.create_todo(&title, &description);

    let id = todo_creation.id;

    let mark_todo_call = setup_result.client.mark_is_completed(&id);

    assert!(mark_todo_call, "failed to mark todo as completed");

    let todos = setup_result.client.get_todos();

    let completed_todo = todos.get(0).unwrap();

    assert_eq!(completed_todo.title, title);
    assert_eq!(completed_todo.description, description);
    assert_eq!(completed_todo.is_completed, true);

}

// #[test]
// fn test_delete_todo
#[test]
fn test_delete_todo(){
   let setup_result = setup();

    let title = String::from_str(&setup_result.env, "stellar impact bootcamp");

    let description = String::from_str(&setup_result.env, "soroban smartcontract");

    let todo_creation = setup_result.client.create_todo(&title, &description);

    let id = todo_creation.id;

    let delete_todo_call = setup_result.client.delete_todo(&id);

    assert!(delete_todo_call, "failed to delete todo");

    let todos = setup_result.client.get_todos();

    assert_eq!(todos.len(), 0);
}