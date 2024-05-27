use std::{
    borrow::Cow,
    fs::File,
    io::{Read, Seek},
    path::PathBuf,
};

use anyhow::{anyhow, Result};
use image::RgbaImage;
use lofty::{file::TaggedFileExt, picture::PictureType, tag::Accessor};
use slint::SharedPixelBuffer;

pub struct SongData {
    pub path: PathBuf,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub cover_art: Option<RgbaImage>,
}

impl SongData {
    pub fn load(path: PathBuf) -> Result<Self> {
        let mut file = &mut File::open(&path)?;
        let tag = lofty::read_from(&mut file)?;
        file.seek(std::io::SeekFrom::Start(0))?;
        let Some(tag) = tag.primary_tag() else {
            return Err(anyhow!("Tag not found"));
        };

        let title = tag.title().map(Cow::into_owned);
        let artist = tag.artist().map(Cow::into_owned);
        let album = tag.album().map(Cow::into_owned);

        let cover_art = tag
            .pictures()
            .into_iter()
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
        })
    }

    pub fn cover_art(&self) -> Result<slint::Image> {
        Ok(if let Some(ref cover_art) = self.cover_art {
            let bytes = &cover_art.bytes().collect::<std::io::Result<Vec<_>>>()?;
            slint::Image::from_rgba8(SharedPixelBuffer::clone_from_slice(
                bytes,
                cover_art.width(),
                cover_art.height(),
            ))
        } else {
            slint::Image::default()
        })
    }
}
