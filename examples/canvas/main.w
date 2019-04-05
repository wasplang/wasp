extern global_get_window()
extern window_get_document(window)
extern document_query_selector(document,query)
extern htmlcanvas_get_context(element,context)
extern canvas_set_fill_style(canvas,color)
extern canvas_fill_rect(canvas,x,y,w,h)

static colors = ("black","grey","red")

pub fn main(){
 let(   window      global_get_window()
        document    window_get_document(window)
        canvas      document_query_selector(document,"#screen")
        ctx         htmlcanvas_get_context(canvas,"2d")
        ){
     42
 }
}
// (pub defn main []
//   (let [window (global_get_window)
//         document (window_get_document window)
//         canvas (document_query_selector document "#screen")
//         ctx (htmlcanvas_get_context canvas "2d")]
//         (loop [x 0]
//                (if (< x 3)
//                    (do (canvas_set_fill_style ctx (mem_num (+ colors (* size_num x))))
//                        (canvas_fill_rect ctx (* x 10) (* x 10) 50 50 )
//                        (recur [x (+ x 1)]))))))
