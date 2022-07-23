// Em Rust, existem dois tipos de comentários:
// Os ignorados pelo compilador
// Eles são seguidos de // para comentários em linha, que serão validos até o final da linha
/* E os comentários em bloco.
Estes, teram o começo e fim dos comentários definidos pelo delimitador
de começo e fim. */

// E os que são gerados pelo cargo doc
// O primeiro é determinado por ter ///, e é gerado para o seguinte item.
// O segundo é //!, e é gerado para um item anexado.

fn main() {
    // Este é um exemplo de comentário de linha
    // Existem duas barras no início da linha
    // E nada escrito dentro deles será lido pelo compilador

    // println!("Olá mundo!");
    
    // Execute-o. Vejo? Agora tente excluir as duas barras e execute-as novamente.

    /*
    * Este é outro tipo de comentário, um comentário em bloco. Em geral,
    * comentários de linha são o estilo de comentário recomendado. Mas
    * comentários em bloco são extremamente úteis para desativar temporariamente
    * pedaços de código. Os comentários em bloco podem ser aninhados, * / * /
    *, portanto, são necessárias apenas algumas teclas para comentar tudo
    * nesta função principal (). / * / * / * Tente você mesmo!
    */

    /*
    Nota: A coluna anterior de `*` era inteiramente para estilo. Há sim
    nenhuma necessidade real disso.
    */

    // Você pode manipular expressões mais facilmente com comentários em bloco
    // do que com comentários de linha. Tente excluir os delimitadores de comentários
    // para alterar o resultado:

    let x = 5 /*+ 90 */+ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}
// Os comentários dentro do Main foram copidos.