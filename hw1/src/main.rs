/*
функция double_int32 принимает 32-х битное целое знаковое число
и возвращает 32-х битное целое знаковое число, равное удвоенному входному.

функция double_int64 принимает 32-х битное целое знаковое число
и возвращает 64-х битное целое знаковое число, равное удвоенному входному.

функция double_float32 принимает 32-х битное число с плавающей точкой
и возвращает 32-х битное число с плавающей точкой, равное удвоенному входному.

функция double_float64 принимает 32-х битное число с плавающей точкой
и возвращает 64-х битное число с плавающей точкой, равное удвоенному входному.

функция int_plus_float_to_float принимает 32-х битное целое знаковое число и 32-х битное число с плавающей точкой.
Возвращает 64-х битное число с плавающей точкой, равное сумме входных.

функция int_plus_float_to_int принимает 32-х битное целое знаковое число и 32-х битное число с плавающей точкой.
Возвращает 64-х битное целое знаковое число, равное сумме входных.

функция tuple_sum принимает кортеж из двух целых чисел. Возвращает целое число, равное сумме чисел во входном кортеже.

функция array_sum принимает массив из трёх целых чисел. Возвращает целое число, равное сумме чисел во входном массиве
*/

fn main() {
    misc();

    print!("\n\n");
    println!("double_int32: {}", double_int32(5i32));
    println!("double_int64: {}", double_int64(2_000_000_000i32));

    println!("double_float32: {}", double_float32(5f32));
    println!("double_float64: {}", double_float64(f32::MAX - 1.0));

    println!("int_plus_float_to_float: {}", int_plus_float_to_float(5i32, 6f32));
    println!("int_plus_float_to_int: {}", int_plus_float_to_int(5i32, 6f32));

    println!("tuple_sum: {}", tuple_sum((5, 6)));
    println!("array_sum: {}", array_sum([1, 2, 4]));
}

fn double_int32(x: i32) -> i32 {
    x * 2
}

fn double_int64(x: i32) -> i64 {
    (x as i64) * 2
}

fn double_float32(x: f32) -> f32 {
    x * 2.0
}

fn double_float64(x: f32) -> f64 {
    (x as f64) * 2.0
}

fn int_plus_float_to_float(x: i32, y: f32) -> f64 {
    (x as f64) + (y as f64)
}

fn int_plus_float_to_int(x: i32, y: f32) -> i64 {
    (x as i64) + (y as i64)
}

fn tuple_sum(t: (i32, i32)) -> i32 {
    t.0 + t.1
}

fn array_sum(arr: [i32; 3]) -> i64 {
    (arr[0] + arr[1] + arr[2]) as i64
}

fn misc() {
    println!("i32::MAX = {}", i32::MAX);
    println!("i32::MIN = {}", i32::MIN);

    println!("f32::MIN = {}", f32::MIN);
    println!("f32::MAX = {}", f32::MAX);

    println!("f64::MIN = {}", f64::MIN);
    println!("f64::MAX = {}", f64::MAX);
}
