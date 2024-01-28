use cursive::{
    views::{Dialog, SelectView},
    Cursive,
};

use crate::GameData;

pub mod layer0;
pub mod layer1;
pub mod layer2;
pub mod layer3;

pub fn intro(s: &mut Cursive) {
    s.add_layer(
        Dialog::around(
            SelectView::new()
                .item("Checar integridade", 0)
                .item("Checar requisitos mínimos", 1)
                .item("Sair", 2)
                .on_submit(|s, d| {
                    if let Some(game_data) = s.user_data::<GameData>() {
                        game_data.play_click();
                    }

                    s.pop_layer();
                    match d {
                        0 => layer0::scene0(s),
                        1 => layer0::scene1(s),
                        2 => layer0::scene2(s),
                        _ => unreachable!(),
                    }
                })
                .on_select(|s, _| {
                    if let Some(game_data) = s.user_data::<GameData>() {
                        game_data.play_click();
                    }
                }),
        )
        .title("Como posso ajudar?"),
    );
}

pub fn credits(s: &mut Cursive, music: bool) {
    if music {
        if let Some(game_data) = s.user_data::<GameData>() {
            game_data.stop_loop();
            game_data.play(5);
        }
    }

    s.add_layer(
        Dialog::text("Um jogo para Global Game Jam 2024 por Nashira Deer, Kenai e Yuri Silva.")
            .title("Créditos")
            .button("Sair", |s| {
                if let Some(game_data) = s.user_data::<GameData>() {
                    game_data.play_click();
                }

                s.quit();
            }),
    );
}

pub fn demo(s: &mut Cursive) {
    if let Some(game_data) = s.user_data::<GameData>() {
        game_data.stop_loop();
        game_data.play(0);
    }

    s.add_layer(
        Dialog::text("Essa foi apenas uma demonstração!")
            .title("(੭*ˊᵕˋ)੭")
            .button("Já acabou?", |s| {
                if let Some(game_data) = s.user_data::<GameData>() {
                    game_data.play_click();
                }

                s.pop_layer();
                s.add_layer(Dialog::text("Infelizmente sim...").title("D:").button(
                    "Aaawwww",
                    |s| {
                        if let Some(game_data) = s.user_data::<GameData>() {
                            game_data.play_click();
                        }

                        s.pop_layer();
                        s.add_layer(
                            Dialog::text("Mas em breve tem mais!")
                                .title("ヽ(・⌣・)ﾉ")
                                .button("Yeeeyyy", |s| {
                                    if let Some(game_data) = s.user_data::<GameData>() {
                                        game_data.play_click();
                                    }

                                    s.pop_layer();
                                    s.add_layer(
                                        Dialog::text("Eu espero...").title("¯\\_(ツ)_/¯").button(
                                            "Oi???",
                                            |s| {
                                                if let Some(game_data) = s.user_data::<GameData>() {
                                                    game_data.play_click();
                                                }

                                                s.pop_layer();
                                                credits(s, false);
                                            },
                                        ),
                                    );
                                }),
                        );
                    },
                ));
            }),
    );
}
