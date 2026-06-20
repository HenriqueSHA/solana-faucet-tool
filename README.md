# Solana Faucet Tool ☀️

**Solana Faucet Tool** é um utilitário de linha de comando (CLI) simples e eficiente desenvolvido em Rust para automatizar a geração de novas contas na blockchain Solana e solicitar fundos de teste na Devnet.

---

## 🔍 Para Que Serve?
Este projeto foi desenvolvido como um ponto de entrada para o ecossistema Solana. Ele resolve um problema comum no início do desenvolvimento: criar uma nova carteira (Keypair) e obter SOLs de teste para interagir com a rede Devnet de forma rápida e programática, sem precisar recorrer a carteiras em navegador ou CLI complexas da Solana.

---

## ⚙️ Como Funciona?
O programa executa uma sequência direta de interações com a rede Solana:
1. **Conexão**: Cria uma instância do cliente RPC apontando para a Solana Devnet (`https://api.devnet.solana.com`).
2. **Geração de Chave**: Cria programaticamente um par de chaves criptográficas (`Keypair`) gerando uma nova conta.
3. **Solicitação de Airdrop**: Envia uma transação de airdrop requisitando **0.2 SOL** (200.000.000 lamports) para a conta recém-criada através do faucet da Devnet.
4. **Log**: Imprime no console o endereço público gerado e confirma o envio da requisição de airdrop.

---

## 🛠️ O Que Já Tem e O Que Terá?

### Já Desenvolvido (O que já tem):
- [x] Integração com `solana-client` e `solana-sdk` na versão moderna (`2.1.10`).
- [x] Geração automatizada de Keypair local temporário em memória.
- [x] Requisição automática de airdrop de 0.2 SOL na Devnet.
- [x] Tratamento básico de erros com o tipo dinâmico `Box<dyn Error>`.

### Próximos Passos (O que terá):
- [ ] **Persistência de Carteira**: Salvar a chave privada gerada em um arquivo local (ex: `id.json`) no formato padrão da Solana para ser reutilizada em outras ferramentas.
- [ ] **Confirmação de Recebimento**: Implementar uma verificação de saldo logo após a solicitação do airdrop para assegurar que a transação foi processada e os fundos foram creditados.
- [ ] **Configuração Dinâmica**: Permitir especificar o valor de airdrop desejado e selecionar a rede (Devnet, Testnet, Localnet) por meio de argumentos CLI.

---

## 🚀 Como Rodar Localmente

### Pré-requisitos:
* Rust e Cargo instalados ([rustup.rs](https://rustup.rs)).
* Conexão ativa com a internet para alcançar a Devnet.

### Passos:
1. Navegue até o diretório do projeto:
   ```bash
   cd AddSol
   ```
2. Execute o projeto usando o Cargo:
   ```bash
   cargo run
   ```
3. A saída mostrará a nova conta criada e a solicitação de airdrop:
   ```text
   Nova conta criada
   Endereço público: F2jR...uR1
   Solicitação de 200000000 lamports enviada para o endereço F2jR...uR1
   ```

---

## 📈 O Que Isso Agregou na Minha Jornada?
* **Fundamentos de Criptografia Web3**: Entendimento prático de como pares de chaves pública e privada são formados e representados na Solana.
* **Compreensão de Unidades**: Assimilação prática da diferença entre SOL e Lamports ($1 \text{ SOL} = 1.000.000.000 \text{ Lamports}$).
* **Comunicação RPC**: Primeiro contato real com o cliente RPC da Solana (`RpcClient`), entendendo como transações básicas são transmitidas para a rede de desenvolvimento (Devnet).
