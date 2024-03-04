use gpui::*;
use std::sync::Arc;

use crate::*;

type Vcx<'a> = ViewContext<'a, Tracks>;

pub struct Tracks {
    view: TrackView,
    tracks: Vec<Arc<Track>>,
}

enum TrackView {
    AllTracks,
    // AllArtists,
    // AllAlbums,
    ArtistTracks(String),
    // ArtistAlbums(String),
    Album(String, String),
    // Label(String),
    // Playlist(String),
}

impl Tracks {
    pub fn new(cx: &mut Vcx, library: &Model<Library>) -> Tracks {
        cx.observe(library, |this, library, cx| {
            this.tracks = get_tracks(cx, &library, &this.view);
            cx.notify();
        }).detach();

        let view = TrackView::AllTracks;
        let tracks = get_tracks(cx, library, &view);

        Tracks {
            view,
            tracks,
        }
    }
}

impl Render for Tracks {
    fn render(&mut self, cx: &mut ViewContext<Tracks>) -> impl IntoElement {
        div()
            .id("tracks")
            .size_full()
            .overflow_y_scroll()
            .children(
                self.tracks.iter().map(|track|
                    elements::track(track, cx)
                )
            )
    }
}

fn get_tracks(cx: &mut Vcx, library: &Model<Library>, view: &TrackView) -> Vec<Arc<Track>> {
    let tracks = (*library.read(cx).tracks).clone();

    match view {
        TrackView::AllTracks => tracks,
        TrackView::ArtistTracks(artist) => tracks.into_iter()
            .filter(|track| track.artist_name == *artist).collect(),
        TrackView::Album(artist, album) => tracks.into_iter()
            .filter(|track| track.artist_name == *artist && track.album_title == *album).collect(),
    }
}
