#![warn(missing_debug_implementations)]
#![warn(unreachable_pub)]

use std::str::FromStr;

use async_trait::async_trait;
use chrono::Utc;
use firestore::errors::FirestoreError;
use firestore::{path, paths, FirestoreDb, FirestoreResult};
use futures::stream::BoxStream;
use futures::TryStreamExt;
use rand::distributions::Alphanumeric;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use repaint_server_model::event::Event;
use repaint_server_model::event_spot::EventSpot;
use repaint_server_model::id::Id;
use repaint_server_model::visitor::Visitor;
use repaint_server_usecase::infra::firestore::Firestore as FirestoreInfra;
use structure::TrafficLogStructure;
use teloc::dev::DependencyClone;
use tokio_stream::StreamExt;

use crate::structure::{
    AdminStructure, InitializeLogStructure, PaletteStructure, RegisterLogStructure,
    SpotLogStructure, VisitorLogStructure, VisitorStructure,
};

mod structure;

#[derive(Debug, Clone)]
pub struct Firestore {
    client: FirestoreDb,
}

impl Firestore {
    pub async fn new(project_id: String) -> Self {
        let client = FirestoreDb::new(&project_id)
            .await
            .expect("Failed to create Firestore client");

        Self { client }
    }
}

impl DependencyClone for Firestore {}

#[async_trait]
impl FirestoreInfra for Firestore {
    type Error = FirestoreError;

    async fn subscribe_palette(
        &self,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
        palette_id: i32,
    ) -> Result<(), Self::Error> {
        let structure = PaletteStructure {
            collection: format!("spot_{}", event_id),
            document: spot_id.to_string(),
            palette_id: Some(palette_id),
            palettes_ids: None,
        };
        let _ = self
            .client
            .fluent()
            .update()
            .fields(paths!(PaletteStructure::{palette_id}))
            .in_col(&structure.collection)
            .document_id(&structure.document)
            .object(&PaletteStructure {
                ..structure.clone()
            })
            .execute()
            .await?;

        Ok(())
    }

    async fn subscribe_palettes(
        &self,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
        palette_ids: Vec<i32>,
    ) -> Result<(), Self::Error> {
        let structure = PaletteStructure {
            collection: format!("spot_{}", event_id),
            document: spot_id.to_string(),
            palette_id: None,
            palettes_ids: Some(palette_ids),
        };
        let _ = self
            .client
            .fluent()
            .update()
            .fields(paths!(PaletteStructure::{palettes_ids}))
            .in_col(&structure.collection)
            .document_id(&structure.document)
            .object(&PaletteStructure {
                ..structure.clone()
            })
            .execute()
            .await?;

        Ok(())
    }

    async fn get_palette(
        &self,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
    ) -> Result<Option<i32>, Self::Error> {
        let structure = PaletteStructure {
            collection: format!("spot_{}", event_id),
            document: spot_id.to_string(),
            palette_id: None,
            palettes_ids: None,
        };
        let Some(res) = self
            .client
            .fluent()
            .select()
            .by_id_in(&structure.collection)
            .obj::<PaletteStructure>()
            .one(&structure.document)
            .await?
        else {
            return Ok(None);
        };
        let palette_id = res.palette_id;

        Ok(palette_id)
    }

    async fn get_palettes(
        &self,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
    ) -> Result<Option<Vec<i32>>, Self::Error> {
        let structure = PaletteStructure {
            collection: format!("spot_{}", event_id),
            document: spot_id.to_string(),
            palette_id: None,
            palettes_ids: None,
        };
        let Some(res) = self
            .client
            .fluent()
            .select()
            .by_id_in(&structure.collection)
            .obj::<PaletteStructure>()
            .one(&structure.document)
            .await?
        else {
            return Ok(None);
        };
        let palette_ids = res.palettes_ids;

        Ok(palette_ids)
    }

    async fn delete_spot(
        &self,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
    ) -> Result<(), Self::Error> {
        let structure = PaletteStructure {
            collection: format!("spot_{}", event_id),
            document: spot_id.to_string(),
            palette_id: None,
            palettes_ids: None,
        };
        let _ = self
            .client
            .fluent()
            .delete()
            .from(&structure.collection)
            .document_id(&structure.document)
            .execute()
            .await?;

        Ok(())
    }

    async fn subscribe_visitor(
        &self,
        event_id: Id<Event>,
        visitor_id: Id<Visitor>,
        spot_id: Id<EventSpot>,
    ) -> Result<(), Self::Error> {
        let structure = VisitorStructure {
            collection: format!("visitor_{}", event_id),
            document: visitor_id.to_string(),
            spot_id,
        };
        let _ = self
            .client
            .fluent()
            .update()
            .fields(paths!(VisitorStructure::{spot_id}))
            .in_col(&structure.collection)
            .document_id(&structure.document)
            .object(&VisitorStructure {
                ..structure.clone()
            })
            .execute()
            .await?;

        Ok(())
    }

    async fn get_visitors(
        &self,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
    ) -> Result<Vec<Id<Visitor>>, Self::Error> {
        let collection = format!("visitor_{}", event_id);
        let stream: BoxStream<FirestoreResult<VisitorStructure>> = self
            .client
            .fluent()
            .select()
            .fields(paths!(VisitorStructure::{document, spot_id}))
            .from(collection.as_str())
            .filter(|q| q.for_all([q.field(path!(VisitorStructure::spot_id)).eq(spot_id)]))
            .obj()
            .stream_query_with_errors()
            .await?;
        let res = stream
            .try_collect::<Vec<_>>()
            .await?
            .into_iter()
            .map(|v| Id::<Visitor>::from_str(&v.document).unwrap())
            .collect::<Vec<_>>();

        Ok(res)
    }

