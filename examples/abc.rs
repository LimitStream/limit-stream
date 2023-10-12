pub enum E0{
  Tsum(sum),
  TNext<Recv<Done>, Next<Send<Int>, Endpoint>>(Next<Recv<Done>, Next<Send<Int>, Endpoint>>),
}
pub struct User{
  name: String,
  age: Uint,
  description: String,
}

enum SB{
  is_sb(User) = 1,
  is_not_sb(User) = 0,
}

pub struct User{
  name: String,
  age: Uint,
  description: String,
}

pub type sum = Next<Recv<Int>, Offer<E0>>;
