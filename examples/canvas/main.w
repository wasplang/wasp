(extern global_getWindow [])
(extern Window_get_document [window])
(extern Document_querySelector [document query])
(extern HTMLCanvasElement_getContext [element context])
(extern CanvasRenderingContext2D_set_fillStyle [canvas color])
(extern CanvasRenderingContext2D_fillRect [canvas x y w h])

(def colors ("black" "grey" "red"))

(pub defn main []
  (let [window (global_getWindow)
        document (Window_get_document window)
        canvas (Document_querySelector document "#screen")
        ctx (HTMLCanvasElement_getContext canvas "2d")]
        (loop [x 0]
               (if (< x 3)
                   (do (CanvasRenderingContext2D_set_fillStyle ctx (mem32 (+ colors (* 4 x))))
                       (CanvasRenderingContext2D_fillRect ctx (* x 10) (* x 10) 50 50 )
                       (recur [x (+ x 1)]))))))
