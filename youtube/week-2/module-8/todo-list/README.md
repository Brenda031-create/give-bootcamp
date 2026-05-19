# Todo List Soroban Contract

This project is a simple Soroban smart contract for managing a todo list. It allows users to create todos, update existing todos, mark todos as completed, delete todos, and read the stored list of todos.

The main contract code is found in:

```text
contracts/todo-list/src/todo.rs
```

The tests are found in:

```text
contracts/todo-list/src/test.rs
```
## Todo Data Structure

Each todo is represented by the `Todo` struct:

```rust
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
}
```

Each todo has:

- `id`: a unique number used to identify the todo.
- `title`: the title of the todo.
- `description`: more details about the todo.
- `is_completed`: a boolean value showing whether the todo has been completed.

## Storage

The contract uses Soroban temporary storage to save data:

```rust
const TODOS: Symbol = symbol_short!("TODOS");
const NEXT_ID: Symbol = symbol_short!("NEXT_ID");
```

`TODOS` stores the list of todos.

`NEXT_ID` stores the next id that should be assigned when a new todo is created.

## Main Functions

### Create Todo

```rust
pub fn create_todo(env: &Env, title: String, description: String) -> Todo
```

This function creates a new todo. It gets the next id from storage, creates a new `Todo`, adds it to the todo list, saves the updated list back into storage, increments the next id, and returns the newly created todo.

New todos start with:

```rust
is_completed: false
```

This means every todo is incomplete when it is first created.

### Update Todo

```rust
pub fn update_todo(env: &Env, id: u32, new_title: String, new_description: String) -> bool
```

This function updates the title and description of an existing todo. It searches through the list using the todo `id`.

If it finds the todo, it updates it, saves the changed list back into storage, and returns `true`.

If no todo with that id exists, it returns `false`.

## Mark Todo As Completed

```rust
pub fn mark_is_completed(env: &Env, id: u32) -> bool
```

This is one of the functions added to the contract. Its job is to mark a todo as completed.

The function first gets the current list of todos:

```rust
let mut todos = Self::get_todos(env);
```

Then it loops through the list:

```rust
for i in 0..todos.len() {
    let mut todo = todos.get(i).unwrap();
```

For each todo, it checks whether the todo id matches the id passed into the function:

```rust
if todo.id == id {
```

When it finds the correct todo, it changes the todo's completion status:

```rust
todo.is_completed = true;
```

After changing the todo, it puts the updated todo back into the same position in the list:

```rust
todos.set(i, todo);
```

Then it saves the updated todo list back into contract storage:

```rust
env.storage().temporary().set(&TODOS, &todos);
```

Finally, it returns `true`:

```rust
return true;
```

This tells the caller that the todo was found and successfully marked as completed.

If the loop finishes and no todo with that id is found, the function returns:

```rust
false
```

That means the todo could not be marked as completed because the id did not match any stored todo.

## Delete Todo

```rust
pub fn delete_todo(env: &Env, id: u32) -> bool
```

This is the other function added to the contract. Its job is to remove a todo from the todo list.

Like the completion function, it first gets the current list of todos:

```rust
let mut todos = Self::get_todos(env);
```

Then it loops through every todo in the list:

```rust
for i in 0..todos.len() {
    let todo = todos.get(i).unwrap();
```

For each todo, it checks whether the todo id is the same as the id passed into the function:

```rust
if todo.id == id {
```

When the matching todo is found, it removes that todo from the vector:

```rust
todos.remove(i);
```

After removing it, the function saves the new todo list back into storage:

```rust
env.storage().temporary().set(&TODOS, &todos);
```

Then it returns `true`:

```rust
return true;
```

This means the todo was found and successfully deleted.

If no todo with the given id is found, the function returns:

```rust
false
```

This means nothing was deleted because the id did not exist in the list.

## Testing

To test the contract, run:

```powershell
cd contracts/todo-list
cargo test
```

The tests create todos and check that the contract functions behave correctly.

For example, the mark-as-completed test creates a todo, calls `mark_is_completed` using the todo id, gets the todo list again, and checks that:

```rust
completed_todo.is_completed == true
```

The delete test creates a todo, calls `delete_todo` using the todo id, gets the todo list again, and checks that:

```rust
todos.len() == 0
```

This confirms that the todo was removed from storage.
