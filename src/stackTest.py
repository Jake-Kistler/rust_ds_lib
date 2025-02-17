import rust_ds_lib

# Create a new stack
stack = rust_ds_lib.Stack()
print("Is stack empty?", stack.is_empty())  # Should be True

# Push elements
stack.push(10)
stack.push(20)
stack.push(30)

print("Stack length:", stack.len())  # Output: 3
print("Peek top element:", stack.peek())  # Output: 30

# Pop elements
print("Popped:", stack.pop())  # Output: 30
print("Popped:", stack.pop())  # Output: 20

print("Stack length after popping:", stack.len())  # Output: 1
print("Is stack empty now?", stack.is_empty())  # Should be False
