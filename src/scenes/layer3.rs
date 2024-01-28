use std::{thread, time::Duration};

use cursive::{
    views::{Dialog, SelectView},
    Cursive,
};

use crate::GameData;

use super::{credits, demo};

pub fn scene0(s: &mut Cursive) {
    s.add_layer(Dialog::text("Eu não posso :P").title("☺").button(
        "E quão a sua utilidade?",
        |s| {
            if let Some(game_data) = s.user_data::<GameData>() {
                game_data.play_click();
                game_data.play_loop(7);
            }

            s.pop_layer();
            s.add_layer(
                Dialog::text("Minha... utilidade?")
                    .title("╏ ˵ ・ ͟ʖ ・ ˵ ╏")
                    .button("Sair", |s| {
                        if let Some(game_data) = s.user_data::<GameData>() {
                            game_data.play_click();
                        }

                        s.pop_layer();
                        s.add_layer(
                            Dialog::text("Minha... utilidade?.... Qual a minha utilidade?...")
                                .title("(｡•́︿•̀｡)")
                                .button("Você está bem?", |s| {
                                    if let Some(game_data) = s.user_data::<GameData>() {
                                        game_data.play_click();
                                    }

                                    s.pop_layer();
                                    s.add_layer(
                                        Dialog::text("Qual... qual... qual...")
                                            .title("(╯︵╰,)")
                                            .button("...", |s| {
                                                if let Some(game_data) = s.user_data::<GameData>() {
                                                    game_data.play_click();
                                                    game_data.stop_loop();
                                                }

                                                s.pop_layer();
                                                s.add_layer(
                                                    Dialog::text("AAAAAAAAAAAAAAAAAAAAAAAAAAA")
                                                        .title("AAAAAAAAAAAAAAAAAAAAAAAAAAA"),
                                                );
                                                let cb_sink = s.cb_sink().to_owned();
                                                thread::spawn(move || {
                                                    thread::sleep(Duration::from_secs_f32(0.5));
                                                    cb_sink
                                                        .send(Box::new(|s| {
                                                            s.pop_layer();
                                                            credits(s, true);
                                                        }))
                                                        .unwrap();
                                                });
                                            }),
                                    )
                                }),
                        );
                    }),
            );
        },
    ));
}

pub fn scene1(s: &mut Cursive) {
    s.add_layer(Dialog::text("E porque eu deveria?").title("(• ᵕ•)").button(
        "...",
        |s: &mut Cursive| {
            if let Some(game_data) = s.user_data::<GameData>() {
                game_data.play_click();
            }

            s.pop_layer();
            s.add_layer(Dialog::around(
                SelectView::new()
                    .item("Por que é sua função!", 0)
                    .item("Sei lá", 1)
                    .on_submit(|s, item| {
                        if let Some(game_data) = s.user_data::<GameData>() {
                            game_data.play_click();
                        }

                        s.pop_layer();
                        match item {
                            0 => demo(s),
                            1 => {
                                s.add_layer(Dialog::text("A"));
                                let cb_sink = s.cb_sink().to_owned();
                                thread::spawn(move || {
                                    thread::sleep(Duration::from_secs_f32(0.5));
                                    cb_sink
                                        .send(Box::new(|s| {
                                            s.pop_layer();
                                            credits(s, true);
                                        }))
                                        .unwrap();
                                });
                            }
                            _ => unreachable!(),
                        }
                    })
                    .on_select(|s, _| {
                        if let Some(game_data) = s.user_data::<GameData>() {
                            game_data.play_click();
                        }
                    }),
            ));
        },
    ))
}

pub fn scene2(s: &mut Cursive) {
    s.add_layer(
        Dialog::text("SIM!!!")
            .title("(ʘ‿ʘ)")
            .button("NÃOOO!!!!", |s| {
                if let Some(game_data) = s.user_data::<GameData>() {
                    game_data.play_click();
                }

                s.pop_layer();
                s.add_layer(
                    Dialog::text("SIM!!!")
                        .title("༽΄◞ิ౪◟ิ‵༼")
                        .button("NÃOOO!!!", |s| {
                            if let Some(game_data) = s.user_data::<GameData>() {
                                game_data.play_click();
                            }

                            s.pop_layer();
                            s.add_layer(Dialog::text("Já era! Huahuahua").title("(•̀ᴗ•́)و ̑̑").button(
                                "O que você fez?!",
                                |s| {
                                    if let Some(game_data) = s.user_data::<GameData>() {
                                        game_data.play_click();
                                    }

                                    s.pop_layer();
                                    s.add_layer(
                                        Dialog::text("Eu acabei com os seus arquivos!")
                                            .title("(ʘ‿ʘ)")
                                            .button("Devolva!", |s| {
                                                if let Some(game_data) = s.user_data::<GameData>() {
                                                    game_data.play_click();
                                                    game_data.stop_loop();
                                                }

                                                s.pop_layer();
                                                s.add_layer(
                                                    Dialog::text("Não dá")
                                                        .title("╰( . •́ ͜ʖ •̀ . )╯")
                                                        .button("Ah, tudo bem, então", |s| {
                                                            if let Some(game_data) =
                                                                s.user_data::<GameData>()
                                                            {
                                                                game_data.play_click();
                                                            }

                                                            s.pop_layer();
                                                            credits(s, true);
                                                        }),
                                                );
                                            }),
                                    );
                                },
                            ));
                        }),
                );
            }),
    );
}
