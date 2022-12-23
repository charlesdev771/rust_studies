fn main()
{

    //let msg = "Hello";
    //println!(msg);


    //let posi :u16 = 132; // usize, U: positives. i positives and negatives

    const MYBOOL: bool = false;
          //char, &str, u32, f65, usize
    println!("{}", MYBOOL);

    let tuplee :(&str, i32) = ("aa", 12);
    //let m :[i32, 1] = [1,2]

    println!("{}, {}", tuplee.0, tuplee.1);
   // println!("{}", m.1);

   let s :u32 = 1000;

   if s == 1000
   {
    println!("aaaaaaaaaaaaaa");
   }
   else
   {
    println!("bbbbbbbbbbbbbbb");
   }


for x in 0..5
{
    println!("{}", x);
}

let v :i32 = 10;

loop{
    println!("{}", v);
}


let ver :u32 = 10;

println!("{}", q(ver))/


}

fn q(x:i32) -> u32{
    return x*x;
}