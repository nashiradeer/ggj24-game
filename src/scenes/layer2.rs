use cursive::{
    views::{Dialog, SelectView},
    Cursive,
};

use crate::GameData;

use super::credits;

pub fn i_trust_you(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(Dialog::text("Eu confio em você!").title("ಠ‿↼").button(
        "Então... sobre meu jogo",
        |s| {
            if let Some(game_data) = s.user_data::<GameData>() {
                game_data.play_click();
            }

            let mut select = SelectView::new();

            select.add_item("Concerta ele", 0);
            select.add_item("Nada não", 1);

            select.set_on_submit(|s, d| {
                if let Some(game_data) = s.user_data::<GameData>() {
                    game_data.play_click();
                }

                match d {
                    0 => {
                        todo!()
                    }
                    1 => credits(s),
                    _ => unreachable!(),
                }
            });

            s.pop_layer();
            s.add_layer(Dialog::around(select).title("O que tem ele?"));
        },
    ));
}
