//! Icon metadata generators and upstream parsers.
//!
//! This module provides:
//! - [`generator::IconsMetadataGenerator`] — build-time generator producing
//!   `phf::Map` constants for keywords and categories
//! - [`source::IconMetadataSource`] — trait for upstream metadata providers
//! - [`registry::IconMetadataRegistry`] — aggregator for all metadata sources
//! - [`fa::FaMetadata`] — Font Awesome parser (categories.yml + icons.yml)
//! - [`md::MdMetadata`] — Material Design parser (Google fonts metadata)
//! - [`devicon::DeviconMetadata`] — Devicon parser (devicon.json)
//! - [`octicons::OcticonsMetadata`] — Primer Octicons parser (keywords.json)

pub mod devicon;
pub mod fa;
pub mod generator;
pub mod mapping;
pub mod md;
pub mod octicons;
pub mod registry;
pub mod source;

pub use generator::IconsMetadataGenerator;
pub use registry::IconMetadataRegistry;
