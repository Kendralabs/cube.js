use std::{any::Any, sync::Arc};

use async_trait::async_trait;

use crate::arrow::array::Int16Builder;
use datafusion::{
    arrow::{
        array::{Array, ArrayRef, BooleanBuilder, Int32Builder, ListBuilder, StringBuilder},
        datatypes::{DataType, Field, Schema, SchemaRef},
        record_batch::RecordBatch,
    },
    datasource::{datasource::TableProviderFilterPushDown, TableProvider, TableType},
    error::DataFusionError,
    logical_plan::Expr,
    physical_plan::{memory::MemoryExec, ExecutionPlan},
};

struct PgCatalogConstraintBuilder {
    oid: Int32Builder,
    conname: StringBuilder,
    connamespace: Int32Builder,
    contype: StringBuilder,
    condeferrable: BooleanBuilder,
    condeferred: BooleanBuilder,
    convalidated: BooleanBuilder,
    conrelid: Int32Builder,
    contypid: Int32Builder,
    conindid: Int32Builder,
    conparentid: Int32Builder,
    confrelid: Int32Builder,
    confupdtype: StringBuilder,
    confdeltype: StringBuilder,
    confmatchtype: StringBuilder,
    conislocal: BooleanBuilder,
    coninhcount: Int32Builder,
    connoinherit: BooleanBuilder,
    conkey: ListBuilder<Int16Builder>,
    confkey: ListBuilder<Int16Builder>,
    conpfeqop: ListBuilder<Int32Builder>,
    conppeqop: ListBuilder<Int32Builder>,
    conffeqop: ListBuilder<Int32Builder>,
    conexclop: ListBuilder<Int32Builder>,
    // TODO: type pg_node_tree?
    conbin: StringBuilder,
}

impl PgCatalogConstraintBuilder {
    fn new() -> Self {
        let capacity = 10;

        Self {
            oid: Int32Builder::new(capacity),
            conname: StringBuilder::new(capacity),
            connamespace: Int32Builder::new(capacity),
            contype: StringBuilder::new(capacity),
            condeferrable: BooleanBuilder::new(capacity),
            condeferred: BooleanBuilder::new(capacity),
            convalidated: BooleanBuilder::new(capacity),
            conrelid: Int32Builder::new(capacity),
            contypid: Int32Builder::new(capacity),
            conindid: Int32Builder::new(capacity),
            conparentid: Int32Builder::new(capacity),
            confrelid: Int32Builder::new(capacity),
            confupdtype: StringBuilder::new(capacity),
            confdeltype: StringBuilder::new(capacity),
            confmatchtype: StringBuilder::new(capacity),
            conislocal: BooleanBuilder::new(capacity),
            coninhcount: Int32Builder::new(capacity),
            connoinherit: BooleanBuilder::new(capacity),
            conkey: ListBuilder::new(Int16Builder::new(capacity)),
            confkey: ListBuilder::new(Int16Builder::new(capacity)),
            conpfeqop: ListBuilder::new(Int32Builder::new(capacity)),
            conppeqop: ListBuilder::new(Int32Builder::new(capacity)),
            conffeqop: ListBuilder::new(Int32Builder::new(capacity)),
            conexclop: ListBuilder::new(Int32Builder::new(capacity)),
            conbin: StringBuilder::new(capacity),
        }
    }

