/*
Tipos Inteiros Unsigned (u8, u16, u32, u64, u128)

    Código: Declare uma variável positive_only do tipo u32 com o valor 100. Declare outra variável can_be_negative do tipo i32 com o valor -100. Imprima ambas. Agora, tente atribuir -5 à variável positive_only. O que acontece ao tentar compilar e por quê?
    Código: Escreva um código Rust para declarar uma variável max_u8 do tipo u8 e atribua a ela o maior valor possível para este tipo. Imprima esse valor. Em seguida, no seu código, tente atribuir max_u8 + 1 a uma nova variável u8. O que acontece durante a compilação ou execução (especialmente em modo de debug vs release, se souber)?
    Código: Declare uma variável chamada world_population do tipo u64 e atribua a ela o valor 7800000000. Em seguida, imprima o valor da variável formatado com separadores de milhar (pesquise como fazer isso se necessário, ou apenas imprima o número).
    Código: Suponha que você está modelando um sistema que usa IDs únicos extremamente grandes que nunca serão negativos. Declare uma variável super_large_id do tipo u128 e atribua a ela o valor 250000000000000000000000000000000000000 (25 seguido de 36 zeros). Imprima esta variável.
    Código - Depuração: O código abaixo causa um erro. Identifique o erro, explique por que ele ocorre e corrija o código para que funcione, mantendo a intenção de usar um tipo pequeno se o valor permitir.
    Rust

    // fn main() {
    let small_number: u8 = 260;
    println!("{}", small_number);
    // }

    Código: Você precisa armazenar o número de visualizações de um vídeo, que pode chegar a bilhões, mas nunca será negativo. Escolha o tipo u mais apropriado (u32 ou u64). Declare uma variável video_views com este tipo, atribua 2500000000 e imprima.

Tipos Inteiros Signed (i8, i16, i32, i64, i128)

    Código: Declare duas variáveis, min_val_i8 e max_val_i8, ambas do tipo i8. Atribua a elas o menor e o maior valor possível, respectivamente, para o tipo i8. Imprima ambos os valores.
    Código: Declare uma variável chamada current_altitude_change do tipo i16 para representar uma mudança de altitude em metros. Atribua a ela o valor -350 (descida de 350 metros). Imprima este valor.
    Código: Você está processando transações financeiras onde os valores podem ser muito grandes e representar tanto créditos quanto débitos (em centavos, para evitar problemas com ponto flutuante). Declare uma variável transaction_value_cents do tipo i64 e atribua a ela um valor como -12345678900 (representando um débito de mais de 123 milhões). Imprima-a.
    Código - Depuração: O código abaixo tenta atribuir um valor que excede o limite de i32. Corrija o código escolhendo um tipo i maior que possa acomodar o valor 2_200_000_000.
    Rust

    // fn main() {
    let large_positive: i32 = 2_200_000_000; // Excede o limite de i32
    println!("{}", large_positive);
    // }

    Código: Declare uma variável value_a do tipo i8 com 120. Declare value_b do tipo u8 com 120. Agora, tente declarar value_c do tipo i8 com -10 e value_d do tipo u8 tentando atribuir -10 (literalmente). O que acontece com value_d durante a compilação? Explique.
    Código: Escreva um trecho de código que declare duas variáveis score_team_a e score_team_b (ambas i32). Atribua valores a elas e calcule a score_difference (podendo ser negativa). Imprima a diferença.

Tipos de Ponto Flutuante (f32, f64)

    Código: Declare uma variável ratio_f32 do tipo f32 com o valor 2.0 / 7.0. Declare outra variável ratio_f64 do tipo f64 com o mesmo valor 2.0 / 7.0. Imprima ambas usando println!("{:.18}", variable_name); para mostrar várias casas decimais. Compare os resultados impressos.
    Código: Declare uma variável precise_measurement do tipo f64 com o valor 123.456789123456. Imprima-a.
    Código: Declare uma variável gravity_force com o valor 9.80665 sem especificar o tipo, deixando o Rust inferir. Em seguida, use std::any::type_name_of_val(&gravity_force) para imprimir o tipo inferido. O que é impresso e por quê?
    Código: Crie uma variável result_f64 do tipo f64 e atribua a ela a expressão 0.1 + 0.2. Imprima o resultado com pelo menos 17 casas decimais. O resultado é exatamente 0.3? Escreva um pequeno trecho de código para verificar se result_f64 == 0.3 e imprima "Igual" ou "Diferente".
    Código: Em um sistema embarcado com memória limitada, você precisa armazenar a leitura de um sensor que varia de -10.0 a +10.0 com duas casas decimais de precisão. Declare uma variável sensor_reading usando f32 e atribua 7.89. Imprima-a. Justifique por que f32 pode ser adequado aqui.

Caracteres (char)

    Código: Declare um caractere char_ascii = 'Z'; e um caractere Unicode char_unicode = 'Ω'; (letra grega Ômega). Use std::mem::size_of_val(&char_ascii) e std::mem::size_of_val(&char_unicode) para imprimir o tamanho em bytes de cada um. O que você observa?
    Código: Declare uma variável my_initial do tipo char e atribua a ela a primeira letra do seu nome. Imprima-a.
    Código: Declare três variáveis char: emoji_char com '😊', math_symbol com '∑', e arrow_char com '→'. Imprima todas elas.
    Código: Tente declarar uma variável char com mais de um caractere, por exemplo: let not_a_char: char = 'ab';. O que acontece quando você tenta compilar?

Booleanos (bool)

    Código: Declare uma variável is_file_loaded e atribua a ela true. Declare has_errors e atribua false. Imprima ambas. Em seguida, tente atribuir o inteiro 1 a uma variável booleana. O que o compilador Rust diz?
    Código: Declare uma variável user_is_premium do tipo bool. Use um if para imprimir "Acesso total liberado!" se for true, ou "Considere virar premium!" se for false. Teste com ambos os valores.
    Código: Escreva uma função simples can_vote(age: u8, is_citizen: bool) -> bool que retorna true se age for 18 ou mais E is_citizen for true. Chame-a com alguns valores e imprima os resultados.
    Código: Declare is_online = true e has_new_messages = false. Crie uma variável should_notify que seja true se is_online E has_new_messages forem ambos true. Imprima should_notify. Depois, mude has_new_messages para true e recalcule/imprima should_notify.

Tuplas

    Código: Crie uma tupla record que contenha um nome (&str), uma idade (u8), e uma nota de aprovação (f32). Por exemplo: ("Maria", 22, 7.5). Imprima a tupla inteira usando {:?}.
    Código: Crie uma tupla chamada server_response que contenha um código de status HTTP (u16) e uma mensagem de resposta (&str), como (404, "Not Found"). Imprima a tupla.
    Código: Dada a tupla let product_info = ("Laptop XPTO", 1250.99, 15); (nome, preço, quantidade em estoque), acesse e imprima o preço do produto e a quantidade em estoque usando a indexação de tupla (ex: product_info.1).
    Código: Dada a tupla let color_rgb = (255, 128, 0); (representando Laranja), desestruture-a nas variáveis red, green, e blue. Imprima cada variável separadamente.
    Código: Crie uma tupla complex_data = ('X', vec![1,2,3], ("nested", true));. Imprima-a. O que isso demonstra sobre os tipos que uma tupla pode conter?
    Código: Declare uma tupla api_result com um booleano indicando sucesso, um u64 para um ID, e um String para uma mensagem. Ex: (true, 1234567890, String::from("Operação bem-sucedida")). Imprima usando {:#?}.

Tuplas e Mutabilidade (mut)

    Código - Experimento: Declare uma tupla let config = ("localhost", 8080);. Tente modificar o segundo elemento para 8081 (ex: config.1 = 8081;). O que acontece ao compilar? Agora, redeclare-a como let mut config = ("localhost", 8080); e tente a mesma modificação. Imprima config.
    Código: Declare uma tupla mutável player_stats para armazenar nome (&str), pontuação (i32), e vidas (u8). Inicialize com ("Hero", 0, 3). Em seguida, modifique a pontuação para 1500 e as vidas para 2. Imprima a tupla atualizada.
    Código: Crie uma tupla mutável let mut point = (10.0, 20.0);. Modifique o primeiro elemento para 15.5 e o segundo para 25.0 usando a sintaxe de acesso por índice. Imprima a tupla.
    Código: Crie uma tupla mutável file_details contendo nome do arquivo (String), tamanho (u64) e se é editável (bool). Inicialize-a. Depois, modifique o nome do arquivo (anexando "_v2" ao nome original) e altere o status de editável. Imprima a tupla.

Arrays

    Código: Crie um array temperatures que armazene as seguintes leituras de temperatura f32: [22.5, 23.1, 21.9, 22.8, 23.5]. Imprima o array inteiro. Tente adicionar um &str a este array. O que acontece?
    Código: Declare um array months contendo os nomes dos primeiros três meses do ano como string slices. Imprima o nome do segundo mês (lembre-se da indexação baseada em zero).
    Código - Experimento: Tente criar um array let mixed_data = [1, "dois", 3.0];. O que o compilador Rust informa? Como isso difere de uma tupla com elementos de tipos diferentes?
    Código - Depuração: O código a seguir tem um erro relacionado ao tamanho do array e aos inicializadores. Corrija-o de duas maneiras diferentes (uma ajustando o tamanho, outra ajustando os inicializadores) e mostre ambas as soluções.
    Rust

    // fn main() {
    // Erro original:
    // let some_numbers: [i32; 4] = [10, 20, 30];
    // println!("{:?}", some_numbers);
    // }

    Código: Declare um array powers_of_two de 6 elementos do tipo u32. Inicialize-o com os valores [1, 2, 4, 8, 16, 32]. Imprima o último elemento do array usando indexação.
    Código: Declare um array grades com 5 notas u8 inicializadas todas com o valor 0 usando a sintaxe de repetição (ex: [0; 5]). Imprima o array e seu tamanho usando .len().

Arrays e Mutabilidade (mut)

    Código - Experimento: Declare um array let fixed_scores = [100, 90, 80];. Tente modificar o segundo elemento para 95 (ex: fixed_scores[1] = 95;). O que acontece? Agora, declare-o como let mut fixed_scores = [100, 90, 80];, faça a modificação e imprima.
    Código: Declare um array mutável inventory_counts de 4 elementos do tipo u16, inicializado com [10, 25, 5, 30]. Modifique a contagem do primeiro item para 12 e do terceiro para 8. Imprima o array atualizado.
    Código: Crie um array mutável active_services: [bool; 3] inicializado como [true, false, true]. Modifique o segundo serviço para true e o último para false. Imprima o array.
    Código - Desafio: Dado o array let mut values: [i32; 6] = [5, -2, 10, -8, 0, 3];, escreva um loop for que itere sobre este array e, para cada elemento, se ele for negativo, substitua-o pelo seu valor absoluto (ex: -2 vira 2, -8 vira 8). Imprima o array modificado.
    Código: Crie um array mutável pixel_colors: [[u8; 3]; 2] para representar dois pixels, cada um com componentes R, G, B. Inicialize-o como [[255, 0, 0], [0, 255, 0]] (um pixel vermelho, um verde). Modifique o primeiro pixel para ser azul ([0, 0, 255]) e o segundo para ser amarelo ([255, 255, 0]). Imprima o array de pixels.
*/
