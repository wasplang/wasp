(extern console_log [message])
(def success 0)
(defn is [c m] (if c success m))

(deftest multiplication
  (is (== 4 (* 2 2)) "2 * 2 should be 4")
  (is (== 1 (* 1 1)) "1 * 1 should be 2"))

(deftest addition
  (is (== 5 (+ 2 2)) "2 + 2 should be 4")
  (is (== 2 (+ 1 1)) "1 + 1 should be 2"))
