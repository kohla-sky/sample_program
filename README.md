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
    ├── crypto-primitives/        # Cryptographic utilities (depends on common) - ISOLATED
    │   ├── Cargo.toml
    │   └── src/
    │       └── lib.rs
    ├── math-primitives/          # Mathematical primitives (depends on common)
    │   ├── Cargo.toml
    │   └── src/
    │       └── lib.rs
    ├── math-utils/               # Math utilities (depends on common + math-primitives)
    │   ├── Cargo.toml
    │   └── src/
    │       └── lib.rs
    └── account-utils/            # Account utilities (depends on common + crypto-primitives)
        ├── Cargo.toml
        └── src/
            └── lib.rs
```

## Dependency Structure

The project demonstrates **nested path dependencies with depth-2 dependencies** including **isolated dependencies**:

```
my-solana-program
├── account-utils (path dependency)
│   ├── crypto-primitives (path dependency) ⬅️ ISOLATED DEPTH-2 DEPENDENCY
│   │   └── common (path dependency)
│   └── common (path dependency)
├── math-utils (path dependency)
│   ├── math-primitives (path dependency) ⬅️ SHARED DEPTH-2 DEPENDENCY
│   │   └── common (path dependency)
│   └── common (path dependency)
└── common (path dependency - direct)
```

**Key Features**: 
1. The main program does NOT directly import `math-primitives`, but gets it transitively through `math-utils`
2. **ISOLATED DEPENDENCY**: Only `account-utils` imports `crypto-primitives` - neither the main program nor other libraries use it directly
3. This demonstrates both **shared transitive dependencies** and **completely isolated dependencies** in the same project

### 1. Common Library (`libs/common`)
- **Purpose**: Base utilities shared across all libraries
- **Dependencies**: Only external crates (`solana-program`, `thiserror`)
- **Provides**: 
  - Common error types (`CommonError`)
  - Result type (`CommonResult<T>`)
  - Constants used across the project
  - Pubkey validation utilities
  - Basic validation functions

### 2. Crypto Primitives Library (`libs/crypto-primitives`) - **ISOLATED DEPENDENCY**
- **Purpose**: Cryptographic utilities specifically for account operations
- **Dependencies**: `common` (path dependency)
- **Used by**: **ONLY** `account-utils` (isolated dependency)
- **Provides**:
  - Cryptographic hashing utilities for account data
  - Deterministic seed generation for PDA creation
  - Address derivation utilities (vault addresses, metadata addresses)
  - Account validation using cryptographic proofs
  - Security utilities (entropy validation, security tokens)
- **Key Feature**: This library is NEVER directly imported by the main program or other libraries

### 3. Math Primitives Library (`libs/math-primitives`)
- **Purpose**: Low-level mathematical primitives and constants
- **Dependencies**: `common` (path dependency)
- **Provides**:
  - Mathematical constants (basis points, precision values)
  - Primitive operations (power of 10, square root, validation)
  - Number theory utilities (GCD, LCM, modular arithmetic)
  - Input validation for mathematical operations

### 4. Math Utils Library (`libs/math-utils`)
- **Purpose**: High-level mathematical operations for token calculations
- **Dependencies**: `common`, `math-primitives` (both path dependencies)
- **Provides**:
  - Token amount calculations with decimals (using math-primitives)
  - Percentage and basis point calculations
  - Compound interest calculations
  - Safe arithmetic operations (overflow-safe)
  - Advanced math operations (liquidity calculations, ratios)

### 5. Account Utils Library (`libs/account-utils`)
- **Purpose**: Account management and validation utilities
- **Dependencies**: `common`, `crypto-primitives` (both path dependencies)
- **Provides**:
  - PDA (Program Derived Address) creation with validation
  - Account validation utilities
  - Account data serialization/deserialization helpers
  - Account space validation
  - **Advanced cryptographic features** (using crypto-primitives):
    - Advanced user PDAs with crypto-generated seeds
    - Vault PDA creation using crypto address derivation
    - Account validation with cryptographic proofs
    - Security token generation for account operations

### 6. Main Solana Program (`programs/my-solana-program`)
- **Purpose**: The actual Solana program using the utility libraries
- **Dependencies**: `account-utils`, `math-utils`, `common` (all path dependencies)
- **Transitive Dependencies**: 
  - Gets `math-primitives` transitively through `math-utils` (shared depth-2 dependency)
  - Gets `crypto-primitives` transitively through `account-utils` (isolated depth-2 dependency) but NEVER directly imports it
- **Features**:
  - Program state initialization
  - User account creation with advanced crypto features
  - Token transfers with fee calculations

## Key Features Demonstrated

### Nested Path Dependencies (Including Depth-2)
- The main program depends on `account-utils`, `math-utils`, and `common`
- `math-utils` depends on both `common` and `math-primitives`
- `account-utils` depends on both `common` and `crypto-primitives`
- `math-primitives` and `crypto-primitives` depend on `common`
- This creates **two different depth-2 dependency chains**:
  1. **Shared**: `program → math-utils → math-primitives → common`
  2. **Isolated**: `account-utils → crypto-primitives → common` (only account-utils uses crypto-primitives)

### Isolated vs Shared Dependencies
- **Shared dependency**: `math-primitives` is used by `math-utils` and transitively by the main program
- **Isolated dependency**: `crypto-primitives` is ONLY used by `account-utils` - never directly by the main program or other libraries
- This demonstrates real-world patterns where some deep dependencies are shared across the codebase while others are isolated to specific modules

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