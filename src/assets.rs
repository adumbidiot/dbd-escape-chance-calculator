use anyhow::Context;
use image::gif::GifDecoder;
use image::AnimationDecoder;
use image::DynamicImage;
use std::io::Read;

const SLIPPERY_MEAT_GIF: &[u8] = include_bytes!("../resources/images/SlipperyMeat.gif");
const UP_THE_ANTE_GIF: &[u8] = include_bytes!("../resources/images/UpTheAnte.gif");

pub const PERK_ICON_SIZE: u16 = 100;

pub struct Assets {
    pub slippery_meat_perk_images: PerkImages,
    pub up_the_ante_perk_images: PerkImages,
}

impl Assets {
    pub fn new() -> anyhow::Result<Self> {
        let slippery_meat_perk_images = PerkImages::load_from_gif(SLIPPERY_MEAT_GIF)
            .context("failed to load slippery meat assets")?;

        let up_the_ante_perk_images = PerkImages::load_from_gif(UP_THE_ANTE_GIF)
            .context("failed to load up the ante assets")?;

        Ok(Self {
            slippery_meat_perk_images,
            up_the_ante_perk_images,
        })
    }
}

#[derive(Clone)]
pub struct PerkImages {
    pub tier_i: iced::image::Handle,
    pub tier_ii: iced::image::Handle,
    pub tier_iii: iced::image::Handle,
}

impl PerkImages {
    /// Load this from a GIF where each frame is a new tier of the perk.
    pub fn load_from_gif<R>(reader: R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let mut decoder = GifDecoder::new(reader)
            .context("failed to decode gif")?
            .into_frames();
        let tier_i = decoder
            .next()
            .context("missing tier i asset")?
            .context("failed to decode tier i asset")?
            .into_buffer();
        let tier_ii = decoder
            .next()
            .context("missing tier ii asset")?
            .context("failed to decode tier ii asset")?
            .into_buffer();
        let tier_iii = decoder
            .next()
            .context("missing tier iii asset")?
            .context("failed to decode tier iii asset")?
            .into_buffer();

        Ok(Self {
            tier_i: rgba_image_perk_to_iced_handle(tier_i),
            tier_ii: rgba_image_perk_to_iced_handle(tier_ii),
            tier_iii: rgba_image_perk_to_iced_handle(tier_iii),
        })
    }
}

fn rgba_image_perk_to_iced_handle(image: image::RgbaImage) -> iced::image::Handle {
    let bgra_image = DynamicImage::ImageRgba8(image)
        .resize(
            PERK_ICON_SIZE.into(),
            PERK_ICON_SIZE.into(),
            image::imageops::FilterType::Triangle,
        )
        .into_bgra8();
    iced::widget::image::Handle::from_pixels(
        bgra_image.width(),
        bgra_image.height(),
        bgra_image.into_raw(),
    )
}
