import rust_ds_lib

vec = rust_ds_lib.Vector()
vec.push(10)
vec.push(20)

print("Vector Length:", vec.len())  # Output: 2
print("First Element:", vec.get(0))  # Output: 10
print("Popped:", vec.pop())  # Output: 20
