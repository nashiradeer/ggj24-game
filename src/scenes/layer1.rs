use cursive::{
    views::{Dialog, SelectView},
    Cursive,
};

use crate::GameData;

use super::layer2;

pub fn scene0(s: &mut Cursive) {
    s.add_layer(
        Dialog::around(
            SelectView::new()
                .item("Sobre os arquivos do jogo", 0)
                .item("Reparando os arquivos", 1)
                .on_submit(|s, d| {
                    if let Some(game_data) = s.user_data::<GameData>() {
                        game_data.play_click();
                    }

                    s.pop_layer();
                    match d {
                        0 => layer2::scene0(s),
                        1 => layer2::scene1(s),
                        _ => unreachable!(),
                    }
                }),
        )
        .title("O que estamos fazendo mesmo?"),
    );
}

pub fn scene1(s: &mut Cursive) {
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

                            s.pop_layer();
                            layer2::scene2(s);
                        })
                        .button("...", |s| {
                            if let Some(game_data) = s.user_data::<GameData>() {
                                game_data.play_click();
                            }

                            s.pop_layer();
                            layer2::scene3(s);
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

                                s.pop_layer();
                                match d {
                                    0 => layer2::scene4(s),
                                    1 => layer2::scene5(s),
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

pub fn scene2(s: &mut Cursive) {
    if let Some(game_data) = s.user_data::<GameData>() {
        game_data.stop_loop();
    }

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

                    s.pop_layer();
                    layer2::scene6(s);
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

                            s.pop_layer();
                            layer2::scene7(s);
                        }),
                );
            }),
    );
}
