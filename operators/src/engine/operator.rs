use std::path::PathBuf;

use super::{
    query_processor::{TypedRasterQueryProcessor, TypedVectorQueryProcessor},
    CloneableRasterOperator, CloneableVectorOperator, RasterResultDescriptor, ResultDescriptor,
    VectorResultDescriptor,
};
use crate::engine::query_processor::QueryProcessor;
use crate::error;
use crate::util::Result;

use serde::{Deserialize, Serialize};

/// Common methods for `RasterOperator`s
#[typetag::serde(tag = "type")]
pub trait RasterOperator: CloneableRasterOperator + Send + Sync + std::fmt::Debug {
    fn initialize(
        self: Box<Self>,
        context: &ExecutionContext,
    ) -> Result<Box<InitializedRasterOperator>>;

    /// Wrap a box around a `RasterOperator`
    fn boxed(self) -> Box<dyn RasterOperator>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
}

/// Common methods for `VectorOperator`s
#[typetag::serde(tag = "type")]
pub trait VectorOperator: CloneableVectorOperator + Send + Sync + std::fmt::Debug {
    fn initialize(
        self: Box<Self>,
        context: &ExecutionContext,
    ) -> Result<Box<InitializedVectorOperator>>;

    /// Wrap a box around a `VectorOperator`
    fn boxed(self) -> Box<dyn VectorOperator>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionContext {
    pub raster_data_root: PathBuf,
}

impl ExecutionContext {
    pub fn mock_empty() -> Self {
        ExecutionContext {
            raster_data_root: "".into(),
        }
    }
}

pub trait InitializedOperatorBase {
    type Descriptor: ResultDescriptor + Clone;

    /// Get the result descriptor of the `Operator`
    fn result_descriptor(&self) -> Self::Descriptor;

    /// Get the sources of the `Operator`
    fn raster_sources(&self) -> &[Box<InitializedRasterOperator>];

    /// Get the sources of the `Operator`
    fn vector_sources(&self) -> &[Box<InitializedVectorOperator>];

    /// Get the sources of the `Operator`
    fn raster_sources_mut(&mut self) -> &mut [Box<InitializedRasterOperator>];

    /// Get the sources of the `Operator`
    fn vector_sources_mut(&mut self) -> &mut [Box<InitializedVectorOperator>];
}

pub type InitializedVectorOperator =
    dyn InitializedOperator<VectorResultDescriptor, TypedVectorQueryProcessor>;

pub type InitializedRasterOperator =
    dyn InitializedOperator<RasterResultDescriptor, TypedRasterQueryProcessor>;

pub trait InitializedOperator<R, Q>: InitializedOperatorBase<Descriptor = R> + Send + Sync
where
    R: ResultDescriptor,
{
    /// Instantiate a `TypedVectorQueryProcessor` from a `RasterOperator`
    fn query_processor(&self) -> Result<Q>;

    /// Wrap a box around a `RasterOperator`
    fn boxed(self) -> Box<dyn InitializedOperator<R, Q>>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
}

impl<R> InitializedOperatorBase for Box<dyn InitializedOperatorBase<Descriptor = R>>
where
    R: ResultDescriptor + std::clone::Clone,
{
    type Descriptor = R;

    fn result_descriptor(&self) -> Self::Descriptor {
        self.as_ref().result_descriptor()
    }
    fn raster_sources(&self) -> &[Box<InitializedRasterOperator>] {
        self.as_ref().raster_sources()
    }
    fn vector_sources(&self) -> &[Box<InitializedVectorOperator>] {
        self.as_ref().vector_sources()
    }
    fn raster_sources_mut(&mut self) -> &mut [Box<InitializedRasterOperator>] {
        self.as_mut().raster_sources_mut()
    }
    fn vector_sources_mut(&mut self) -> &mut [Box<InitializedVectorOperator>] {
        self.as_mut().vector_sources_mut()
    }
}

impl<R, Q> InitializedOperatorBase for Box<dyn InitializedOperator<R, Q>>
where
    R: ResultDescriptor,
    Q: QueryProcessor,
{
    type Descriptor = R;
    fn result_descriptor(&self) -> Self::Descriptor {
        self.as_ref().result_descriptor()
    }
    fn raster_sources(&self) -> &[Box<InitializedRasterOperator>] {
        self.as_ref().raster_sources()
    }
    fn vector_sources(&self) -> &[Box<InitializedVectorOperator>] {
        self.as_ref().vector_sources()
    }
    fn raster_sources_mut(&mut self) -> &mut [Box<InitializedRasterOperator>] {
        self.as_mut().raster_sources_mut()
    }
    fn vector_sources_mut(&mut self) -> &mut [Box<InitializedVectorOperator>] {
        self.as_mut().vector_sources_mut()
    }
}

impl<R, Q> InitializedOperator<R, Q> for Box<dyn InitializedOperator<R, Q>>
where
    R: ResultDescriptor,
    Q: QueryProcessor,
{
    fn query_processor(&self) -> Result<Q> {
        self.as_ref().query_processor()
    }
}

/// An enum to differentiate between `Operator` variants
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "operator")]
pub enum TypedOperator {
    Vector(Box<dyn VectorOperator>),
    Raster(Box<dyn RasterOperator>),
}

impl TypedOperator {
    pub fn get_vector(self) -> Result<Box<dyn VectorOperator>> {
        if let TypedOperator::Vector(o) = self {
            return Ok(o);
        }
        Err(error::Error::InvalidOperatorType)
    }

    pub fn get_raster(self) -> Result<Box<dyn RasterOperator>> {
        if let TypedOperator::Raster(o) = self {
            return Ok(o);
        }
        Err(error::Error::InvalidOperatorType)
    }
}

impl Into<TypedOperator> for Box<dyn VectorOperator> {
    fn into(self) -> TypedOperator {
        TypedOperator::Vector(self)
    }
}

impl Into<TypedOperator> for Box<dyn RasterOperator> {
    fn into(self) -> TypedOperator {
        TypedOperator::Raster(self)
    }
}

/// An enum to differentiate between `InitializedOperator` variants
pub enum TypedInitializedOperator {
    Vector(Box<InitializedVectorOperator>),
    Raster(Box<InitializedRasterOperator>),
}

impl Into<TypedInitializedOperator> for Box<InitializedVectorOperator> {
    fn into(self) -> TypedInitializedOperator {
        TypedInitializedOperator::Vector(self)
    }
}

impl Into<TypedInitializedOperator> for Box<InitializedRasterOperator> {
    fn into(self) -> TypedInitializedOperator {
        TypedInitializedOperator::Raster(self)
    }
}
