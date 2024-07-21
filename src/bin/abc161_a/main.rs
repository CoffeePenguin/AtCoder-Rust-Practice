use proconio::input;

fn main(){
    input! {
        mut x:i32,
        mut y:i32,
        mut z:i32
    }

    //std::mem::swap(&mut x, &mut y);
    let mut tmp:i32 = y;
    y = x;
    x = tmp;
    
    //std::mem::swap(&mut x, &mut z);
    tmp = z;
    z = x;
    x = tmp;
    

    println! ("{} {} {}",x,y,z);

}