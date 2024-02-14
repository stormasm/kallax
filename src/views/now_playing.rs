use gpui::*;
use std::sync::Arc;

use crate::*;

pub struct NowPlaying {
    pub track: Option<Arc<Track>>,
}

impl NowPlaying {
    pub fn new() -> NowPlaying {
        NowPlaying { track: None }
    }
}

impl Render for NowPlaying {
    fn render(&mut self, cx: &mut ViewContext<NowPlaying>) -> impl IntoElement {
        let now_playing = div()
            .py_1()
            .px_3()
            .border()
            .border_color(rgb(COLOUR_BORDER))
            .size_full()
            .child("Now playing:")
            .child(self.track.clone().map_or("-".to_string(), |track| track.name.clone()));

        let now_playing = if let Some(track) = self.track.clone() {
            if let Some(artwork) = track.artwork.clone() {
                now_playing.child(
                    img(artwork)
                        .flex_none()
                        .w_80()
                        .h_80()
                )
            } else {
                now_playing
            }
        } else {
            now_playing
        };

        now_playing
            .child(div().id("pause").child("Pause").on_click(cx.listener(|_this, _event, cx| {
                cx.emit(Arc::new(UiEvent::PauseClicked))
            })))
            .child(div().id("resume").child("Resume").on_click(cx.listener(|_this, _event, cx| {
                cx.emit(Arc::new(UiEvent::ResumeClicked))
            })))
            .child(div().id("skip").child("Skip").on_click(cx.listener(|_this, _event, cx| {
                cx.emit(Arc::new(UiEvent::SkipClicked))
            })))
    }
}
