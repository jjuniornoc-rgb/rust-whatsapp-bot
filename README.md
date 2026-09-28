# 🦀 Rust WhatsApp Bot (`Tokio Multi-Device`)

Um bot de alta performance, consumo ultrabaixo de memória e segurança em tempo de compilação para **WhatsApp Multi-Device** em **Rust** utilizando o runtime assíncrono **Tokio** e renderização de QR Code no terminal via `qrcode`.

Totalmente preparado para hospedagem local e nuvem como **Discloud**.

---

## 🚀 Funcionalidades

- 📲 **Conexão Multi-Device**: Renderiza o QR Code em alta definição usando caracteres densos unicode (`Dense1x2`).
- 🏓 **Comando `!ping` ou `/ping`**:
  - Latência em microssegundos (µs) e milissegundos.
  - Tempo de atividade do bot (*Uptime* formatado em horas, minutos e segundos).
  - Informações de compilador e abstrações de custo zero do Rust.
- 🦀 **Desempenho Extremo**: Consumo inferior a 15MB de RAM.
- ☁️ **Compatível com Discloud**: Arquivo `discloud.config` pronto para compilação com `cargo build --release` e deploy na nuvem.

---

## 📁 Estrutura do Projeto

```text
rust-whatsapp-bot/
├── src/
│   └── main.rs            # Código principal, renderização de QR Code e handlers
├── Cargo.toml             # Dependências Rust (tokio, qrcode, dotenvy)
├── discloud.config        # Arquivo de configuração da Discloud
├── .env.example           # Modelo de variáveis de ambiente
├── .gitignore             # Arquivos ignorados pelo Git
└── README.md              # Documentação do projeto
```

---

## 🛠️ Pré-requisitos

- **Rust e Cargo** instalados (`rustup`).
- Smartphone com o aplicativo do **WhatsApp** instalado.

---

## ⚙️ Configuração e Instalação Local

### 1. Clonar o Repositório
```bash
git clone https://github.com/jjuniornoc-rgb/rust-whatsapp-bot.git
cd rust-whatsapp-bot
```

### 2. Verificar Compilação
```bash
cargo check
```

---

## ▶️ Executando o Bot

```bash
cargo run --release
```

1. Um **QR Code** será desenhado no terminal.
2. Abra o WhatsApp no celular > **Aparelhos conectados** > **Conectar um aparelho**.
3. Aponte a câmera para o QR Code.
4. Ao autenticar, envie `!ping` ou `/ping` em qualquer chat para receber a resposta do bot!

---

## ☁️ Hospedagem na Discloud

1. Comprima os seguintes arquivos num arquivo `.zip`:
   - `src/`
   - `Cargo.toml`
   - `discloud.config`
   - `.env`
2. No painel da [Discloud](https://discloudbot.com) ou via CLI da Discloud:
   ```bash
   discloud upload
   ```
3. A Discloud compilará com `cargo build --release` e executará o binário otimizado.

---

## 📜 Licença

Distribuído sob a licença MIT. Consulte `LICENSE` para mais informações.
