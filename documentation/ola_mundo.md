# Olá Mundo em LBR

Esse é o inicio da linguagem, provavelmente o seu primeiro programa nela. E como sua jornada programando, existem várias formas de começar. Aqui estão alguns exemplos:

```lbr
constante inicio = fn {
    escreval("Olá, Mundo!");
};
```

Esse mesmo código pode ser escrito desta forma:

```lbr
inicio := fn {
    escreval("Olá, Mundo!");
}
```

A sintaxe `:=` é só uma forma mais consisa de escrever o bloco original.

## `escreva` e `escreval`

Quando escrevendo LBR, você pode usar tanto a função `escreva` para escrever no `stdout`, ou usar `escreval` para fazer a mesma coisa e depois pular uma linha.

Se você quiser pular uma linha manualmente, você pode usar a sequência de escape `\n` para pular uma linha.

### Avisos:
 - Tanto `escreva` e `escreval` fazem um flush após escrever. Se você não quiser esse comportamento, você pode usar `escreva_nf` e `escreval_nf` para evitar esse comportamento.
 - 

# Olá Você

Vamos agora ver um programa que peguntará qual o nome do usuário e escreverá um olá para ele.

```lbr
inicio := fn {
    escreva("Qual o seu nome? ");
    val nome: texto = leial();
    escreva("Olá, ${nome}");
};
```

Aqui temos várias informações! Vamos uma por vez:

## Valores e variáveis

Em LBR, nós temos várias formas de guardar um valor. São eles: `val`, `var` e `const`.

### `val` vs `var`

A principal diferença entre uma variável declarada com `val` ou `var` é mutabilidade. qualquer variável declarada com `val` não pode ser modificado após a sua criação. Já variáveis declarados com `var`, podem ser modificadas a qualquer momento desde que ela se mantenha do mesmo tipo.
