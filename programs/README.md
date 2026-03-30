**Практика до лекції з програмами**

---
**Деплой escrownконтракту**

1. Встановіть всі необхідні [залежності](https://www.anchor-lang.com/docs/installation)
Версії, що використовувались під час лекцій:
- solana-cli 3.1.8
- anchor-0.32.1

1. Зклонуйте репозиторій з прикладами
```bash
git clone https://github.com/solana-developers/program-examples.git
```

2. Зайдіть в потрібну директорію
```bash
cd ./tokens/escrow/anchor
```

3. Збілдіть контракт

```bash
cargo build
```

3. Перевірте згенеровану адресу і адресу в макросі declare_id

```bash
anchor keys list
```

4. Замініть значення адреси програми в контракті

```bash
anchor keys sync
```

5. Перевірте значення кластера в Anchor.toml

```toml
[programs.devnet]
favorites = <program_id>

[provider]
cluster = "Devnet"
```

6. Деплой на айді:

```bash
anchor deploy
```

Або на згенеровані ключі:

```bash
anchor deploy --program-keypair  program.json --program-name escrow
```
7. Перевірте акаунт:

```bash
solana account <program_id>
```

**Взаємодія з програмою з Rust**

1. Згенеруйте та намінтіть 2 нових токени

 Збілдіть і запустіть проект
```bash
cargo run -p programs -- --mint-a <mint_a> --mint-b <mint_b> --program-id <program_id>
```
