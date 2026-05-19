# SEP-41 Token Contract

This project implements a simple SEP-41 style token contract using Soroban. The token is called `SibToken` and uses the symbol `SIB`.

The main contract implementation is in:

```text
contracts/sep41-token/src/our_token.rs
```

The tests are in:

```text
contracts/sep41-token/src/test.rs
```

## Token Storage

The contract stores balances and allowances using persistent storage.

Balances are stored with:

```rust
DataKey::Balance(address)
```

Allowances are stored with:

```rust
DataKey::Allowance(AllowanceKey { from, spender })
```

An allowance means one address, called the `spender`, is allowed to spend or burn tokens from another address, called `from`.

## Burn

```rust
pub fn burn(env: Env, from: Address, amount: i128) -> Result<(), ContractError>
```

The `burn` function removes tokens from the `from` address.

First, the function requires authorization from the address whose tokens are being burned:

```rust
from.require_auth();
```

This means the owner of the tokens must approve the burn.

Next, the function gets the current balance of `from`:

```rust
let from_balance = Self::balance(env.clone(), from.clone());
```

It then checks whether the balance is enough:

```rust
if from_balance < amount {
    return Err(ContractError::InsufficientFunds);
}
```

If the balance is too low, the function returns `InsufficientFunds`.

If the balance is enough, it subtracts the burned amount from the user's balance and saves the new balance back into storage:

```rust
env.storage()
    .persistent()
    .set(&DataKey::Balance(from.clone()), &(from_balance - amount));
```

Finally, it publishes a `Burn` event:

```rust
Burn {
    from,
    amount: amount.try_into().unwrap(),
}
.publish(&env);
```

If everything succeeds, the function returns:

```rust
Ok(())
```

## Burn From

```rust
pub fn burn_from(
    env: Env,
    spender: Address,
    from: Address,
    amount: i128,
) -> Result<(), ContractError>
```

The `burn_from` function allows a spender to burn tokens from another user's balance, but only if the spender has enough allowance.

First, the function requires authorization from the spender:

```rust
spender.require_auth();
```

This means the spender must approve the action.

Then the function checks the balance of the `from` address:

```rust
let from_balance = Self::balance(env.clone(), from.clone());
```

If the `from` address does not have enough tokens, the function returns:

```rust
ContractError::InsufficientFunds
```

After that, the function checks how much allowance the spender has:

```rust
let allowance = Self::allowance(env.clone(), from.clone(), spender.clone());
```

If the allowance is less than the amount being burned, the function returns:

```rust
ContractError::InsufficientAllowance
```

If both checks pass, the function reduces the `from` balance:

```rust
env.storage()
    .persistent()
    .set(&DataKey::Balance(from.clone()), &(from_balance - amount));
```

Then it reduces the spender's allowance:

```rust
env.storage().persistent().set(
    &DataKey::Allowance(AllowanceKey {
        from: from.clone(),
        spender: spender.clone(),
    }),
    &(allowance - amount),
);
```

Finally, it publishes a `Burn` event and returns `Ok(())`.

This means `burn_from` changes two things:

- The token owner's balance goes down.
- The spender's allowance goes down.

## Mint

```rust
pub fn mint(env: Env, to: Address, amount: i128) -> Result<(), ContractError>
```

The `mint` function creates new tokens and adds them to the `to` address.

First, it requires authorization from the receiving address:

```rust
to.require_auth();
```

Then it reads the current balance of the receiver:

```rust
let to_balance = Self::balance(env.clone(), to.clone());
```

The function adds the minted amount to the receiver's current balance and saves it back into storage:

```rust
env.storage()
    .persistent()
    .set(&DataKey::Balance(to.clone()), &(to_balance + amount));
```

Finally, it publishes a `Mint` event:

```rust
Mint {
    to,
    amount: amount.try_into().unwrap(),
}
.publish(&env);
```

If the mint succeeds, it returns:

```rust
Ok(())
```

## Events

The contract uses events to record important token actions.

For the assignment functions:

- `Burn` is published when tokens are burned.
- `Mint` is published when new tokens are created.

The events are defined in:

```text
contracts/sep41-token/src/events.rs
```

## Testing

To run all tests:

```powershell
cd contracts/sep41-token
cargo test
```

To run only one test:

```powershell
cargo test test_burn
cargo test test_burn_from
cargo test test_mint
```

### Burn Test

The burn test first creates a test environment and enables mocked authorization:

```rust
setup_result.env.mock_all_auths();
```

Because the contract requires authorization with `require_auth`, mocked authorization allows the test to call the function without real account signatures.

The test manually gives the sender an initial balance of `1000` tokens:

```rust
setup_result.env.as_contract(&setup_result.contract_id, || {
    setup_result
        .env
        .storage()
        .persistent()
        .set(&DataKey::Balance(setup_result.sender.clone()), &1000i128);
});
```

The storage write is wrapped in `env.as_contract` because contract storage can only be accessed from inside the contract context.

Then the test burns `500` tokens:

```rust
setup_result.client.burn(&setup_result.sender, &500);
```

After burning, the test checks that the sender has `500` tokens left:

```rust
assert_eq!(balance, 500);
```

### Burn From Test

The burn-from test gives the sender `1000` tokens and gives the receiver an allowance of `700` tokens.

In this test:

- `sender` is the token owner.
- `receiver` is the spender.

The allowance is stored like this:

```rust
DataKey::Allowance(AllowanceKey {
    from: setup_result.sender.clone(),
    spender: setup_result.receiver.clone(),
})
```

Then the test calls:

```rust
setup_result
    .client
    .burn_from(&setup_result.receiver, &setup_result.sender, &500);
```

This means the receiver is burning `500` tokens from the sender's balance.

After the call, the test checks two results:

```rust
assert_eq!(sender_balance, 500);
assert_eq!(remaining_allowance, 200);
```

This proves that:

- The sender's balance was reduced from `1000` to `500`.
- The receiver's allowance was reduced from `700` to `200`.

### Mint Test

The mint test enables mocked authorization and calls:

```rust
setup_result.client.mint(&setup_result.receiver, &1000);
```

This mints `1000` tokens to the receiver.

Then it checks the receiver's balance:

```rust
assert_eq!(balance, 1000);
```

This proves that the minted tokens were added to the receiver's stored balance.
