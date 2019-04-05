extern console_log(message)
fn heads(){ console_log("heads!") }
fn tails(){ console_log("tails!") }
pub fn main(h){
  call((fnsig [] f64),if(==(h,1),heads,tails))
}
