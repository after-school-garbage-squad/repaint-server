#![warn(missing_debug_implementations)]
#![warn(unreachable_pub)]

use async_trait::async_trait;
use chrono::Utc;
use firestore::errors::FirestoreError;
use firestore::{paths, FirestoreDb};
use futures::stream::BoxStream;
use rand::distributions::Alphanumeric;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use repaint_server_model::event::Event;
use repaint_server_model::event_spot::EventSpot;
use repaint_server_model::id::Id;
use repaint_server_model::visitor::Visitor;
use repaint_server_usecase::infra::firestore::Firestore as FirestoreInfra;
use teloc::dev::DependencyClone;
use tokio_stream::StreamExt;
use tracing::info;

use crate::structure::{
    AdminStructure, InitializeLogStructure, PaletteStructure, RegisterLogStructure,
    SpotLogStructure, TrafficLogStructure, VisitorLogStructure,
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
        info!("initialized Firestore client");

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
        let collection = format!("spot_{}", event_id);
        let document = spot_id.to_string();
        let palette_ids = match self
            .client
            .fluent()
            .select()
            .by_id_in(collection.as_str())
            .obj::<PaletteStructure>()
            .one(document.clone())
            .await?
        {
            Some(p) => {
                let mut palette_ids = p.palette_ids;
                palette_ids.push(palette_id);

                palette_ids
            }
            None => vec![palette_id],
        };
        let structure = PaletteStructure { palette_ids };
        match self
            .client
            .fluent()
            .update()
            .fields(paths!(PaletteStructure::palette_ids))
            .in_col(collection.as_str())
            .document_id(document)
            .object(&structure)
            .execute::<()>()
            .await
        {
            Ok(_) => info!("subscribed palette"),
            Err(e) => return Err(e),
        }

        Ok(())
    }

    async fn unsubscribe_palette(
        &self,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
        palette_id: i32,
    ) -> Result<(), Self::Error> {
        let collection = format!("spot_{}", event_id);
        let document = spot_id.to_string();
        match self
            .client
            .fluent()
            .select()
            .by_id_in(collection.as_str())
            .obj::<PaletteStructure>()
            .one(document.clone())
            .await?
        {
            Some(p) => {
                let mut palette_ids = p.palette_ids;
                if let Some(i) = palette_ids.iter().position(|p| *p == palette_id) {
                    palette_ids.swap_remove(i);
                }
                let structure = PaletteStructure { palette_ids };
                match self
                    .client
                    .fluent()
                    .update()
                    .fields(paths!(PaletteStructure::palette_ids))
                    .in_col(collection.as_str())
                    .document_id(document)
                    .object(&structure)
                    .execute::<()>()
                    .await
                {
                    Ok(_) => info!("unsubscribed palette"),
                    Err(e) => return Err(e),
                }
            }
            None => {
                unreachable!("spot has no palettes");
            }
        }

        Ok(())
    }

    async fn get_palettes(
        &self,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
    ) -> Result<Vec<i32>, Self::Error> {
        let collection = format!("spot_{}", event_id);
        let document = spot_id.to_string();
        let Some(res) = self
            .client
            .fluent()
            .select()
            .by_id_in(collection.as_str())
            .obj::<PaletteStructure>()
            .one(document)
            .await?
        else {
            unreachable!("spot has no palettes")
        };
        info!("got palettes: {:?}", res.palette_ids);

        Ok(res.palette_ids)
    }

    async fn set_event_id(&self, token: String, event_id: i32) -> Result<(), Self::Error> {
        let collection = "admin".to_string();
        let document = token;
        let structure = AdminStructure {
            event_id: Some(event_id),
        };
        match self
            .client
            .fluent()
            .insert()
            .into(collection.as_str())
            .document_id(document)
            .object(&AdminStructure {
                ..structure.clone()
            })
            .execute::<()>()
            .await
        {
            Ok(_) => info!("set event id"),
            Err(e) => return Err(e),
        }

        Ok(())
    }

    async fn get_event_id(&self, token: String) -> Result<Option<i32>, Self::Error> {
        let collection = "admin".to_string();
        let document = token;
        let Some(res) = self
            .client
            .fluent()
            .select()
            .by_id_in(collection.as_str())
            .obj::<AdminStructure>()
            .one(document)
            .await?
        else {
            return Ok(None);
        };
        let event_id = res.event_id;
        info!("got event id: {:?}", event_id);

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
        let collection = format!("event_log_{}", event_id);
        let document = format!("visitor_log_{}_{}", Utc::now().timestamp(), s);
        let structure = VisitorLogStructure {
            visitor_id,
            spot_id,
            palettes_length,
            took_photo,
        };
        match self
            .client
            .fluent()
            .insert()
            .into(collection.as_str())
            .document_id(document)
            .object(&structure)
            .execute::<()>()
            .await
        {
            Ok(_) => info!("subscribed visitor log"),
            Err(e) => return Err(e),
        }

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
        let collection = format!("event_log_{}", event_id);
        let document = format!("spot_log_{}_{}", Utc::now().timestamp(), s);
        let structure = SpotLogStructure {
            spot_id,
            head_count,
        };
        match self
            .client
            .fluent()
            .insert()
            .into(collection.as_str())
            .document_id(document)
            .object(&structure)
            .execute::<()>()
            .await
        {
            Ok(_) => info!("subscribed spot log"),
            Err(e) => return Err(e),
        }

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
        let collection = format!("event_log_{}", event_id);
        let document = format!("traffic_log_{}_{}", Utc::now().timestamp(), s);
        let structure = TrafficLogStructure { from, to };
        match self
            .client
            .fluent()
            .insert()
            .into(collection.as_str())
            .document_id(document)
            .object(&structure)
            .execute::<()>()
            .await
        {
            Ok(_) => info!("subscribed traffic log"),
            Err(e) => return Err(e),
        }

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
        let collection = format!("event_log_{}", event_id);
        let document = format!("register_log_{}_{}", Utc::now().timestamp(), s);
        let structure = RegisterLogStructure { visitor_id };
        match self
            .client
            .fluent()
            .insert()
            .into(collection.as_str())
            .document_id(document)
            .object(&structure)
            .execute::<()>()
            .await
        {
            Ok(_) => info!("subscribed register log"),
            Err(e) => return Err(e),
        }

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
        let collection = format!("event_log_{}", event_id);
        let document = format!("initialize_log_{}_{}", Utc::now().timestamp(), s);
        let structure = InitializeLogStructure { visitor_id };
        match self
            .client
            .fluent()
            .insert()
            .into(collection.as_str())
            .document_id(document)
            .object(&structure)
            .execute::<()>()
            .await
        {
            Ok(_) => info!("subscribed initialize log"),
            Err(e) => return Err(e),
        }

        Ok(())
    }

    async fn delete(&self, event_id: Id<Event>) -> Result<(), Self::Error> {
        let collection = format!("event_log_{}", event_id);
        let stream: BoxStream<_> = self
            .client
            .fluent()
            .list()
            .from(collection.as_str())
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
        info!("deleted event log");

        Ok(())
    }
}
