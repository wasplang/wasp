(defn-wasm ^:export malloc [i32] [i32]
  ; this function creates an ever increasing memory allocation
  ; it writes the length in the first 4 bytes of the block
  ; followed by an zero byte to represent its free
  ; followed by the content
  [i32] ; let current_heap = 0
  ; current_heap = global.heap
  GLOBAL_GET  1
  LOCAL_SET   1
  ; memory[current_heap..current_heap+3] = length
  GLOBAL_GET  0
  LOCAL_GET   0
  I32_STORE   0 0
  ; global.heap = current_heap + 5 + length
  LOCAL_GET   1
  I32_CONST   5
  I32_ADD
  LOCAL_GET   0
  I32_ADD
  GLOBAL_SET  1
  ; return current_heap + 5
  LOCAL_GET   1
  I32_CONST   5
  I32_ADD
  END)

(def size 20)
(defn ^:export main []
  (malloc size))
