use valence::client::despawn_disconnected_clients;
use valence::hand_swing::HandSwingEvent;
use valence::message::ChatMessageEvent;
use valence::network::ConnectionMode;
use valence::op_level::OpLevel;
use valence::prelude::*;
use valence::text::color::NamedColor;
use valence_creator::ValenceCreatorPlugin;

const SPAWN_Y: i32 = 64;

pub fn main() {
    App::new()
        .insert_resource(NetworkSettings {
            connection_mode: ConnectionMode::Offline,
            ..Default::default()
        })
        .add_plugins((DefaultPlugins.build(), ValenceCreatorPlugin))
        .add_systems(Startup, setup)
        .add_systems(EventLoopUpdate, (say_hello, change_op_level))
        .add_systems(Update, (init_clients, despawn_disconnected_clients))
        .run();
}

fn setup(
    mut commands: Commands,
    server: Res<Server>,
    biomes: Res<BiomeRegistry>,
    dimensions: Res<DimensionTypeRegistry>,
) {
    let mut layer = LayerBundle::new(ident!("overworld"), &dimensions, &biomes, &server);

    for z in -5..5 {
        for x in -5..5 {
            layer.chunk.insert_chunk([x, z], UnloadedChunk::new());
        }
    }

    for z in -25..25 {
        for x in -25..25 {
            layer
                .chunk
                .set_block([x, SPAWN_Y, z], BlockState::GRASS_BLOCK);
        }
    }

    commands.spawn(layer);
}

fn init_clients(
    mut clients: Query<
        (
            &mut EntityLayerId,
            &mut VisibleChunkLayer,
            &mut VisibleEntityLayers,
            &mut Position,
            &mut GameMode,
        ),
        Added<Client>,
    >,
    layers: Query<Entity, (With<ChunkLayer>, With<EntityLayer>)>,
) {
    for (
        mut layer_id,
        mut visible_chunk_layer,
        mut visible_entity_layers,
        mut pos,
        mut game_mode,
    ) in &mut clients
    {
        let layer = layers.single();

        layer_id.0 = layer;
        visible_chunk_layer.0 = layer;
        visible_entity_layers.0.insert(layer);
        pos.set([0.0, SPAWN_Y as f64 + 1.0, 0.0]);
        *game_mode = GameMode::Creative;
    }
}

// Add more systems here!

fn say_hello(mut hand_swing_events: EventReader<HandSwingEvent>, mut client_query: Query<&mut Client>) {
    for event in hand_swing_events.iter() {
        if let Ok(mut client) = client_query.get_mut(event.client) {
            client.send_chat_message("Hello, world!".color(NamedColor::DarkAqua));
        }
    }
}

fn change_op_level(mut chat_message_events: EventReader<ChatMessageEvent>,
    mut client_query: Query<(&mut Client, &mut OpLevel)>) {
    for event in chat_message_events.iter() {
        if let Ok((mut client, mut op_level)) = client_query.get_mut(event.client) {
            if let Ok(level) = event.message.parse::<u8>() {
                op_level.set(level);
                client.send_chat_message(format!("Set op level to {}", level).color(NamedColor::DarkAqua));
            }
        }
    }
}
