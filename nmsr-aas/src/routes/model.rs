use actix_web::web::Bytes;
use crate::mojang_requests;
use crate::utils::errors::NMSRaaSError;
use crate::utils::Result;

#[derive(Debug, Clone)]
pub(crate) enum PlayerRenderInput {
    PlayerUuid(uuid::Uuid),
    TextureHash(String)
}

impl TryFrom<String> for PlayerRenderInput {
    type Error = NMSRaaSError;

    fn try_from(value: String) -> Result<PlayerRenderInput> {
        if value.len() == 32 || value.len() == 36 {
            let uuid = uuid::Uuid::parse_str(&value).map_err(NMSRaaSError::InvalidUUID)?;
            Ok(PlayerRenderInput::PlayerUuid(uuid))
        } else if value.len() > 36 {
            Ok(PlayerRenderInput::TextureHash(value))
        } else {
            Err(NMSRaaSError::InvalidPlayerRequest(value))
        }
    }
}

impl PlayerRenderInput {
    pub(crate) async fn get_skin_bytes(&self) -> Result<Bytes> {
        let hash = match self {
            PlayerRenderInput::PlayerUuid(id) => mojang_requests::get_skin_hash(*id).await?,
            PlayerRenderInput::TextureHash(hash) => hash.to_owned()
        };

        mojang_requests::get_skin_bytes(hash).await
    }
}