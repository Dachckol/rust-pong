use amethyst::{
    prelude::*,
    assets::{Handle, AssetStorage,Loader},
    renderer::{
        PngFormat,
        SpriteSheetHandle,
        Texture,
        TextureMetadata,
        SpriteSheet,
        SpriteSheetFormat,
    },
    ui::{TtfFormat, FontAsset},
};


pub fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "textures/spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "textures/spritesheet.ron",
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store,
    )
}

pub fn load_font(world: &mut World) -> Handle<FontAsset> {
    world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        Default::default(),
        (),
        &world.read_resource(),
    )
}
