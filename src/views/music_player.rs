use gpui::*;
use std::sync::Arc;

use crate::*;

pub struct MusicPlayer {
    playback: Model<Playback>,
    tracks: View<Tracks>,
    now_playing: View<NowPlaying>,
    context_menu: View<ContextMenu>,
    modal: View<Modal>,
}

impl MusicPlayer {
    pub fn new(cx: &mut ViewContext<MusicPlayer>) -> MusicPlayer {
        let playback = cx.new_model(Playback::new);

        let tracks = cx.new_view(|_cx| Tracks::new());
        let now_playing = cx.new_view(|cx| NowPlaying::new(&playback, cx));
        let context_menu = cx.new_view(|_cx| ContextMenu::new());
        let modal = cx.new_view(|_cx| Modal::new());

        cx.subscribe(&tracks, move |subscriber, _emitter, event: &Arc<UiEvent>, cx| {
            subscriber.handle_ui_event(event, cx);
        }).detach();

        cx.subscribe(&context_menu, move |subscriber, _emitter, event: &Arc<UiEvent>, cx| {
            subscriber.handle_ui_event(event, cx);
        }).detach();

        cx.subscribe(&now_playing, move |subscriber, _emitter, event: &Arc<UiEvent>, cx| {
            subscriber.handle_ui_event(event, cx);
        }).detach();

        MusicPlayer {
            playback,
            tracks,
            now_playing,
            context_menu,
            modal,
        }
    }

    fn handle_ui_event(&mut self, event: &Arc<UiEvent>, cx: &mut ViewContext<MusicPlayer>) {
        match (**event).clone() {
            UiEvent::PlayClicked(event) => self.playback.update(cx, |this, cx| {
                this.play(Arc::clone(&event.track), cx);
                cx.notify();
            }),
            UiEvent::QueueClicked(event) => self.playback.update(cx, |this, cx| {
                this.add_to_queue(Arc::clone(&event.track), cx);
                cx.notify();
            }),
            UiEvent::PauseClicked => self.playback.update(cx, |this, cx| {
                this.pause(cx);
                cx.notify();
            }),
            UiEvent::ResumeClicked => self.playback.update(cx, |this, cx| {
                this.resume(cx);
                cx.notify();
            }),
            UiEvent::SkipClicked => self.playback.update(cx, |this, cx| {
                this.skip(cx);
                cx.notify();
            }),
            UiEvent::NowPlayingTabClicked(tab_index) => self.now_playing.update(cx, |this, cx| {
                this.selected_tab = tab_index;
                cx.notify();
            }),
            UiEvent::RightClick(event) => self.context_menu.update(cx, |this, cx| {
                this.items = Arc::clone(&event.items);
                this.position = Some(event.position);
                cx.notify();
            }),
        };
    }
}

impl Render for MusicPlayer {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(theme::colours::TOUCH))
            .size_full()
            .text_color(rgb(theme::colours::WINTER))
            .font("Work Sans")
            .child(self.tracks.clone())
            .child(self.now_playing.clone())
            .child(self.context_menu.clone())
            .child(self.modal.clone())
            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, cx| {
                this.context_menu.update(cx, |context_menu, _cx| {
                    context_menu.position = None;
                });
            }))
    }
}
