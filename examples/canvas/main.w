extern console_log(msg)
extern global_get_window()
extern window_get_document(window)
extern document_query_selector(document,query)
extern htmlcanvas_get_context(element,context)
extern canvas_set_fill_style(canvas,color)
extern canvas_fill_rect(canvas,x,y,w,h)

static colors = ("black","grey","red")

pub fn main(){
    // setup a drawing context
    window = global_get_window()
    document = window_get_document(window)
    canvas = document_query_selector(document,"#screen")
    ctx = htmlcanvas_get_context(canvas,"2d")
    x = 0
    loop {
        // get the offset for the color to use
        color_offset = (colors + (x * size_num))
        // set current color to string at that position
        canvas_set_fill_style(ctx,mem_num(color_offset))
        // draw the rect
        canvas_fill_rect(ctx,(x * 10),(x * 10),50,50)
        // recur until 3 squares are drawn
        x = (x + 1)
        if (x < 3) {
            recur
        } else {
            0
        }
    }
}
