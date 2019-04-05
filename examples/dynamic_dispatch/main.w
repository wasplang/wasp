extern console_log(message)
fn heads(){ console_log("heads!") }
fn tails(){ console_log("tails!") }
pub fn main(h){
    function_to_call = if((h==1)){heads}else{tails}
    call(fn()->f64,function_to_call)
}
