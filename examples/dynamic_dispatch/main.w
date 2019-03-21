(extern console_log [message])
(defn heads []
  (console_log "heads!"))
(defn tails []
  (console_log "tails!"))
(pub defn main [h]
  (call (fnsig [] i32) (if (== h 1) heads tails)))
