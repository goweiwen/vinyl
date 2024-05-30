use std::{
    borrow::Cow,
    fs::File,
    io::{Read, Seek},
    path::PathBuf,
    time::Duration,
};

use anyhow::{anyhow, Result};
use image::RgbaImage;
use lofty::{
    file::{AudioFile, TaggedFileExt},
    picture::PictureType,
    tag::Accessor,
};
use slint::SharedPixelBuffer;

use crate::Song;

pub struct SongData {
    pub path: PathBuf,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub cover_art: Option<RgbaImage>,
    pub duration: Duration,
}

impl SongData {
    pub fn load(path: PathBuf) -> Result<Self> {
        let file = &mut File::open(&path)?;
        let tagged_file = lofty::read_from(file)?;
        file.seek(std::io::SeekFrom::Start(0))?;
        let Some(tag) = tagged_file.primary_tag() else {
            return Err(anyhow!("Tag not found"));
        };

        let title = tag.title().map(Cow::into_owned);
        let artist = tag.artist().map(Cow::into_owned);
        let album = tag.album().map(Cow::into_owned);

        let properties = tagged_file.properties();
        let duration = properties.duration();

        let cover_art = tag
            .pictures()
            .iter()
            .find(|x| x.pic_type() == PictureType::CoverFront || x.pic_type() == PictureType::Other)
            .map(|picture| image::load_from_memory(picture.data()))
            .transpose()?
            .map(|image| image.to_rgba8());

        Ok(Self {
            path,
            title,
            artist,
            album,
            cover_art,
            duration,
        })
    }

    pub fn cover_art(&self, border_radius: u32) -> Result<slint::Image> {
        Ok(if let Some(ref cover_art) = self.cover_art {
            let bytes = cover_art.bytes().collect::<std::io::Result<Vec<_>>>()?;
            let image = RgbaImage::from_vec(cover_art.width(), cover_art.height(), bytes).unwrap();
            let mut image =
                image::imageops::resize(&image, 250, 250, image::imageops::FilterType::Lanczos3);
            crate::image::squircle(&mut image, border_radius);

            slint::Image::from_rgba8(SharedPixelBuffer::clone_from_slice(
                image.as_raw(),
                image.width(),
                image.height(),
            ))
        } else {
            slint::Image::default()
        })
    }
}

impl From<&SongData> for Song {
    fn from(song: &SongData) -> Self {
        Song {
            album: song.album.as_deref().unwrap_or_default().into(),
            artist: song.artist.as_deref().unwrap_or_default().into(),
            cover_art: song.cover_art(125).unwrap_or_default(),
            duration: song.duration.as_secs() as i32,
            path: song.path.to_string_lossy().as_ref().into(),
            title: song.title.as_deref().unwrap_or_default().into(),
        }
    }
}
