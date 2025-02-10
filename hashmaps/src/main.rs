use std::collections::HashMap;

fn main() {
    let mut map: HashMap<&str, &str> = HashMap::new();

    // Inserir valor
    map.insert("chave1", "valor1");
    map.insert("chave2", "valor2");
    println!("{:?}", map);

    map.insert("chave1", "novo valor1");
    println!("{:?}", map);

    let v = map.insert("chave3", "valor3");
    println!("{:?}", v);

    let v = map.insert("chave3", "novo valor de 3");
    println!("{:?}", v);

    // Acessar valores
    println!("{:?}", map.get("chave1"));
    println!("{:?}", map.get("chave10"));

    // Remover valores
    println!("{:?}", map);
    map.remove("chave1");
    println!("{:?}", map);

    // Iterar
    for (key, value) in &map {
        println!("{} - {}", key, value);
    }

    // Verificar se uma chave existe
    println!("{}", map.contains_key("chave10"));
    println!("{}", map.contains_key("chave3"));

    // Verificar tamanho
    println!("{}", map.len());

    // Verifica se está vazio
    println!("{}", map.is_empty());

    // Limpar
    map.clear();
    println!("{:?}", map);
    println!("{}", map.is_empty());
}
