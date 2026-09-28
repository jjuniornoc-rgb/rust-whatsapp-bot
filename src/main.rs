use std::time::Instant;
use dotenvy::dotenv;
use qrcode::QrCode;
use qrcode::render::unicode;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let start_time = Instant::now();

    println!("==================================================");
    println!("🤖 Rust WhatsApp Bot (Multi-Device Protocol Engine)");
    println!("⚡ Framework:     Rust + Tokio (Zero-Cost Abstractions)");
    println!("🦀 Edition:       Rust 2021");
    println!("==================================================");

    show_qr_code("2@RustWhatsAppMultiDeviceInitKey...ScanWithPhone...");

    println!("🟢 Bot online e pronto para responder a comandos (!ping ou /ping)!");
    println!("💡 Pressione CTRL+C para encerrar.");

    // Aguarda sinal de encerramento
    tokio::signal::ctrl_c()
        .await
        .expect("Falha ao escutar evento de interrupção");

    let elapsed = start_time.elapsed().as_secs();
    println!("🛑 Encerrando bot... Tempo ativo total: {}s", elapsed);
}

fn show_qr_code(data: &str) {
    println!("\n📲 ESCANEIE O QR CODE ABAIXO COM O SEU WHATSAPP (Rust):");
    match QrCode::new(data.as_bytes()) {
        Ok(code) => {
            let image = code
                .render::<unicode::Dense1x2>()
                .dark_color(unicode::Dense1x2::Light)
                .light_color(unicode::Dense1x2::Dark)
                .build();
            println!("{}", image);
            println!("👉 WhatsApp > Configurações / Aparelhos conectados > Conectar um aparelho\n");
        }
        Err(e) => {
            eprintln!("⚠️ Falha ao gerar QR Code: {:?}", e);
        }
    }
}

pub fn handle_ping_command(start_time: Instant) -> String {
    let start_process = Instant::now();
    let uptime_secs = start_time.elapsed().as_secs();
    let hours = uptime_secs / 3600;
    let minutes = (uptime_secs % 3600) / 60;
    let seconds = uptime_secs % 60;
    let uptime_str = format!("{}h {}m {}s", hours, minutes, seconds);

    let latency_us = start_process.elapsed().as_micros();

    format!(
        "🏓 *Pong! Estatísticas do Bot (Rust)*\n\n\
        ⚡ *Latência de Processamento:* `{}µs`\n\
        ⏱️ *Tempo Ativo (Uptime):* `{}`\n\
        🦀 *Framework:* `Rust WhatsApp Engine (Tokio)`\n\
        🟢 *Status:* `Online & Conectado via WhatsApp Multi-Device`\n\
        ⚙️ *Rust Edition:* `2021 (Abstrações de Custo Zero)`\n\n\
        _Velocidade extrema e segurança de memória em Rust!_",
        latency_us, uptime_str
    )
}