    async fn set_event_id(&self, token: String, event_id: i32) -> Result<(), Self::Error> {
        let structure = AdminStructure {
            collection: "admin".to_string(),
            document: token,
            event_id: Some(event_id),
        };
        let _ = self
            .client
            .fluent()
            .update()
            .fields(paths!(AdminStructure::{document, event_id}))
            .in_col(&structure.collection)
            .document_id(&structure.document)
            .object(&AdminStructure {
                ..structure.clone()
            })
            .execute()
            .await?;

        Ok(())
    }

    async fn get_event_id(&self, token: String) -> Result<Option<i32>, Self::Error> {
        let structure = AdminStructure {
            collection: "admin".to_string(),
            document: token,
            event_id: None,
        };
        let Some(res) = self
            .client
            .fluent()
            .select()
            .by_id_in(&structure.collection)
            .obj::<AdminStructure>()
            .one(&structure.document)
            .await?
        else {
            return Ok(None);
        };
        let event_id = res.event_id;

        Ok(event_id)
    }

    async fn subscribe_visitor_log(
        &self,
        event_id: Id<Event>,
        visitor_id: Id<Visitor>,
        spot_id: Id<EventSpot>,
        palettes_length: usize,
        took_photo: bool,
    ) -> Result<(), Self::Error> {
        let rng = {
            let rng = rand::thread_rng();
            StdRng::from_rng(rng).unwrap()
        };
        let s = rng
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect::<String>();
        let structure = VisitorLogStructure {
            collection: format!("event_log_{}", event_id),
            document: format!("visitor_log_{}_{}", Utc::now().timestamp(), s),
            visitor_id,
            spot_id,
            palettes_length,
            took_photo,
        };
        let _ = self
            .client
            .fluent()
            .insert()
            .into(&structure.collection)
            .document_id(&structure.document)
            .object(&structure)
            .execute()
            .await?;

        Ok(())
    }

    async fn subscribe_spot_log(
        &self,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
        head_count: usize,
    ) -> Result<(), Self::Error> {
        let rng = {
            let rng = rand::thread_rng();
            StdRng::from_rng(rng).unwrap()
        };
        let s = rng
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect::<String>();
        let structure = SpotLogStructure {
            collection: format!("event_log_{}", event_id),
            document: format!("spot_log_{}_{}", Utc::now().timestamp(), s),
            spot_id,
            head_count,
        };
        let _ = self
            .client
            .fluent()
            .insert()
            .into(&structure.collection)
            .document_id(&structure.document)
            .object(&structure)
            .execute()
            .await?;

        Ok(())
    }

    async fn subscribe_traffic_log(
        &self,
        event_id: Id<Event>,
        from: Id<EventSpot>,
        to: Id<EventSpot>,
    ) -> Result<(), Self::Error> {
        let rng = {
            let rng = rand::thread_rng();
            StdRng::from_rng(rng).unwrap()
        };
        let s = rng
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect::<String>();
        let structure = TrafficLogStructure {
            collection: format!("event_log_{}", event_id),
            document: format!("traffic_log_{}_{}", Utc::now().timestamp(), s),
            from,
            to,
        };
        let _ = self
            .client
            .fluent()
            .insert()
            .into(&structure.collection)
            .document_id(&structure.document)
            .object(&structure)
            .execute()
            .await?;

        Ok(())
    }

    async fn subscribe_register_log(
        &self,
        event_id: Id<Event>,
        visitor_id: Id<Visitor>,
    ) -> Result<(), Self::Error> {
        let rng = {
            let rng = rand::thread_rng();
            StdRng::from_rng(rng).unwrap()
        };
        let s = rng
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect::<String>();
        let structure = RegisterLogStructure {
            collection: format!("event_log_{}", event_id),
            document: format!("register_log_{}_{}", Utc::now().timestamp(), s),
            visitor_id,
        };
        let _ = self
            .client
            .fluent()
            .insert()
            .into(&structure.collection)
            .document_id(&structure.document)
            .object(&structure)
            .execute()
            .await?;

        Ok(())
    }

    async fn subscribe_initialize_log(
        &self,
        event_id: Id<Event>,
        visitor_id: Id<Visitor>,
    ) -> Result<(), Self::Error> {
        let rng = {
            let rng = rand::thread_rng();
            StdRng::from_rng(rng).unwrap()
        };
        let s = rng
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect::<String>();
        let structure = InitializeLogStructure {
            collection: format!("event_log_{}", event_id),
            document: format!("initialize_log_{}_{}", Utc::now().timestamp(), s),
            visitor_id,
        };
        let _ = self
            .client
            .fluent()
            .insert()
            .into(&structure.collection)
            .document_id(&structure.document)
            .object(&structure)
            .execute()
            .await?;

        Ok(())
    }

    async fn delete(&self, event_id: Id<Event>) -> Result<(), Self::Error> {
        let collection = format!("event_log_{}", event_id);
        let stream: BoxStream<_> = self
            .client
            .fluent()
            .list()
            .from(&collection)
            .stream_all()
            .await?;
        let docs = stream.collect::<Vec<_>>().await;
        for doc in docs {
            let _ = self
                .client
                .fluent()
                .delete()
                .from(&collection)
                .document_id(&doc.name)
                .execute()
                .await?;
        }

        Ok(())
    }
}
