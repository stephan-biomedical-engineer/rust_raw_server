# Rust Clean Architecture API 🦀

API RESTful construída em Rust com foco em performance, segurança, concorrência assíncrona e arquitetura em camadas.

Este projeto começou como um servidor TCP/HTTP construído manualmente para estudar conceitos de baixo nível em Rust, como ownership, borrowing, threads, concorrência e I/O. Depois evoluiu para uma API moderna usando Axum, Tokio, PostgreSQL, SQLx, Docker, autenticação JWT, validação de entrada e logs estruturados.

## ✨ Features

- API REST com Axum
- Runtime assíncrono com Tokio
- PostgreSQL via Docker
- SQLx com migrations
- Autenticação JWT
- Hash de senha com Argon2
- Rotas protegidas por extractor customizado
- Validação de payloads com `validator`
- Logs estruturados com `tracing`
- CORS configurado
- Healthcheck no Docker
- Dockerfile multi-stage
- Docker Compose com API + banco
- Migrations automáticas no startup do container
- Arquitetura em camadas
- Validação de payloads com `validator`
- Logs estruturados com `tracing` (JSON em produção)
- Rate limiting com `tower-governor`
- Testes de integração com `tokio` + `reqwest`

## 🛠️ Tecnologias Utilizadas

- **Linguagem:** Rust
- **Web Framework:** Axum
- **Runtime Assíncrono:** Tokio
- **Banco de Dados:** PostgreSQL
- **Database Driver:** SQLx
- **Serialização:** Serde
- **Autenticação:** JWT
- **Hash de Senha:** Argon2
- **Validação:** validator
- **Logs:** tracing / tracing-subscriber
- **Containerização:** Docker / Docker Compose

## 🏗️ Arquitetura

O projeto segue uma separação de responsabilidades em camadas:

```text
src/
├── auth/
│   ├── extractor.rs
│   ├── jwt.rs
│   └── mod.rs
├── models/
│   ├── auth.rs
│   ├── mod.rs
│   └── user.rs
├── repositories/
│   ├── mod.rs
│   └── users_repository.rs
├── responses/
│   ├── api_response.rs
│   └── mod.rs
├── routes/
│   ├── auth.rs
│   ├── health.rs
│   ├── mod.rs
│   └── users.rs
├── services/
│   ├── auth_service.rs
│   ├── mod.rs
│   └── users_service.rs
├── app.rs          # montagem do Router (rotas + middlewares)
├── config.rs       # leitura de variáveis de ambiente
├── telemetry.rs    # configuração de logs (JSON ou texto)
├── main.rs         # bootstrap da aplicação
└── lib.rs          # módulos
````

### `routes/`

Camada HTTP. Recebe requisições, extrai `Json`, `Path`, `State`, valida payloads e retorna respostas HTTP.

### `services/`

Camada de regras de negócio. Decide o fluxo da aplicação sem depender diretamente de HTTP.

### `repositories/`

Camada de acesso ao banco. É onde ficam as queries SQL usando SQLx.

### `models/`

Structs usadas pela aplicação, como `User`, `RegisterRequest`, `LoginRequest` e `AuthResponse`.

### `responses/`

Padronização de erros da API, como `not_found`, `unauthorized`, `validation_error` e `internal_error`.

### `auth/`

Responsável por JWT, hash de senha, validação de senha e extractor de usuário autenticado.

## 🔐 Autenticação

A API usa autenticação com JWT.

Fluxo:

```text
```md
POST /auth/register
→ cria usuário com senha hasheada usando Argon2

POST /auth/login
→ valida email/senha
→ retorna token JWT

PUT /users/{id}
DELETE /users/{id}
→ exigem Authorization: Bearer <token>
```

As senhas nunca são salvas em texto puro. Apenas o `password_hash` é persistido no banco.

## 🚀 Como executar com Docker Compose

### 1. Crie o arquivo `.env`

## ⚙️ Variáveis de Ambiente

O projeto usa múltiplos arquivos `.env`:

| Arquivo      | Uso                     |
|--------------|------------------------|
| `.env.dev`   | Desenvolvimento local  |
| `.env.test`  | Testes automatizados   |
| `.env.prod`  | Produção (futuro)      |
| `.env.example` | Template versionado |

### Exemplo (`.env.example`)

```env
POSTGRES_USER=user
POSTGRES_PASSWORD=password
POSTGRES_DB=rust_db

DATABASE_URL=postgres://user:password@localhost:5432/rust_db

JWT_SECRET=your_super_secret_key_here

APP_ENV=development
RUST_LOG=info
```

> Dentro do Docker Compose, o host do banco é `postgres`, não `localhost`.

### 2. Suba a aplicação

```bash
docker compose --env-file .env up --build -d
```

Isso sobe:

* PostgreSQL
* API Rust
* migrations automáticas
* healthcheck da API

### 3. Verifique os containers

```bash
docker ps
```

A API deve aparecer como `healthy`.

### 4. Teste o healthcheck

```bash
curl http://127.0.0.1:7878/health
```

Resposta esperada:

