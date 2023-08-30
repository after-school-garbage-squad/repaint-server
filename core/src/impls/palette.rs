use async_trait::async_trait;
use repaint_server_usecase::infra::repo::{IsUpdated, PaletteRepository};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, TransactionTrait};

use crate::entity::{visitor_palettes, visitors};
use crate::{Error, SeaOrm};

use super::IsUpdatedExt;

#[async_trait]
impl PaletteRepository for SeaOrm {
    type Error = Error;

    async fn get(&self, visitor_id: i32) -> Result<Vec<i32>, Self::Error> {
        let Some(palettes) = visitors::Entity::find_by_id(visitor_id)
            .find_also_related(visitor_palettes::Entity)
            .one(self.con())
            .await?
            .and_then(|(_, p)| p)
        else {
            return Ok(Vec::new());
        };

        Ok(palettes.palette_id_list)
    }

    async fn set(&self, visitor_id: i32, palette: i32) -> Result<IsUpdated, Self::Error> {
        let tx = self.con().begin().await?;

        let palettes = visitors::Entity::find_by_id(visitor_id)
            .find_also_related(visitor_palettes::Entity)
            .one(&tx)
            .await?
            .and_then(|(_, p)| p)
            .unwrap();

        let p = palettes.palette_id_list.clone();
        let mut palettes: visitor_palettes::ActiveModel = palettes.into();
        palettes.palette_id_list = Set([p, vec![palette]].concat());
        let res = palettes.update(&tx).await;
        tx.commit().await?;

        res.to_is_updated()
    }
}
