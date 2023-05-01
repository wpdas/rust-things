// funcao reverse recebe Tupla (construida usando parenteses) passando o tipo de dado de cada casa
// retorna uma Tupla com as casas invertidas
fn reverse(pair: (i32, bool)) -> (bool, i32) {
  // `let` can be used to bind the members of a tuple to variables. (semelhate ao desestruction do JavaScript)
  let (int_param, bool_param) = pair;
  
  // retorno
  (bool_param, int_param)
}

// É obrigatório ter o main()
fn main() {
  let pair = (32, true);
  println!("Pair is {:?}", pair);
  
  // é necessário o uso do "pretty-print"´[{:?}] para exibir esse tipo de dado
  println!("The reversed pair is {:?}", reverse(pair));
}