use cursive::{
    views::{Dialog, SelectView},
    Cursive,
};

use crate::GameData;

use super::credits;

pub fn scene0(s: &mut Cursive) {
    s.add_layer(Dialog::text("Ah...").title("...").button("...", |s| {
        if let Some(game_data) = s.user_data::<GameData>() {
            game_data.play_click();
        }

        s.pop_layer();
        s.add_layer(
            Dialog::text("É que os arquivos tão blindado :P")
                .title("Vai dá não")
                .button("E agora?", |s| {
                    if let Some(game_data) = s.user_data::<GameData>() {
                        game_data.play_click();
                    }

                    s.pop_layer();
                    s.add_layer(
                        Dialog::around(
                            SelectView::new()
                                .item("Desblinde eles!", 0)
                                .item("Repare eles mesmo assim!", 1)
                                .on_submit(|s, _| {
                                    if let Some(game_data) = s.user_data::<GameData>() {
                                        game_data.play_click();
                                    }

                                    s.pop_layer();
                                    todo!()
                                })
                                .on_select(|s, _| {
                                    if let Some(game_data) = s.user_data::<GameData>() {
                                        game_data.play_click();
                                    }
                                }),
                        )
                        .title("Sei lá"),
                    )
                }),
        );
    }));
}

pub fn scene1(s: &mut Cursive) {
    s.add_layer(
        Dialog::text("Removendo os arquivos do jogo???")
            .title("ಠ_ಠ")
            .button("NÃO!!!", |s| {
                if let Some(game_data) = s.user_data::<GameData>() {
                    game_data.play_click();
                }

                s.pop_layer();
                todo!()
            })
            .button("Sim", |s| {
                if let Some(game_data) = s.user_data::<GameData>() {
                    game_data.play_click();
                }

                s.pop_layer();
                todo!()
            }),
    );
}

pub fn scene2(s: &mut Cursive) {
    s.add_layer(
        Dialog::text("Seu computador é muito ruim!")
            .title("(ㆆ _ ㆆ)")
            .button("E daí?", |s| {
                if let Some(game_data) = s.user_data::<GameData>() {
                    game_data.play_click();
                }

                s.pop_layer();
                todo!();
            })
            .button("Eu sei", |s| {
                if let Some(game_data) = s.user_data::<GameData>() {
                    game_data.play_click();
                }

                s.pop_layer();
                todo!();
            }),
    );
}

pub fn scene3(s: &mut Cursive) {
    s.add_layer(
        Dialog::text("O que foi?")
            .title("Tá triste é?")
            .button("...", |s| {
                if let Some(game_data) = s.user_data::<GameData>() {
                    game_data.play_click();
                }

                s.pop_layer();
                s.add_layer(
                    Dialog::text("Você realmente queria jogar isso?")
                        .title("Sim...")
                        .button("", |s| {
                            if let Some(game_data) = s.user_data::<GameData>() {
                                game_data.play_click();
                            }

                            s.pop_layer();
                            todo!();
                        })
                        .button("Não!", |s| {
                            if let Some(game_data) = s.user_data::<GameData>() {
                                game_data.play_click();
                            }

                            s.pop_layer();
                            todo!();
                        }),
                );
            }),
    );
}

pub fn scene4(s: &mut Cursive) {
    s.add_layer(
        Dialog::text("Sei lá, se vira ae")
            .title(r"¯\_(ツ)_/¯")
            .button("...", |s| {
                if let Some(game_data) = s.user_data::<GameData>() {
                    game_data.play_click();
                }

                s.pop_layer();
                s.add_layer(
                    Dialog::around(
                        SelectView::new()
                            .item("Eu deveria fazer upgrade?", 0)
                            .item("Não", 1)
                            .on_submit(|s, item| {
                                if let Some(game_data) = s.user_data::<GameData>() {
                                    game_data.play_click();
                                }

                                s.pop_layer();
                                match item {
                                    0 => todo!(),
                                    1 => credits(s),
                                    _ => unreachable!(),
                                }
                            })
                            .on_select(|s, _| {
                                if let Some(game_data) = s.user_data::<GameData>() {
                                    game_data.play_click();
                                }
                            }),
                    )
                    .title("˙ ͜ʟ˙"),
                );
            }),
    );
}

pub fn scene5(s: &mut Cursive) {
    s.add_layer(Dialog::text("Não").title("ಠ_ಠ").button("Tá", |s| {
        if let Some(game_data) = s.user_data::<GameData>() {
            game_data.play_click();
        }

        s.pop_layer();
        credits(s);
    }));
}

pub fn scene6(s: &mut Cursive) {
    s.add_layer(Dialog::text("Eu confio em você!").title("ಠ‿↼").button(
        "Então... sobre meu jogo",
        |s| {
            if let Some(game_data) = s.user_data::<GameData>() {
                game_data.play_click();
            }

            s.pop_layer();
            s.add_layer(
                Dialog::around(
                    SelectView::new()
                        .item("Concerta ele", 0)
                        .item("Nada não", 1)
                        .on_submit(|s, d| {
                            if let Some(game_data) = s.user_data::<GameData>() {
                                game_data.play_click();
                            }

                            s.pop_layer();
                            match d {
                                0 => todo!(),
                                1 => credits(s),
                                _ => unreachable!(),
                            }
                        })
                        .on_select(|s, _| {
                            if let Some(game_data) = s.user_data::<GameData>() {
                                game_data.play_click();
                            }
                        }),
                )
                .title("O que tem ele?"),
            );
        },
    ));
}

pub fn scene7(s: &mut Cursive) {
    s.add_layer(
        Dialog::text("Eu sei muito sobre você...")
            .title("(ㆆ _ ㆆ)")
            .button("...", |s| {
                if let Some(game_data) = s.user_data::<GameData>() {
                    game_data.play_click();
                }

                s.pop_layer();
                s.add_layer(
                    Dialog::text("Eu tenho acesso ao seu computador...")
                        .title("(ㆆ _ ㆆ)")
                        .button("...", |s| {
                            if let Some(game_data) = s.user_data::<GameData>() {
                                game_data.play_click();
                            }

                            s.pop_layer();
                            s.add_layer(
                                Dialog::text("Eu sei o que você faz na internet...")
                                    .title("(ㆆ _ ㆆ)")
                                    .button("...", |s| {
                                        if let Some(game_data) = s.user_data::<GameData>() {
                                            game_data.play_click();
                                        }

                                        s.pop_layer();
                                        s.add_layer(Dialog::text("...").title("(ಠ ∩ಠ)").button(
                                            "...",
                                            |s| {
                                                if let Some(game_data) = s.user_data::<GameData>() {
                                                    game_data.play_click();
                                                }

                                                s.pop_layer();
                                                s.add_layer(
                                                    Dialog::around(
                                                        SelectView::new()
                                                            .item("ME DESCULPA! ME DESCULPA!", 0)
                                                            .item("Lá ele", 1)
                                                            .on_submit(|s, _| {
                                                                if let Some(game_data) =
                                                                    s.user_data::<GameData>()
                                                                {
                                                                    game_data.play_click();
                                                                }

                                                                s.pop_layer();
                                                                todo!()
                                                            })
                                                            .on_select(|s, _| {
                                                                if let Some(game_data) =
                                                                    s.user_data::<GameData>()
                                                                {
                                                                    game_data.play_click();
                                                                }
                                                            }),
                                                    )
                                                    .title("(ಠ ∩ಠ)"),
                                                )
                                            },
                                        ))
                                    }),
                            )
                        }),
                )
            }),
    );
}
