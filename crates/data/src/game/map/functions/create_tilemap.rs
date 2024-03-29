use crate::prelude::*;

pub fn create_tilemap_parent(commands: &mut Commands, name: &str) -> Entity {
    commands.spawn((Name::new(name.to_string()), SpatialBundle::default())).id()
}

pub fn create_tilemap<ZLevel: Into<f32>>(
    commands: &mut Commands,
    size: impl Size2d,
    z_level: ZLevel,
    tileset: &Tileset,
    tile_scale: f32,
) -> Entity {
    let entity = commands.spawn_empty().id();
    create_tilemap_on_entity(commands, entity, size, z_level, tileset, tile_scale);
    entity
}

pub fn create_tilemap_on_entity<ZLevel: Into<f32>>(
    commands: &mut Commands,
    entity: Entity,
    size: impl Size2d,
    z_level: ZLevel,
    tileset: &Tileset,
    tile_scale: f32,
) {
    let texture_handle = tileset.texture().clone();
    let tilemap_size = TilemapSize {
        x: size.width(),
        y: size.height(),
    };
    let tile_size = TilemapTileSize {
        x: tileset.tile_size().x,
        y: tileset.tile_size().y,
    };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Square;
    let mut tile_storage = TileStorage::empty(tilemap_size);
    for y in 0..tilemap_size.y {
        for x in 0..tilemap_size.x {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(entity),
                    texture_index: TileTextureIndex(0),
                    visible: TileVisible(false),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }
    commands.entity(entity).insert(TilemapBundle {
        texture: TilemapTexture::Single(texture_handle),
        size: tilemap_size,
        tile_size,
        grid_size,
        map_type,
        storage: tile_storage,
        transform: Transform {
            translation: Vec3 {
                x: 0.5,
                y: 0.5,
                z: z_level.into(),
            },
            scale: Vec3 {
                x: tile_scale / tile_size.x,
                y: tile_scale / tile_size.y,
                z: 1.0,
            },
            ..Default::default()
        },
        ..Default::default()
    });
}
