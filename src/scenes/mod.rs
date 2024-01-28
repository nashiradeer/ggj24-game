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

    let mut select = SelectView::new();

    select.add_item("Checar integridade", 0);
    select.add_item("Checar requisitos mínimos", 1);
    select.add_item("Sair", 2);

    select.set_on_submit(|s, d| {
        if let Some(game_data) = s.user_data::<GameData>() {
            game_data.play_click();
        }

        match d {
            0 => layer0::check_integrity(s),
            1 => {
                s.pop_layer();
                s.add_layer(layer0::check_requirements())
            }
            2 => {
                s.pop_layer();
                s.add_layer(layer0::quit())
            }
            _ => unreachable!(),
        }
    });

    s.add_layer(Dialog::around(select).title("Como posso ajudar?"));
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
