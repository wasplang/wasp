(extern console_log [message])
(defn hashset [a b c]
  (cons a (cons b c)))
(defn ^:export main [msg]
  (console_log (str_join (# hashset :a :string :red :int ) "-")))
