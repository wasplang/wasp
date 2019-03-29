(extern console_log [message])
(defn heads []
  (console_log "heads!"))
(defn tails []
  (console_log "tails!"))
(pub defn main [h]
  (call (fnsig [] f64) (if (== h 1) heads tails)))
