use std::collections::VecDeque;
use std::convert::Infallible;

use futures::prelude::*;

use crate::ai::suggest;
use crate::mechanics::{collapse_lines, place_piece};

mod ai;
mod mechanics;

pub async fn run(
    mut incoming: impl Stream<Item = tbp::FrontendMessage> + Unpin,
    mut outgoing: impl Sink<tbp::BotMessage, Error = Infallible> + Unpin,
) {
    outgoing
        .send(tbp::BotMessage::Info {
            name: "Dellacherie".to_owned(),
            version: env!("CARGO_PKG_VERSION").to_owned(),
            author: "MinusKelvin".to_owned(),
            features: tbp::Feature::enabled(),
        })
        .await
        .unwrap();

    let mut state = None;

    while let Some(msg) = incoming.next().await {
        match msg {
            tbp::FrontendMessage::Quit => break,
            tbp::FrontendMessage::Start { board, queue, .. } => {
                let mut field = [[false; 10]; 40];
                for y in 0..40 {
                    for x in 0..10 {
                        field[y][x] = board[y][x].is_some();
                    }
                }
                state = Some((field, VecDeque::from(queue)));
            }
            tbp::FrontendMessage::NewPiece { piece } => {
                if let Some((_, queue)) = &mut state {
                    queue.push_back(piece);
                }
            }
            tbp::FrontendMessage::Stop => {
                state = None;
            }
            tbp::FrontendMessage::Suggest => {
                if let Some((board, queue)) = &state {
                    outgoing
                        .send(tbp::BotMessage::Suggestion {
                            moves: suggest(board, *queue.front().unwrap()),
                        })
                        .await
                        .unwrap();
                }
            }
            tbp::FrontendMessage::Play { mv } => {
                if let Some((board, queue)) = &mut state {
                    assert_eq!(queue.pop_front(), Some(mv.location.kind));
                    place_piece(board, mv.location);
                    collapse_lines(board);
                }
            }
            tbp::FrontendMessage::Rules {} => {
                outgoing.send(tbp::BotMessage::Ready).await.unwrap();
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod web;
