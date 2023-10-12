use async_trait::async_trait;
use repaint_server_usecase::infra::repo::{IsUpdated, PaletteRepository};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DbErr, EntityTrait, TransactionTrait};

use crate::entity::{events, visitor_palettes, visitors};
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
            .and_then(|(_, p)| p);

        let p = match palettes.clone() {
            Some(p) => p.palette_id_list,
            None => Vec::new(),
        };

        match palettes {
            Some(palettes) => {
                let mut palettes: visitor_palettes::ActiveModel = palettes.into();
                palettes.palette_id_list = Set([p, vec![palette]].concat());
                let res = palettes.update(&tx).await;
                tx.commit().await?;

                res.to_is_updated()
            }
            None => {
                let palettes = visitor_palettes::ActiveModel {
                    visitor_id: Set(visitor_id),
                    palette_id_list: Set(vec![palette]),
                    ..Default::default()
                }
                .insert(&tx)
                .await;
                tx.commit().await?;

                palettes.to_is_updated()
            }
        }
    }

    async fn get_all(&self, event_id: i32) -> Result<Option<Vec<i32>>, Self::Error> {
        Ok(events::Entity::find_by_id(event_id)
            .one(self.con())
            .await?
            .map(|e| e.palettes))
    }

    async fn set_all(&self, event_id: i32, palette: Vec<i32>) -> Result<IsUpdated, Self::Error> {
        let tx = self.con().begin().await?;
        let mut event: events::ActiveModel = events::Entity::find_by_id(event_id)
            .one(&tx)
            .await?
            .ok_or(Error::SeaOrm(DbErr::RecordNotFound(format!(
                "events doesn't found with {}",
                event_id
            ))))?
            .into();
        event.palettes = Set(palette);
        let res = event.update(&tx).await;
        tx.commit().await?;

        res.to_is_updated()
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::*;

    use crate::TestingSeaOrm;

    use super::*;

    #[test_log::test(tokio::test)]
    async fn test_get_set() {
        let orm = TestingSeaOrm::new().await;
        let event = orm.make_test_event().await;
        let image_id = orm.make_test_default_image(event.id).await;
        let visitor = orm.make_test_visitor(event.id, image_id).await;

        let palette1 = PaletteRepository::get(orm.orm(), visitor.id).await.unwrap();
        let _ = PaletteRepository::set(orm.orm(), visitor.id, 1)
            .await
            .unwrap();
        let palette2 = PaletteRepository::get(orm.orm(), visitor.id).await.unwrap();
        for i in 2..6 {
            let _ = PaletteRepository::set(orm.orm(), visitor.id, i)
                .await
                .unwrap();
        }
        let palette3 = PaletteRepository::get(orm.orm(), visitor.id).await.unwrap();

        self::assert_eq!(palette1, Vec::<i32>::new());
        self::assert_eq!(palette2, [1]);
        self::assert_eq!(palette3, [1, 2, 3, 4, 5]);
    }

    #[test_log::test(tokio::test)]
    async fn test_get_set_all() {
        let orm = TestingSeaOrm::new().await;
        let event = orm.make_test_event().await;
        let res1 = PaletteRepository::get_all(orm.orm(), event.id)
            .await
            .unwrap();
        let _ = PaletteRepository::set_all(orm.orm(), event.id, vec![0, 1, 0])
            .await
            .unwrap();
        let res2 = PaletteRepository::get_all(orm.orm(), event.id)
            .await
            .unwrap();

        self::assert_eq!(res1, Some(vec![0, 0, 0]));
        self::assert_eq!(res2, Some(vec![0, 1, 0]));
    }
}
