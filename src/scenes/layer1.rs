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
                        .button("Como você sabe?...", |s| {
                            if let Some(game_data) = s.user_data::<GameData>() {
                                game_data.play_click();
                            }

                            layer2::i_know(s);
                        }),
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
            0 => layer2::armored(s),
            1 => layer2::repairing_game_files(s),
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
                        .button("Qual o problema?...", |s| {
                            if let Some(game_data) = s.user_data::<GameData>() {
                                game_data.play_click();
                            }

                            layer2::whats_problem(s);
                        })
                        .button("...", |s| {
                            if let Some(game_data) = s.user_data::<GameData>() {
                                game_data.play_click();
                            }

                            layer2::what_happen(s);
                        }),
                );
            })
            .button("Não", |s| {
                if let Some(game_data) = s.user_data::<GameData>() {
                    game_data.play_click();
                }

                s.pop_layer();
                s.add_layer(
                    Dialog::around(
                        SelectView::new()
                            .item("O que eu faço?", 0)
                            .item("Faz funcionar", 1)
                            .on_submit(|s, d| {
                                if let Some(game_data) = s.user_data::<GameData>() {
                                    game_data.play_click();
                                }

                                match d {
                                    0 => layer2::what_i_do(s),
                                    1 => layer2::only_do(s),
                                    _ => unreachable!(),
                                }
                            })
                            .on_select(|s, _| {
                                if let Some(game_data) = s.user_data::<GameData>() {
                                    game_data.play_click();
                                }
                            }),
                    )
                    .title("..."),
                )
            }),
    );
}
