use crate::prelude::*;

#[derive(Resource)]
pub struct MapManager {
    loaded_maps: HashMap<WorldPosition, Entity>,
}

impl MapManager {
    pub fn new() -> Self { Self { loaded_maps: HashMap::new() } }

    pub fn get_or_generate(
        &mut self,
        commands: &mut Commands,
        game_context: &mut ResMut<GameContext>,
        tilesets: &Tilesets,
        world_position: WorldPosition,
    ) -> AtrlResult<Entity> {
        if !game_context.is_valid_world_position(world_position) {
            return Err(AtrlError::InvalidWorldPosition(
                world_position.position.x,
                world_position.position.y,
                world_position.position.z,
            ));
        }

        if let Some(entity) = self.loaded_maps.get(&world_position) {
            return Ok(entity.clone());
        }

        // TODO: check deserialize map from world_position
        let map_seed = game_context.map_manager_random.prht.get(
            world_position.position.x,
            world_position.position.y,
            world_position.position.z,
        );
        let map_noise = game_context.map_manager_random.noise.get(
            world_position.position.x,
            world_position.position.y,
            world_position.position.z,
        );
        let map_noise = (map_noise + 1.0) * 0.5; // TODO: Verify noise.get() returns (-1, 1)
        let map_name = format!(
            "Map ({}, {}, {})",
            world_position.position.x, world_position.position.y, world_position.position.z
        );

        let mut map = Self::generate_map(&map_name, map_seed, world_position)?;

        let tileset_count = tilesets.len() as f64 - 1.0;
        let tileset_selection = (tileset_count * map_noise).round() as u8;
        let tileset = tilesets
            .get_by_id(&tileset_selection)
            .expect(format!("couldn't find tilemap_id: {:?}", tileset_selection).as_str());

        let terrain_layer = create_tilemap(
            commands,
            [GRID_WIDTH, GRID_HEIGHT],
            f32::from(MapLayer::Terrain),
            tileset,
            1.0,
        );
        let feature_layer = create_tilemap(
            commands,
            [GRID_WIDTH, GRID_HEIGHT],
            f32::from(MapLayer::Features),
            tileset,
            1.0,
        );
        let item_layer = create_tilemap(
            commands,
            [GRID_WIDTH, GRID_HEIGHT],
            f32::from(MapLayer::Items),
            tileset,
            1.0,
        );

        commands.entity(terrain_layer).insert(Name::new(format!(
            "TerrainLayer ({}, {}, {})",
            world_position.position.x, world_position.position.y, world_position.position.z
        )));
        commands.entity(feature_layer).insert(Name::new(format!(
            "FeatureLayer ({}, {}, {})",
            world_position.position.x, world_position.position.y, world_position.position.z
        )));
        commands.entity(item_layer).insert(Name::new(format!(
            "ItemLayer ({}, {}, {})",
            world_position.position.x, world_position.position.y, world_position.position.z
        )));

        map.terrain_layer_entity = Some(terrain_layer);
        map.feature_layer_entity = Some(feature_layer);
        map.item_layer_entity = Some(item_layer);

        let map_entity = commands
            .spawn(map)
            .insert(Name::new(format!(
                "Map ({}, {}, {})",
                world_position.position.x, world_position.position.y, world_position.position.z
            )))
            .insert(Transform::default())
            .insert(GlobalTransform::default())
            .insert(Visibility::default())
            .insert(ComputedVisibility::default())
            .id();

        commands.entity(map_entity).add_child(terrain_layer);
        commands.entity(map_entity).add_child(feature_layer);
        commands.entity(map_entity).add_child(item_layer);

        Ok(map_entity)
    }

    fn generate_map(name: &str, seed: u64, world_position: WorldPosition) -> AtrlResult<GameMap> {
        let mut random = Random::new(seed);
        let (start_x, start_y) = random_start_position(&mut random);
        let chain = BuilderChain::new([GRID_WIDTH, GRID_HEIGHT], random, world_position, name)
            .start_with(CellularAutomataArchitect::new())
            .with(RoomMapArchitect::new())
            .with(AreaStartingPosition::new(start_x, start_y))
            .generate();

        Ok(chain.get_map())
    }
}
