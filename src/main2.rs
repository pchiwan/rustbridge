fn add_five_to_string(s: String) ->
    Result<i32, std::num::ParseIntError> {
    let answer = s.parse::<i32>()? + 5;
    Ok(answer)
}

fn change_vec(v: &mut Vec<i32>) {
  v[0] *= 5;
}

fn main () {
  let numstr = "6";
  let num = numstr.parse::<i32>();
  println!("num = {:?}", num);
  let numstr = "florp";
  let num = numstr.parse::<i32>();
  println!("num = {:?}", num);

  let numstr = "6";
  let num = numstr.parse::<i32>();
  let num = num.expect("should have a number");
  println!("num + 5 = {}", num + 5);

  let numstr = "florp";
  let num = numstr.parse::<i32>();
  let answer = match num {
      Ok(n) => n + 5,
      Err(_) => 0,
  };
  println!("Answer is {}", answer);

  println!("{:?}", add_five_to_string(String::from("5")));
  println!("{:?}", add_five_to_string(String::from("hello")));

  let v = vec![1, 2, 3, 4, 5];
  let piece = &v[3..];
  println!("piece of v = {:?}", piece);

  let mut v = vec![1, 2, 3];
  change_vec(&mut v);
  change_vec(&mut v);
  println!("v is {:?}", v);
}
