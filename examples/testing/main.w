(extern console_log [message])

(defn is [c m] (if c None m))

(deftest multiplication
  (is (== 4 (* 2 2)) "2 * 2 should be 4")
  (is (== 1 (* 1 1)) "1 * 1 should be 2"))

(deftest addition
  (is (== 5 (+ 2 2)) "2 + 2 should be 4")
  (is (== 2 (+ 1 1)) "1 + 1 should be 2"))
