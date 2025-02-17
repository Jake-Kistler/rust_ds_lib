import rust_ds_lib

vec = rust_ds_lib.RustVector()
vec.push(10)
vec.push(20)

print("Length:", vec.len())  # Output: 2
print("First Element:", vec.get(0))  # Output: 10
print("Popped:", vec.pop())  # Output: 20
print("Length after pop:", vec.len())  # Output: 1
