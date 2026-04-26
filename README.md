# Rust Clean Architecture API 🦀

Uma API RESTful construída em Rust, focada em alta performance, concorrência segura e arquitetura em camadas (Domain-Driven Design). 

Este projeto começou como um servidor TCP assíncrono construído "do zero" e evoluiu para utilizar o ecossistema moderno do Rust, demonstrando a transição de conceitos de baixo nível para abstrações web de nível de produção.

## 🛠️ Tecnologias Utilizadas

*   **Linguagem:** Rust
*   **Web Framework:** Axum (Roteamento e extração de dados HTTP)
*   **Runtime Assíncrono:** Tokio
*   **Banco de Dados:** PostgreSQL (Rodando via Docker)
*   **Database Driver:** SQLx (Queries tipadas e validadas em tempo de compilação)
*   **Serialização:** Serde (JSON parsing)

## 🏗️ Arquitetura (Camadas)

O projeto segue uma separação rigorosa de responsabilidades para garantir manutenibilidade e escalabilidade:

*   **`routes/` (Controllers):** Lida exclusivamente com requisições HTTP, extrai payloads/parâmetros e formata as respostas (JSON/Status Code).
*   **`services/` (Regras de Negócio):** O núcleo da aplicação. Orquestra a lógica, validações e decide o fluxo dos dados sem saber nada sobre HTTP ou sintaxe de Banco de Dados.
*   **`repositories/` (Acesso a Dados):** Isolamento total do `sqlx`. É a única camada que escreve queries SQL e conversa diretamente com o PostgreSQL.
*   **`models/` (Entidades):** Estruturas de dados (Structs) e definições de domínio.
*   **`responses/`:** Padronização de erros e respostas da API para o cliente.

## 🚀 Como Executar Localmente

### Pré-requisitos
*   [Rust & Cargo](https://rustup.rs/) instalados.
*   [Docker](https://www.docker.com/) e Docker Compose instalados.
*   `sqlx-cli` instalado (`cargo install sqlx-cli --no-default-features --features postgres`).

### Passo a Passo

1.  **Clone o repositório:**
    ```bash
    git clone [https://github.com/SEU_USUARIO/rust_raw_server.git](https://github.com/SEU_USUARIO/rust_raw_server.git)
    cd rust_raw_server
    ```

2.  **Inicie o Banco de Dados:**
    Isso fará o download e iniciará o PostgreSQL em segundo plano.
    ```bash
    docker compose up -d
    ```

3.  **Configuração de Ambiente:**
    Crie um arquivo `.env` na raiz do projeto com a seguinte URL de conexão:
    
```env
    DATABASE_URL=postgres://rust_server_user:rust_server_password@localhost:5432/rust_server_db
    ```

4.  **Prepare o Banco de Dados (Migrations):**
    ```bash
    sqlx database setup
    ```

5.  **Rode o Servidor:**
    ```bash
    cargo run
    ```
    O servidor estará rodando em `http://127.0.0.1:7878`.

## 📌 Endpoints da API

| Método   | Rota           | Descrição                              |
| :---     | :---           | :---                                   |
| `GET`    | `/health`      | Verifica a saúde do servidor           |
| `GET`    | `/users`       | Lista todos os usuários                |
| `POST`   | `/users`       | Cria um novo usuário                   |
| `GET`    | `/users/:id`   | Busca um usuário específico pelo ID    |
| `PUT`    | `/users/:id`   | Atualiza o nome de um usuário          |
| `DELETE` | `/users/:id`   | Remove um usuário do banco             |
