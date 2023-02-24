use std::io;

fn convert_to_int(data_input: & String) -> i32{
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main() {
    let mut sum = 0;
    let mut value_entrance = String::new();

    io::stdin().read_line(&mut value_entrance).expect("Erro na leitura do valor de entrada");
    let mut value_i32 = convert_to_int(&value_entrance);

    while value_i32 != 0 {
        let mut r = value_i32 % 10;
        sum = sum + r;
        value_i32 = value_i32 / 10;
    }

    print!("O valor das somas dos digitos é {}", sum);
}
