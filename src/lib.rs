
use valence::abilities::PlayerAbilitiesFlags;
use valence::ecs::query::Has;

use valence::event_loop::PacketEvent;
use valence::message::ChatMessageEvent;
use valence::op_level::OpLevel;
use valence::prelude::*;
use valence::protocol::packets::play::CommandExecutionC2s;
use valence::text::color::NamedColor;

pub struct ValenceCreatorPlugin;

impl Plugin for ValenceCreatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            EventLoopUpdate,
            (
                say_feur,
                set_creator_state,
                handle_creator_state.after(set_creator_state),
                handle_creator_state_selector.before(handle_creator_state),
            ),
        );
    }
}

#[derive(Component, Default)]
pub enum CreatorState {
    Build,
    Config,
    #[default]
    View,
    Test,
}

impl Into<GameMode> for &CreatorState {
    fn into(self) -> GameMode {
        match self {
            CreatorState::Build => GameMode::Creative,
            CreatorState::Config => GameMode::Adventure,
            CreatorState::View => GameMode::Spectator,
            CreatorState::Test => GameMode::Survival,
        }
    }
}

impl std::fmt::Display for CreatorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreatorState::Build => write!(f, "Build"),
            CreatorState::Config => write!(f, "Config"),
            CreatorState::View => write!(f, "View"),
            CreatorState::Test => write!(f, "Test"),
        }
    }
}

fn set_creator_state(
    mut commands: Commands,
    mut client_query: Query<(Entity, &mut Client, &OpLevel, Has<CreatorState>), Changed<OpLevel>>,
) {
    for (entity, mut client, op_level, has_creator_state) in client_query.iter_mut() {
        if op_level.get() < 3 {
            continue;
        }
        if has_creator_state {
            commands.entity(entity).remove::<CreatorState>();
        } else {
            commands.entity(entity).insert(CreatorState::default());
        }
    }
}

fn handle_creator_state(
    mut client_query: Query<(&mut Client, &CreatorState, &mut GameMode, &mut PlayerAbilitiesFlags), Changed<CreatorState>>,
) {
    for (mut client, creator_state, mut gamemode, mut abilities) in client_query.iter_mut() {
        *gamemode = creator_state.into();
        match *creator_state {
            CreatorState::Config => {
                abilities.set_allow_flying(true);
                abilities.set_flying(true);
            }
            _ => {}
        }
        client.send_chat_message(
            format!("CreatorState set to {}", creator_state).color(NamedColor::DarkAqua),
        );
    }
}

fn handle_creator_state_selector(
    mut packet_events: EventReader<PacketEvent>,
    mut client_query: Query<(&mut Client, &mut CreatorState)>,
) {
    for packet in packet_events.iter() {
        if let Ok((mut client, mut creator_state)) = client_query.get_mut(packet.client) {
            if let Some(command_execution_packet) = packet.decode::<CommandExecutionC2s>() {
                client.send_chat_message(
                    format!("Command: {}", command_execution_packet.command.0)
                        .color(NamedColor::DarkAqua),
                );
                match command_execution_packet.command.0 {
                    "gamemode creative" => {
                        *creator_state = CreatorState::Build;
                     }
                    "gamemode spectator" => {
                        *creator_state = CreatorState::View;
                    }
                    "gamemode adventure" => {
                        *creator_state = CreatorState::Config;
                   
                    }
                    "gamemode survival" => {
                        *creator_state = CreatorState::Test;
                     }
                    _ => {
                        *creator_state = CreatorState::View;
                    }
                }
            }
        }
    }
}

fn say_feur(
    mut chat_message_events: EventReader<ChatMessageEvent>,
    mut client_query: Query<(&mut Client, &OpLevel)>,
) {
    for event in chat_message_events.iter() {
        if let Ok((mut client, op_level)) = client_query.get_mut(event.client) {
            client.send_chat_message(format!("Chat: {:?}", event).color(NamedColor::DarkAqua));
            if op_level.get() >= 3 {
                client.send_chat_message("Hello, quoi!".color(NamedColor::DarkAqua));
            }
        }
    }
}
