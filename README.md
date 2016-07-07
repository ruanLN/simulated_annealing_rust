## Objetivo
Este projeto coonsiste em uma implementação da meta-heuristica "simulated annealing" aplicada sobre o _bin packing problem_.  
Nesta primeira versão os parametros do algoritmo, arquivo de entrada e forma de saida estão todos _hardcoded_, mas serão as proximas melhorias do projeto.  
Na pasta test\_files existe um conjunto de instâncias do _bin packing problem_, se quiser testar o algoritmo usando elas apenas substitua o na linha   
`let path = Path::new("test_files/N1C1W1_A.BPP");`
o nome N1C1W1\_A.BPP pelo nome do arquivo que 
deseja testar.  
## Compilando
O projeto depende da crate _Rand_, já inclusa no arquivo Cargo.toml.  
Para compilar em modo debug, já com o Rust instalado e no Path do sistema, use: 
```cargo build```
Para executar em modo _debug_ use: 
```cargo run```
Para modo _release_ apenas adicione ```--release``` aos comandos.  
Por hora o projeto não conta com testes unitários ou funções de _benchmarking_, isto será corrigido em versões futuras.

## TO-DO list
- Criar um arquivo separado para struct/impl Solution
- adicionar testes unitários aos métodos básicos (eval, new, change\_randomocally com _seed_ fixa, leitura de entradam etc)
- separar o método do simulated annealing em uma função para facilitar o benchmarking
- adicionar benchmarking de trechos criticos (eval, change randomically)
- minimizar as clonagens de vetores
- implementar parâmetros por linha de comando para uso geral
- implementar saida em forma de relatório para facilitar _parsing_ por outros programas
- implementar método de calculo de temperatura inicial dinâmico
- implementar _"pretty printing"_ da struct Solution