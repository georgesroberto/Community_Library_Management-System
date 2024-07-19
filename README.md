# Community Library Management System

This project is a decentralized platform built on the Internet Computer, aiming to facilitate the management of books, members, loans, and reservations. It leverages the power of blockchain to ensure transparency and reliability in the management processes.

## Key Features

### Book Management
- **Create Book**: Allows the creation of new book records with validation for input fields.
- **Get Books**: Retrieves all registered book records.
- **Get Book by ID**: Retrieves the details of a specific book by its unique ID.

### Member Management
- **Create Member**: Allows the creation of new member records with validation for input fields.
- **Get Members**: Retrieves all registered member records.
- **Get Member by ID**: Retrieves the details of a specific member by their unique ID.

### Loan Management
- **Create Loan**: Allows the creation of new loans for books to members.
- **Get Book Loans**: Retrieves all registered loan records.
- **Get Loan by ID**: Retrieves the details of a specific loan by its unique ID.

### Reservation Management
- **Create Reservation**: Allows the creation of new reservations for books by members.
- **Get Reservations**: Retrieves all registered reservation records.
- **Get Reservation by ID**: Retrieves the details of a specific reservation by its unique ID.

### Error Handling
- **Not Found**: Returns an error if a requested resource (book, member, loan, reservation) is not found.
- **Invalid Input**: Handles errors related to invalid input fields.




## Requirements
* rustc 1.64 or higher
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```
* rust wasm32-unknown-unknown target
```bash
$ rustup target add wasm32-unknown-unknown
```
* candid-extractor
```bash
$ cargo install candid-extractor
```
* install `dfx`
```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background
```

If you want to start working on your project right away, you might want to try the following commands:

```bash
$ cd icp_rust_boilerplate/
$ dfx help
$ dfx canister --help
```

## Update dependencies

update the `dependencies` block in `/src/{canister_name}/Cargo.toml`:
```
[dependencies]
candid = "0.9.9"
ic-cdk = "0.11.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
ic-stable-structures = { git = "https://github.com/lwshang/stable-structures.git", branch = "lwshang/update_cdk"}
```

## did autogenerate

Add this script to the root directory of the project:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh
```

Update line 16 with the name of your canister:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh#L16
```

After this run this script to generate Candid.
Important note!

You should run this script each time you modify/add/remove exported functions of the canister.
Otherwise, you'll have to modify the candid file manually.

Also, you can add package json with this content:
```
{
    "scripts": {
        "generate": "./did.sh && dfx generate",
        "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
      }
}
```

and use commands `npm run generate` to generate candid or `npm run gen-deploy` to generate candid and to deploy a canister.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
$ dfx start --background

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```# Community_Library_Management-System