```json
{
  "status": "ok",
  "message": "Server is healthy"
}
```


---

## Nova seção: Logs
## 📊 Observabilidade (Logs)

A aplicação usa `tracing` com dois modos:

- **development** → logs coloridos e legíveis
- **production** → logs em JSON estruturado

Controlado pela variável:

```env
APP_ENV=development | production
```
Exemplo de log em produção:

```json
{
  "level": "INFO",
  "user_id": 42,
  "message": "User logged in successfully"
}
```

## Nova seção: Testes
## 📊 Testes Automatizados

Os testes ficam em:

```text
tests/api.rs
```

Para rodar:

```text
cargo test
```

Os testes usam um banco separado (.env.test) e podem rodar migrations automaticamente.

## 🧪 Executando localmente sem Docker para a API

Você também pode rodar a API localmente e deixar apenas o Postgres no Docker.

Nesse caso, use uma `DATABASE_URL` com `localhost`:

```env
DATABASE_URL=postgres://rust_server_user:rust_server_password@localhost:5432/rust_server_db
JWT_SECRET=uma_chave_super_secreta_para_estudo_123456789
RUST_LOG=info
```

Suba o Postgres:

```bash
docker compose --env-file .env.dev up -d postgres
```

Rode migrations:

```bash
sqlx migrate run
```

Rode a API:

```bash
cargo run
```

## 📌 Endpoints

| Método   | Rota             | Protegida | Descrição                   |
| :------- | :--------------- | :-------: | :-------------------------- |
| `GET`    | `/health`        |    Não    | Verifica a saúde da API     |
| `POST`   | `/auth/register` |    Não    | Registra um novo usuário    |
| `POST`   | `/auth/login`    |    Não    | Realiza login e retorna JWT |
| `GET`    | `/users`         |    Não    | Lista usuários              |
| `GET`    | `/users/{id}`    |    Não    | Busca usuário por ID        |
| `PUT`    | `/users/{id}`    |    Sim    | Atualiza o próprio usuário  |
| `DELETE` | `/users/{id}`    |    Sim    | Remove o próprio usuário    |

## 🔑 Exemplos de uso

### Registrar usuário

```bash
curl -i -X POST http://127.0.0.1:7878/auth/register \
  -H "Content-Type: application/json" \
  -d '{"name":"Stephan","email":"stephan@test.com","password":"12345678"}'
```

### Login

```bash
curl -i -X POST http://127.0.0.1:7878/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"stephan@test.com","password":"12345678"}'
```

Resposta esperada:

```json
{
  "token": "eyJ...",
  "token_type": "Bearer"
}
```

### Salvar token em variável

```bash
TOKEN=$(curl -s -X POST http://127.0.0.1:7878/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"stephan@test.com","password":"12345678"}' \
  | jq -r '.token')
```

### Atualizar usuário autenticado

```bash
curl -i -X PUT http://127.0.0.1:7878/users/5 \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"name":"Stephan Atualizado"}'
```

### Deletar usuário autenticado

```bash
curl -i -X DELETE http://127.0.0.1:7878/users/5 \
  -H "Authorization: Bearer $TOKEN"
```

## 🧾 Migrations

As migrations ficam em:

```text
migrations/
```

Para criar uma nova migration:

```bash
sqlx migrate add nome_da_migration
```

Para rodar localmente:

```bash
sqlx migrate run
```

No Docker, as migrations são executadas automaticamente pelo script:

```text
scripts/docker-entrypoint.sh
```

## 🌐 CORS

A API possui CORS configurado via `tower-http`.

Em desenvolvimento, pode ser usado:

```rust
.allow_origin(tower_http::cors::Any)
```

Para produção, o ideal é restringir para o domínio real do frontend:

```rust
.allow_origin("https://seu-dominio.com".parse::<HeaderValue>().unwrap())
```

## 🩺 Healthcheck

O container da API possui healthcheck configurado no Docker Compose, usando:

```text
GET /health
```

Isso permite verificar se a API está realmente pronta para receber tráfego.

## 🔒 Segurança

Já implementado:

* Hash de senha com Argon2
* JWT com expiração
* Rotas protegidas por extractor customizado
* Validação de entrada
* CORS
* Rate limiting
* Variáveis de ambiente separadas por ambiente
* Logs estruturados (observabilidade)

## 🧭 Próximos passos

* [ ] Refresh tokens
* [ ] Revogação de tokens
* [ ] Secrets externos (AWS / Docker Secrets)
* [ ] HTTPS com reverse proxy (Nginx / Traefik)
* [ ] CI/CD com GitHub Actions
* [ ] Backup automatizado do PostgreSQL
* [ ] Deploy em Kubernetes
* [ ] Frontend consumindo a API

---

## 📚 Objetivo do Projeto

Este projeto nasceu como um exercício para entender servidores web em Rust, concorrência, ownership e borrowing em contexto real. A primeira versão implementava HTTP manualmente com TCP. A versão atual usa o ecossistema moderno do Rust para demonstrar como esses conceitos evoluem para uma API real, segura, assíncrona e containerizada.

---
