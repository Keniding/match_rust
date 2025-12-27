fn main() {
    /*
    Rust cuenta con una construcción de flujo de control extremadamente potente llamada [texto incoherente] match que permite comparar un valor con una serie de patrones y luego ejecutar código según el patrón que coincida. Los patrones pueden estar compuestos por valores literales, nombres de variables, comodines y muchos otros elementos. El capítulo 19 abarca todos los tipos de patrones y sus funciones. Su potencia match reside en la expresividad de los patrones y en que el compilador confirma que se manejan todos los casos posibles.
    Piense en una match expresión como una máquina clasificadora de monedas: las monedas se deslizan por una pista con agujeros de diferentes tamaños, y cada moneda cae por el primer agujero que encuentra en el que encaja. ¡De la misma manera, los valores pasan por cada patrón en un [nombre de la función] match, y en el primer patrón en el que el valor "encaja", este cae en el bloque de código asociado para su uso durante la ejecución.
    Hablando de monedas, tomémoslas como ejemplo usando match!. Podemos escribir una función que tome una moneda estadounidense desconocida y, de forma similar a la máquina contadora, determine qué moneda es y devuelva su valor en centavos, como se muestra en el Listado 6-3.
     */

    // Ejemplo 1: Todas las monedas simples
    println!("=== Ejemplo 1: Monedas simples ===");
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter;

    println!("Penny value: {} cents", value_in_cents(penny));
    println!("Nickel value: {} cents", value_in_cents(nickel));
    println!("Dime value: {} cents", value_in_cents(dime));
    println!("Quarter value: {} cents", value_in_cents(quarter));

    // Ejemplo 2: Función con println!
    println!("\n=== Ejemplo 2: Lucky penny ===");
    let lucky_penny = Coin::Penny;
    value_in_cents_better(lucky_penny);

    // Ejemplo 3: Monedas con estado
    println!("\n=== Ejemplo 3: Monedas con estado ===");
    let alaska_quarter = CoinWithState::Quarter(UsState::Alaska);
    let alabama_quarter = CoinWithState::Quarter(UsState::Alabama);
    let penny_with_state = CoinWithState::Penny;
    let nickel_with_state = CoinWithState::Nickel;
    let dime_with_state = CoinWithState::Dime;

    println!("Quarter value: {} cents", value_in_cents_with_state(alaska_quarter));
    println!("Quarter value: {} cents", value_in_cents_with_state(alabama_quarter));
    println!("Penny value: {} cents", value_in_cents_with_state(penny_with_state));
    print!("Nickel value: {} cents", value_in_cents_with_state(nickel_with_state));
    println!("Nickel value: {} cents", value_in_cents_with_state(dime_with_state));

    // Ejemplo 4: Option con match
    println!("\n=== Ejemplo 4: Option con match ===");
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}, {:?}, {:?}", five, six, none);
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

/*
Analicemos la match función value_in_cents. Primero, listamos la match palabra clave seguida de una expresión, que en este caso es el valor coin. Esto parece muy similar a una expresión condicional usada con if, pero hay una gran diferencia: con if, la condición debe evaluarse como un valor booleano, pero aquí puede ser de cualquier tipo. El tipo de coin en este ejemplo es la Coin enumeración que definimos en la primera línea.
A continuación están los match brazos. Un brazo consta de dos partes: un patrón y código. El primer brazo tiene un patrón que representa el valor Coin::Penny y luego el => operador que separa el patrón del código a ejecutar. En este caso, el código es simplemente el valor 1. Cada brazo se separa del siguiente con una coma.
Al match ejecutarse la expresión, compara el valor resultante con el patrón de cada brazo, en orden. Si un patrón coincide con el valor, se ejecuta el código asociado a ese patrón. Si ese patrón no coincide con el valor, la ejecución continúa con el siguiente brazo, como en una máquina clasificadora de monedas. Podemos tener tantos brazos como necesitemos: en el Listado 6-3, nuestro match tiene cuatro brazos.
El código asociado con cada brazo es una expresión, y el valor resultante de la expresión en el brazo coincidente es el valor que se devuelve para la match expresión completa.
Normalmente no usamos llaves si el código del brazo coincidente es corto, como en el Listado 6-3, donde cada brazo simplemente devuelve un valor. Si desea ejecutar varias líneas de código en un brazo coincidente, debe usar llaves, y la coma después del brazo es opcional. Por ejemplo, el siguiente código imprime "¡Centavo de la suerte!", cada vez que se llama al method con un Coin::Penny, pero sigue devolviendo el último valor del bloque 1:
 */
fn value_in_cents_better(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

/*
Patrones que se vinculan a los valores
Otra característica útil de los brazos de coincidencia es que pueden vincularse a las partes de los valores que coinciden con el patrón. Así es como podemos extraer valores de las variantes de enumeración.
Por ejemplo, modifiquemos una de nuestras variantes de enumeración para que contenga datos. Entre 1999 y 2008, Estados Unidos acuñó monedas de 25 centavos con diseños diferentes para cada uno de los 50 estados en una cara. Ninguna otra moneda tenía diseños de los estados, por lo que solo las monedas de 25 centavos tienen este valor adicional. Podemos añadir esta información enum modificando la Quarter variante para que incluya un UsState valor almacenado
 */
/*
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
 */

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[derive(Debug)]
enum CoinWithState {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents_with_state(coin: CoinWithState) -> u8 {
    match coin {
        CoinWithState::Penny => 1,
        CoinWithState::Nickel => 5,
        CoinWithState::Dime => 10,
        CoinWithState::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}