//! Scenes
//!
//! Contains all the scenes for the game.

use cursive::views::{Dialog, SelectView};

pub fn intro() -> Dialog {
    Dialog::text("Seja bem-vindo ao Assistente de manutenção!")
        .title("Assistente de manutenção")
        .button("Próximo", |s| {
            s.pop_layer();
            s.add_layer(
                Dialog::around({
                    let mut select = SelectView::new();

                    select.add_item("Reparar o jogo", "reparar");
                    select.add_item("Não olhe para trás...", "olhe");
                    select.add_item("Meu gato explodiu!?!", "gato");

                    select.set_on_submit(|_s, item| match item {
                        "reparar" => todo!(),
                        "olhe" => todo!(),
                        "gato" => todo!(),
                        _ => unreachable!(),
                    });

                    select
                })
                .title("Como posso ajudar?"),
            )
        })
}
