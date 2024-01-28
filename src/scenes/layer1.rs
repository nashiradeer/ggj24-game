use cursive::{
    views::{Dialog, SelectView},
    Cursive,
};

use crate::GameData;

use super::layer2;

pub fn quit(s: &mut Cursive) {
    if let Some(game_data) = s.user_data::<GameData>() {
        game_data.stop_loop();
    }

    s.pop_layer();
    s.add_layer(
        Dialog::text("Oh... Então... Por que você clicou no botão de sair? >:C")
            .title("Ficou maluco?")
            .button("Eu não cliquei...", |s| {
                if let Some(game_data) = s.user_data::<GameData>() {
                    game_data.play_click();
                }

                s.pop_layer();
                s.add_layer(Dialog::text(">_>...").title("Hmmm...").button("...", |s| {
                    if let Some(game_data) = s.user_data::<GameData>() {
                        game_data.play_click();
                        game_data.play_loop(3);
                    }

                    layer2::i_trust_you(s);
                }));
            })
            .button("Porque eu quis!", |s| {
                if let Some(game_data) = s.user_data::<GameData>() {
                    game_data.play_click();
                    game_data.play_loop(2);
                }

                s.pop_layer();
                s.add_layer(
                    Dialog::text("Muita coragem para quem se chama *Insira seu nome aqui*...")
                        .button("Como você sabe?...", |_| todo!()),
                );
            }),
    );
}

pub fn ingredients(s: &mut Cursive) {
    s.pop_layer();

    let mut select = SelectView::new();

    select.add_item("Sobre os arquivos do jogo", 0);
    select.add_item("Reparando os arquivos", 1);

    select.set_on_submit(|s, d| {
        if let Some(game_data) = s.user_data::<GameData>() {
            game_data.play_click();
        }

        match d {
            0 => todo!(),
            1 => todo!(),
            _ => unreachable!(),
        }
    });

    s.add_layer(Dialog::around(select).title("O que estamos fazendo mesmo?"));
}

pub fn not_enough(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::text("Você acha que isso é suficiente?")
            .title("ರ_ರ")
            .button("Sim", |s| {
                if let Some(game_data) = s.user_data::<GameData>() {
                    game_data.play_click();
                }

                s.pop_layer();
                s.add_layer(
                    Dialog::text("Não me surpreende não rodar...")
                        .title("Que triste...")
                        .button("Qual o problema?...", |s| todo!())
                        .button("...", |s| todo!()),
                );
            })
            .button("Não", |s| {
                if let Some(game_data) = s.user_data::<GameData>() {
                    game_data.play_click();
                }

                let mut select = SelectView::new();

                select.add_item("O que eu faço?", 0);
                select.add_item("Faz funcionar", 1);

                select.set_on_submit(|s, d| {
                    if let Some(game_data) = s.user_data::<GameData>() {
                        game_data.play_click();
                    }

                    match d {
                        0 => todo!(),
                        1 => todo!(),
                        _ => unreachable!(),
                    }
                });

                s.pop_layer();
                s.add_layer(Dialog::around(select).title("..."))
            }),
    );
}
