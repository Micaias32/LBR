# Programação orientada a objetos em LBR

O conceito de Programação Orientada a Objetos é muito famoso e existe também em LBR,
com algumas mudanças, que podem te ajudar a escrever código mais seguro e eficiente.

Exemplo:
```lbr
const Pessoa = classe {
    nome: Texto;
    cpf: Texto;
    idade: u32;
    altura: f32;
    mãe: &Pessoa;

    comprimentar := fn (&obj, comprimentado: Texto) {
        escreval(f"{obj.nome} diz olá para {comprimentado}");
    };

    construtor(nome: Texto, cpf: Texto, idade: u32, altura: f32) {
        retorne Pessoa {
            nome: nome,
            cpf: cpf,
            idade: idade,
            altura: altura,
        };
    }
};

inicio := função (args: [Texto]) {
    val pessoa = Pessoa("Micaias", "000.000.000-00", 16, 1.70);
    pessoa.comprimentar("Felipe");
    se senão 
};
```

Por padrão qualquer classe criará automáticamente um construtor padrão. Esse
construtor dará valores padrão para cada atributo. Se o atributo também for um objeto,
o objeto tentará ser construido também.

Se algum construtor for criado, o construtor padrão não será, e o objeto só poderá ser
criada com um dos construtores existentes.

Você pode ter vários construtores para uma mesma classe. Desde que eles peçam
parâmetros de tipos diferentes.

```lbr
Pessoa := classe {
    ...

    construtor(nome: Texto, cpf: [u32; 4], idade: u32, altura: f32) {
        val cpf_em_texto = cpf[0].txt() + cpf[1].txt() + cpf[2].txt() + cpf[3].txt();
        retorne Pessoa {
            nome: nome,
            cpf: cpf_em_texto,
            idade: idade,
            altura: altura,
        };
    }
};
```

Uma das coisas que os construtores do LBR tem a favor de outros, é que eles podem retornar
opcionais e falháveis.

```lbr
Pessoa := classe {
    ...

    construtor(nome: Texto, cpf: Texto, idade: u32, altura: f32) ! {
        se (idade < 0 || altura < 0) {
            retorne pdr.Erro.ErroValor;
        } senão {
            retorne Pessoa {
                nome: nome,
                cpf: cpf,
                idade: idade,
                altura: altura,
            };
        }
    }
};

U32NaoZero := classe {
    n: u32;

    construtor(n: u32) ? {
        se (n == 0) {
            retorne Nada;
        } senão {
            retorne U32NaoZero {
                n: n,
            };
        }
    }
};
```
