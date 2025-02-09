# Tipo primitivos no Rust

Cada valor em Rust é de um determinado *tipo de dado*, que informa ao Rust que tipo de dado está sendo especificado, para que ele saiba como trabalhar com esses dados. Vamos analisar dois subconjuntos de tipos de dados: **escalares** e **compostos**.

## Tipos escalares

Um tipo escalar representa um único valor. Rust possui quatro tipos escalares primários: inteiros, ponto flutuante, booleanos e caracteres.

### Inteiros

Um número inteiro é um número sem parte fracionária. Rust suporta números inteiros

- com sinal (positivo ou negativo) e
- sem sinal (apenas positivo).

| Tipo  | Tamanho            | Faixa                                                                                                       |
| ----- | ------------------ | ----------------------------------------------------------------------------------------------------------- |
| i8    | 8-bit              | -128 to 127                                                                                                 |
| u8    | 8-bit              | 0 to 255                                                                                                    |
| i16   | 16-bit             | -32,768 to 32,767                                                                                           |
| u16   | 16-bit             | 0 to 65,535                                                                                                 |
| i32   | 32-bit             | -2,147,483,648 to 2,147,483,647                                                                             |
| u32   | 32-bit             | 0 to 4,294,967,295                                                                                          |
| i64   | 64-bit             | -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807                                                     |
| u64   | 64-bit             | 0 to 18,446,744,073,709,551,615                                                                             |
| i128  | 128-bit            | -170,141,183,460,469,231,731,687,303,715,884,105,728 to 170,141,183,460,469,231,731,687,303,715,884,105,727 |
| u128  | 128-bit            | 0 to 340,282,366,920,938,463,463,374,607,431,768,211,455                                                    |
| isize | depende do sistema | depende do sistema operacional (32-bit ou 64-bit)                                                           |
| usize | depende do sistema | depende do sistema operacional (32-bit ou 64-bit)                                                           |

### Ponto flutuante

Os números de ponto flutuante são representados de acordo com o padrão IEEE-754. Rust suporta dois tipos de ponto flutuante:

- `f32`: ponto flutuante de precisão simples
- `f64`: ponto flutuante de precisão dupla

### Booleanos

Os valores booleanos representam a lógica binária e podem ser `true` ou `false`.

### Caracteres

O tipo `char` é um tipo de dado de quatro bytes que representa um caractere Unicode.