    fn finish(mut self) -> Vec<Arc<dyn Array>> {
        let mut columns: Vec<Arc<dyn Array>> = vec![];

        columns.push(Arc::new(self.oid.finish()));
        columns.push(Arc::new(self.conname.finish()));
        columns.push(Arc::new(self.connamespace.finish()));
        columns.push(Arc::new(self.contype.finish()));
        columns.push(Arc::new(self.condeferrable.finish()));
        columns.push(Arc::new(self.condeferred.finish()));
        columns.push(Arc::new(self.convalidated.finish()));
        columns.push(Arc::new(self.conrelid.finish()));
        columns.push(Arc::new(self.contypid.finish()));
        columns.push(Arc::new(self.conindid.finish()));
        columns.push(Arc::new(self.conparentid.finish()));
        columns.push(Arc::new(self.confrelid.finish()));
        columns.push(Arc::new(self.confupdtype.finish()));
        columns.push(Arc::new(self.confdeltype.finish()));
        columns.push(Arc::new(self.confmatchtype.finish()));
        columns.push(Arc::new(self.conislocal.finish()));
        columns.push(Arc::new(self.coninhcount.finish()));
        columns.push(Arc::new(self.connoinherit.finish()));
        columns.push(Arc::new(self.conkey.finish()));
        columns.push(Arc::new(self.confkey.finish()));
        columns.push(Arc::new(self.conpfeqop.finish()));
        columns.push(Arc::new(self.conppeqop.finish()));
        columns.push(Arc::new(self.conffeqop.finish()));
        columns.push(Arc::new(self.conexclop.finish()));
        columns.push(Arc::new(self.conbin.finish()));

        columns
    }
}

pub struct PgCatalogConstraintProvider {
    data: Arc<Vec<ArrayRef>>,
}

impl PgCatalogConstraintProvider {
    pub fn new() -> Self {
        let builder = PgCatalogConstraintBuilder::new();

        Self {
            data: Arc::new(builder.finish()),
        }
    }
}

#[async_trait]
impl TableProvider for PgCatalogConstraintProvider {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn table_type(&self) -> TableType {
        TableType::View
    }

    fn schema(&self) -> SchemaRef {
        Arc::new(Schema::new(vec![
            Field::new("oid", DataType::Int32, false),
            Field::new("conname", DataType::Utf8, false),
            Field::new("connamespace", DataType::Int32, false),
            Field::new("contype", DataType::Utf8, false),
            Field::new("condeferrable", DataType::Boolean, false),
            Field::new("condeferred", DataType::Boolean, false),
            Field::new("convalidated", DataType::Boolean, false),
            Field::new("conrelid", DataType::Int32, false),
            Field::new("contypid", DataType::Int32, false),
            Field::new("conindid", DataType::Int32, false),
            Field::new("conparentid", DataType::Int32, false),
            Field::new("confrelid", DataType::Int32, false),
            Field::new("confupdtype", DataType::Utf8, false),
            Field::new("confdeltype", DataType::Utf8, false),
            Field::new("confmatchtype", DataType::Utf8, false),
            Field::new("conislocal", DataType::Boolean, false),
            Field::new("coninhcount", DataType::Int32, false),
            Field::new("connoinherit", DataType::Boolean, false),
            Field::new(
                "conkey",
                DataType::List(Box::new(Field::new("item", DataType::Int16, true))),
                true,
            ),
            Field::new(
                "confkey",
                DataType::List(Box::new(Field::new("item", DataType::Int16, true))),
                true,
            ),
            Field::new(
                "conpfeqop",
                DataType::List(Box::new(Field::new("item", DataType::Int32, true))),
                true,
            ),
            Field::new(
                "conppeqop",
                DataType::List(Box::new(Field::new("item", DataType::Int32, true))),
                true,
            ),
            Field::new(
                "conffeqop",
                DataType::List(Box::new(Field::new("item", DataType::Int32, true))),
                true,
            ),
            Field::new(
                "conexclop",
                DataType::List(Box::new(Field::new("item", DataType::Int32, true))),
                true,
            ),
            Field::new("conbin", DataType::Utf8, true),
        ]))
    }

    async fn scan(
        &self,
        projection: &Option<Vec<usize>>,
        _filters: &[Expr],
        _limit: Option<usize>,
    ) -> Result<Arc<dyn ExecutionPlan>, DataFusionError> {
        let batch = RecordBatch::try_new(self.schema(), self.data.to_vec())?;

        Ok(Arc::new(MemoryExec::try_new(
            &[vec![batch]],
            self.schema(),
            projection.clone(),
        )?))
    }

    fn supports_filter_pushdown(
        &self,
        _filter: &Expr,
    ) -> Result<TableProviderFilterPushDown, DataFusionError> {
        Ok(TableProviderFilterPushDown::Unsupported)
    }
}
