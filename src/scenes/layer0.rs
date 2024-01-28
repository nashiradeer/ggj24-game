use std::{thread, time::Duration};

use cursive::{views::Dialog, Cursive};
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

use crate::GameData;

use super::{credits, layer1};

pub fn scene0(s: &mut Cursive) {
    if let Some(game_data) = s.user_data::<GameData>() {
        game_data.play_click();
        game_data.stop_loop();
        game_data.play(4);
    }

    s.add_layer(Dialog::text("Checando integridade...").title("Checando integridade"));

    let cb_sink = s.cb_sink().to_owned();

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(3));

        cb_sink
            .send(Box::new(|s| {
                s.pop_layer();
                s.add_layer(
                    Dialog::text("Ainda checando, por favor espere...")
                        .title("Checando integridade"),
                );

                let cb_sink = s.cb_sink().to_owned();
                thread::spawn(move || {
                    thread::sleep(Duration::from_secs(5));

                    cb_sink
                        .send(Box::new(|s| {
                            s.pop_layer();
                            s.add_layer(
                                Dialog::text(
                                    "Ainda procurando por ingredientes... digo, integridade. <_<",
                                )
                                .title("Adoro ingredientes"),
                            );

                            let cb_sink = s.cb_sink().to_owned();
                            thread::spawn(move || {
                                thread::sleep(Duration::from_secs(7));

                                cb_sink
                                    .send(Box::new(|s| {
                                        s.pop_layer();
                                        s.add_layer(
                                            Dialog::text("Ah... finalmente! Achei a integridade!")
                                                .title("Integridade: OK")
                                                .button("Próximo", |s| {
                                                    if let Some(game_data) =
                                                        s.user_data::<GameData>()
                                                    {
                                                        game_data.play_click();
                                                        game_data.play_loop(3);
                                                    }

                                                    s.pop_layer();
                                                    layer1::scene0(s);
                                                }),
                                        );
                                    }))
                                    .unwrap();
                            });
                        }))
                        .unwrap();
                });
            }))
            .unwrap();
    });
}

pub fn scene1(s: &mut Cursive) {
    let system = System::new_with_specifics(
        RefreshKind::new()
            .with_cpu(CpuRefreshKind::new().with_frequency())
            .with_memory(MemoryRefreshKind::new().with_ram()),
    );

    s.add_layer(
        Dialog::text(format!(
            "Apenas {} GB de RAM...",
            system.total_memory() / 1024 / 1024 / 1024
        ))
        .title("Requisitos mínimos")
        .button("Próximo", move |s| {
            if let Some(game_data) = s.user_data::<GameData>() {
                game_data.play_click();
            }

            s.pop_layer();
            s.add_layer({
                Dialog::text(format!("... e só... {} núcleos?", system.cpus().len()))
                    .title("Requisitos mínimos")
                    .button("E?...", |s| {
                        if let Some(game_data) = s.user_data::<GameData>() {
                            game_data.play_click();
                        }

                        s.pop_layer();
                        layer1::scene1(s);
                    })
            });
        }),
    );
}

pub fn scene2(s: &mut Cursive) {
    s.add_layer(Dialog::text("Oh...").button("Próximo", |s| {
        if let Some(game_data) = s.user_data::<GameData>() {
            game_data.play_click();
        }

        s.pop_layer();
        s.add_layer(Dialog::text("Espera...").button("Próximo", |s| {
            if let Some(game_data) = s.user_data::<GameData>() {
                game_data.play_click();
            }

            s.pop_layer();
            s.add_layer(
                Dialog::text("Você já vai?")
                    .button("Sim", |s| {
                        if let Some(game_data) = s.user_data::<GameData>() {
                            game_data.play_click();
                        }

                        s.pop_layer();
                        s.add_layer(Dialog::text("Ta"));

                        let cb_sink = s.cb_sink().to_owned();

                        thread::spawn(move || {
                            thread::sleep(Duration::from_secs(1));
                            cb_sink
                                .send(Box::new(|s| {
                                    credits(s);
                                }))
                                .unwrap();
                        });
                    })
                    .button("Não", |s| {
                        if let Some(game_data) = s.user_data::<GameData>() {
                            game_data.play_click();
                        }

                        s.pop_layer();
                        layer1::scene2(s);
                    }),
            )
        }));
    }));
}
