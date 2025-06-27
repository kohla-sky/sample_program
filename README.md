# Rust Solana Program with Nested Path Dependencies

This is a sample Rust Solana program that demonstrates how to create and use multiple path dependencies that reference each other in a nested structure.

## Project Structure

```
.
├── Cargo.toml                    # Workspace configuration
├── Anchor.toml                   # Anchor framework configuration
├── programs/
│   └── my-solana-program/        # Main Solana program
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs
└── libs/                         # Utility libraries
    ├── common/                   # Base common utilities
    │   ├── Cargo.toml
    │   └── src/
    │       └── lib.rs
    ├── math-utils/               # Math utilities (depends on common)
    │   ├── Cargo.toml
    │   └── src/
    │       └── lib.rs
    └── account-utils/            # Account utilities (depends on common)
        ├── Cargo.toml
        └── src/
            └── lib.rs
```

## Dependency Structure

The project demonstrates nested path dependencies:

```
my-solana-program
├── account-utils (path dependency)
│   └── common (path dependency)
└── math-utils (path dependency)
    └── common (path dependency)
```

### 1. Common Library (`libs/common`)
- **Purpose**: Base utilities shared across all libraries
- **Dependencies**: Only external crates (`solana-program`, `thiserror`)
- **Provides**: 
  - Common error types (`CommonError`)
  - Result type (`CommonResult<T>`)
  - Constants used across the project
  - Pubkey validation utilities
  - Basic validation functions

### 2. Math Utils Library (`libs/math-utils`)
- **Purpose**: Mathematical operations for token calculations
- **Dependencies**: `common` (path dependency)
- **Provides**:
  - Token amount calculations with decimals
  - Percentage and basis point calculations
  - Compound interest calculations
  - Safe arithmetic operations (overflow-safe)

### 3. Account Utils Library (`libs/account-utils`)
- **Purpose**: Account management and validation utilities
- **Dependencies**: `common` (path dependency), `anchor-lang`
- **Provides**:
  - PDA (Program Derived Address) creation with validation
  - Account validation utilities
  - Account data serialization/deserialization helpers
  - Account space validation

### 4. Main Solana Program (`programs/my-solana-program`)
- **Purpose**: The actual Solana program using the utility libraries
- **Dependencies**: `account-utils`, `math-utils` (both path dependencies)
- **Note**: Also gets `common` transitively through the other dependencies
- **Features**:
  - Program state initialization
  - User account creation
  - Token transfers with fee calculations

## Key Features Demonstrated

### Nested Path Dependencies
- The main program depends on `account-utils` and `math-utils`
- Both utility libraries depend on `common`
- This creates a dependency tree where `common` is shared transitively

### Shared Error Handling
- All libraries use the `CommonError` type from the `common` library
- Consistent error handling patterns across the entire project

### Code Reuse
- Constants like `DEFAULT_DECIMALS` defined in `common` are used in `math-utils`
- Validation functions from `common` are reused in `account-utils`
- The main program leverages functionality from both utility libraries

### Workspace Configuration
- Uses Cargo workspace to manage all crates together
- Shared dependency versions defined at the workspace level
- Consistent toolchain and feature configuration

## Building the Project

To build the entire workspace:

```bash
# Build all crates
cargo build

# Build specific crate
cargo build -p my-solana-program
cargo build -p common
cargo build -p math-utils
cargo build -p account-utils

# Check compilation without building
cargo check
```

**Note**: This example uses native Solana program structure (not Anchor) to avoid framework complexity and focus on demonstrating the nested path dependencies.

## Testing the Dependencies

You can test individual libraries:

```bash
# Test common utilities
cargo test -p common

# Test math utilities (includes common)
cargo test -p math-utils

# Test account utilities (includes common)
cargo test -p account-utils

# Test everything
cargo test
```

## Program Instructions

The main Solana program provides these instructions:

1. **Initialize**: Sets up the program state with initial token supply
2. **CreateUserAccount**: Creates a user account with initial balance
3. **TransferWithFee**: Transfers tokens between users with fee calculation

Each instruction demonstrates the use of multiple path dependencies:
- Math operations from `math-utils`
- Account validation from `account-utils`
- Error handling from `common` (transitively)

## Path Dependency Benefits

This structure provides:

1. **Modularity**: Each library has a specific responsibility
2. **Reusability**: Libraries can be used independently or together
3. **Maintainability**: Changes to common code affect all dependents
4. **Type Safety**: Shared types ensure consistency across libraries
5. **Performance**: No runtime overhead, all resolved at compile time

## Adding New Dependencies

To add a new library that depends on existing ones:

1. Create the new crate in `libs/`
2. Add it to the workspace `members` in the root `Cargo.toml`
3. Add path dependencies in the new crate's `Cargo.toml`:

```toml
[dependencies]
common = { path = "../common" }
math-utils = { path = "../math-utils" }  # Optional: if you need math functions
```

This example demonstrates best practices for organizing Rust Solana programs with complex dependency relationships. 