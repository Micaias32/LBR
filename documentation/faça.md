# `crie` a ferramenta para compilação simples de lbr

`crie` é uma ferramenta para compilação dor arquivos `.lbr` onde a única coisa
necessária é escrever `crie <nome do arquivo sem extensão>` e o programa será feito.

## Exemplos

Um uso simples da ferramenta

```
$ ls
ola.lbr teste.lbr
$ crie ola
$ ls
ola* ola.lbr teste.lbr
```

Arquivo de configuração
```
$ ls
ola.lbr crie.jsonc
$ cat crie.jsonc
{
    parâmetros: ["-O3", "--Avisos=todos"],
    alvo: ["x86_64-linux"],
    bibliotecas: [{
        nome: "ray-lbr",
        ver: "0.1.0",
    }],
}
```
Se nenhum arquivo de confuguração for encontrado, a ferramenta utilizará
a configuração do sistema.


