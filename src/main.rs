fn main() {
    let input = "+ 1 1 |> lambda x -> * x 2";
    let tokens: Vec<&str> = input.split_whitespace().collect();
    println!("{:?}", tokens);
}
