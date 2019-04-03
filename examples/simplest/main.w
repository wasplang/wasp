; (defstruct point
;   :x "json:hey"  ; x is a position on x dimension
;   :y             ; y is a position on y dimension
;   )
;
; (defn new [x] 0)
; (defn set [x y v] 0)
;
; (pub defn main []
;   (let [foo (new point)]
;     (set foo :x 1)
;     (set foo :y 1)))
(pub defn main[] :x)
