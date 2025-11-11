use teloxide::{prelude::*, types::InputFile};
use crate::qr::generate_qr_png;

pub async fn run_bot(bot: Bot) {
    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
    if let Some(text) = msg.text() {
        let qr_bytes = generate_qr_png(text);
        let photo = InputFile::memory(qr_bytes).file_name("qrcode.png");
        bot.send_photo(msg.chat.id, photo)
            .caption(format!("QR for: {}", text))
            .await?;
        } else {
            bot.send_message(msg.chat.id, "Please send text only.").await?;
        }
        Ok(())
    })
    .await;
}

