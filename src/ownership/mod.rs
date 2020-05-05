/*
 * - Ownership enables the rust to make memory safety without garbage collectors.
 * - Rust provides stack and heap memory for code at runtime.
 *
 * Stack - lifo
 *  - push - adding data to the stack
 *  - pop - removing data from the stack
 *  - has fixed size
 *  - scalar types can be stored in stack
 *
 * Heap -
 *  - size not known at compile or size can grows or shrink
 *  - eg. String
 *  - less organised
 *  - OS memory is used for heap and marked as being in use
 *  - returns the pointer(address of that location) of the memory used
 *  - allocating memory on heap is called - allocating
 *  - pointer to heap is stored in stack
 *
 */

// mod ownershiprules;
// mod references;
mod borrowing;

pub fn run() {
    // ownershiprules::run();
    // references::run();
    borrowing::run();
}
