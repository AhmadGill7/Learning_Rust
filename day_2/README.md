# Learning_Rust_Day_2

## Memory Management and Ownership Concepts

Today's learning covered the following topics:

### Memory Management
- Understanding Stack vs Heap memory
- Stack: Storage for static-sized values (integers, booleans)
- Heap: Storage for dynamic-sized values (Strings, Vectors)
- Memory allocation and deallocation concepts

### Ownership Rules
- Core ownership principles:
  1. Each value has exactly one owner
  2. Only one owner at a time
  3. Value is dropped when owner goes out of scope
- Ownership transfer (moving)
- Value copying vs moving
- Function ownership examples

### Borrowing
- Reference borrowing (`&`)
- Mutable borrowing (`&mut`)
- Borrowing rules:
  - Multiple immutable borrows allowed
  - Only one mutable borrow at a time
  - Cannot have both mutable and immutable borrows simultaneously
- Function borrowing examples

### Code Examples Covered
- Stack and heap memory demonstration
- String capacity and memory allocation
- Ownership transfer scenarios
- Borrowing and reference manipulation
- Mutable reference constraints
