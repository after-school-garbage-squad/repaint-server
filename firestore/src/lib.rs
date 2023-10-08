#![warn(missing_debug_implementations)]
#![warn(unreachable_pub)]

use std::str::FromStr;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use firestore::errors::FirestoreError;
use firestore::{
    path, paths, FirestoreDb, FirestoreQueryDirection, FirestoreResult, FirestoreTimestamp,
};
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
use serde::Deserialize;
use teloc::dev::DependencyClone;
use tokio_stream::StreamExt;
use tracing::info;

use crate::structure::{
    AdminStructure, InitializeLogStructure, PaletteStructure, RegisterLogStructure,
    SpotLogStructure, TrafficLogStructure, TrafficStructure, VisitorLogStructure, VisitorStructure,
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
        let structure = PaletteStructure {
            palette_id: Some(palette_id),
            palettes_ids: None,
        };
        match self
            .client
            .fluent()
            .update()
            .fields(paths!(PaletteStructure::{palette_id}))
            .in_col(collection.as_str())
            .document_id(document)
            .object(&PaletteStructure {
                ..structure.clone()
            })
            .execute::<()>()
            .await
        {
            Ok(_) => info!("subscribed palette"),
            Err(e) => return Err(e),
        }

        Ok(())
    }

    async fn subscribe_palettes(
        &self,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
        palette_ids: Vec<i32>,
    ) -> Result<(), Self::Error> {
        let collection = format!("spot_{}", event_id);
        let document = spot_id.to_string();
        let structure = PaletteStructure {
            palette_id: None,
            palettes_ids: Some(palette_ids),
        };
        match self
            .client
            .fluent()
            .update()
            .fields(paths!(PaletteStructure::{palettes_ids}))
            .in_col(collection.as_str())
            .document_id(document)
            .object(&PaletteStructure {
                ..structure.clone()
            })
            .execute::<()>()
            .await
        {
            Ok(_) => info!("subscribed palettes"),
            Err(e) => return Err(e),
        }

        Ok(())
    }

    async fn get_palette(
        &self,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
    ) -> Result<Option<i32>, Self::Error> {
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
            return Ok(None);
        };
        let palette_id = res.palette_id;
        info!("got palette: {:?}", palette_id);

        Ok(palette_id)
    }

    async fn get_palettes(
        &self,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
    ) -> Result<Option<Vec<i32>>, Self::Error> {
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
            return Ok(None);
        };
        let palette_ids = res.palettes_ids;
        info!("got palettes: {:?}", palette_ids);

        Ok(palette_ids)
    }

    async fn delete_spot(
        &self,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
    ) -> Result<(), Self::Error> {
        let collection = format!("spot_{}", event_id);
        let document = spot_id.to_string();
        match self
            .client
            .fluent()
            .delete()
            .from(collection.as_str())
            .document_id(document)
            .execute()
            .await
        {
            Ok(_) => info!("deleted spot"),
            Err(e) => return Err(e),
        }

        Ok(())
    }

    async fn subscribe_visitor(
        &self,
        event_id: Id<Event>,
        visitor_id: Id<Visitor>,
        spot_id: Id<EventSpot>,
    ) -> Result<(), Self::Error> {
        let collection = format!("visitor_{}", event_id);
        let document = visitor_id.to_string();
        let structure = VisitorStructure { spot_id };
        match self
            .client
            .fluent()
            .update()
            .fields(paths!(VisitorStructure::{spot_id}))
            .in_col(collection.as_str())
            .document_id(document)
            .object(&VisitorStructure {
                ..structure.clone()
            })
            .execute::<()>()
            .await
        {
            Ok(_) => info!("subscribed visitor"),
            Err(e) => return Err(e),
        }

        Ok(())
    }

    async fn get_visitors(
        &self,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
    ) -> Result<Vec<Id<Visitor>>, Self::Error> {
        let collection = format!("visitor_{}", event_id);
        #[derive(Debug, Deserialize)]
        struct Res {
            document: String,
        }
        let stream: BoxStream<FirestoreResult<Res>> = self
            .client
            .fluent()
            .select()
            .fields(vec!["document", "spot_id"])
            .from(collection.as_str())
            .filter(|q| q.for_all([q.field(path!(VisitorStructure::spot_id)).eq(spot_id)]))
            .obj()
            .stream_query_with_errors()
            .await?;
        let res = stream
            .try_collect::<Vec<_>>()
            .await?
            .into_iter()
            .map(|v| Id::<Visitor>::from_str(v.document.as_str()).unwrap())
            .collect::<Vec<_>>();
        info!("got visitors: {:?}", res);

        Ok(res)
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

    async fn size_traffic_queue(&self, event_id: Id<Event>) -> Result<usize, Self::Error> {
        let collection = format!("traffic_{}", event_id);
        let stream: BoxStream<_> = self
            .client
            .fluent()
            .list()
            .from(&collection)
            .stream_all()
            .await?;
        let docs = stream.collect::<Vec<_>>().await;
        let size = docs.len();
        info!("got traffic queue size: {:?}", size);

        Ok(size)
    }

    async fn push_traffic_queue(
        &self,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
    ) -> Result<(), Self::Error> {
        let collection = format!("traffic_{}", event_id);
        let document = spot_id.to_string();
        let structure = TrafficStructure {
            timestamp: FirestoreTimestamp(Utc::now()),
        };
        match self
            .client
            .fluent()
            .update()
            .fields(vec!["timestamp"])
            .in_col(collection.as_str())
            .document_id(document)
            .object(&TrafficStructure {
                ..structure.clone()
            })
            .execute::<()>()
            .await
        {
            Ok(_) => info!("pushed traffic queue"),
            Err(e) => return Err(e),
        }

        Ok(())
    }

    async fn pop_traffic_queue(
        &self,
        event_id: Id<Event>,
    ) -> Result<Option<Id<EventSpot>>, Self::Error> {
        let collection = format!("traffic_{}", event_id);
        #[derive(Debug, Deserialize)]
        struct Res {
            document: String,
        }
        let stream: BoxStream<FirestoreResult<Res>> = self
            .client
            .fluent()
            .select()
            .fields(vec!["document", "timestamp"])
            .from(collection.as_str())
            .order_by([(
                path!(TrafficStructure::timestamp),
                FirestoreQueryDirection::Ascending,
            )])
            .obj()
            .stream_query_with_errors()
            .await?;
        let res = stream
            .try_collect::<Vec<_>>()
            .await?
            .into_iter()
            .map(|v| Id::<EventSpot>::from_str(v.document.as_str()).unwrap())
            .collect::<Vec<_>>();
        match res.first() {
            Some(spot_id) => {
                let _ = self
                    .client
                    .fluent()
                    .delete()
                    .from(collection.as_str())
                    .document_id(spot_id.to_string())
                    .execute()
                    .await?;
                info!("popped traffic queue: {:?}", spot_id);

                Ok(Some(spot_id.clone()))
            }
            None => {
                info!("popped traffic queue: None");

                Ok(None)
            }
        }
    }

    async fn remove_traffic_queue(
        &self,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
    ) -> Result<(), Self::Error> {
        let collection = format!("traffic_{}", event_id);
        let document = spot_id.to_string();
        match self
            .client
            .fluent()
            .delete()
            .from(collection.as_str())
            .document_id(document)
            .execute()
            .await
        {
            Ok(_) => info!("removed traffic queue"),
            Err(e) => return Err(e),
        }

        Ok(())
    }

    async fn get_traffic_timestamp(
        &self,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
    ) -> Result<Option<DateTime<Utc>>, Self::Error> {
        let collection = format!("traffic_{}", event_id);
        let document = spot_id.to_string();
        let Some(res) = self
            .client
            .fluent()
            .select()
            .by_id_in(collection.as_str())
            .obj::<TrafficStructure>()
            .one(document)
            .await?
        else {
            return Ok(None);
        };

        Ok(Some(res.timestamp.0))
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
        info!("deleted event log");

        Ok(())
    }
}
