use super::{ClientState, EcsCompPacket, EcsResPacket};
use crate::{
    comp,
    terrain::{Block, TerrainChunk},
    ChatType,
};
use hashbrown::HashMap;
use vek::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RequestStateError {
    Denied,
    Already,
    Impossible,
    WrongMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub name: String,
    pub description: String,
    pub git_hash: String,
    pub git_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMsg {
    InitialSync {
        ecs_state: sphynx::StatePackage<EcsCompPacket, EcsResPacket>,
        entity_uid: u64,
        server_info: ServerInfo,
        // world_map: Vec2<usize>, /*, Vec<u32>)*/
    },
    StateAnswer(Result<ClientState, (RequestStateError, ClientState)>),
    ForceState(ClientState),
    Ping,
    Pong,
    ChatMsg {
        chat_type: ChatType,
        message: String,
    },
    SetPlayerEntity(u64),
    EcsSync(sphynx::SyncPackage<EcsCompPacket>),
    EcsResSync(sphynx::ResSyncPackage<EcsResPacket>),
    CreateEntity(sphynx::EntityPackage<EcsCompPacket>),
    DeleteEntity(u64),
    EntityPos {
        entity: u64,
        pos: comp::Pos,
    },
    EntityVel {
        entity: u64,
        vel: comp::Vel,
    },
    EntityOri {
        entity: u64,
        ori: comp::Ori,
    },
    EntityCharacterState {
        entity: u64,
        character_state: comp::CharacterState,
    },
    InventoryUpdate(comp::Inventory),
    TerrainChunkUpdate {
        key: Vec2<i32>,
        chunk: Result<Box<TerrainChunk>, ()>,
    },
    TerrainBlockUpdates(HashMap<Vec3<i32>, Block>),
    Error(ServerError),
    Disconnect,
    Shutdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerError {
    TooManyPlayers,
    InvalidAuth,
    //TODO: InvalidAlias,
}

impl ServerMsg {
    pub fn chat(message: String) -> ServerMsg {
        ServerMsg::ChatMsg {
            chat_type: ChatType::Chat,
            message,
        }
    }
    pub fn tell(message: String) -> ServerMsg {
        ServerMsg::ChatMsg {
            chat_type: ChatType::Tell,
            message,
        }
    }
    pub fn game(message: String) -> ServerMsg {
        ServerMsg::ChatMsg {
            chat_type: ChatType::GameUpdate,
            message,
        }
    }
    pub fn broadcast(message: String) -> ServerMsg {
        ServerMsg::ChatMsg {
            chat_type: ChatType::Broadcast,
            message,
        }
    }
    pub fn private(message: String) -> ServerMsg {
        ServerMsg::ChatMsg {
            chat_type: ChatType::Private,
            message,
        }
    }
    pub fn kill(message: String) -> ServerMsg {
        ServerMsg::ChatMsg {
            chat_type: ChatType::Kill,
            message,
        }
    }
}
