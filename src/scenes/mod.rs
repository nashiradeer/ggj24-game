use cursive::{
    views::{Dialog, SelectView},
    Cursive,
};

use crate::GameData;

pub mod layer0;
pub mod layer1;
pub mod layer2;

pub fn intro(s: &mut Cursive) {
    s.pop_layer();
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

                    match d {
                        0 => layer0::check_integrity(s),
                        1 => layer0::check_requirements(s),
                        2 => layer0::quit(s),
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

pub fn credits(s: &mut Cursive) {
    if let Some(game_data) = s.user_data::<GameData>() {
        game_data.stop_loop();
        game_data.play(5);
    }

    s.pop_layer();
    s.add_layer(
        Dialog::text("Um jogo por Nashira Deer, Kenai Deer e Yuri Silva.")
            .title("Créditos")
            .button("Sair", |s| {
                if let Some(game_data) = s.user_data::<GameData>() {
                    game_data.play_click();
                }

                s.quit();
            }),
    );
}
