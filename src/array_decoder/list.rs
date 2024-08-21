// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use std::sync::Arc;

use arrow::array::{ArrayRef, ListArray};
use arrow::buffer::{NullBuffer, OffsetBuffer};
use arrow::datatypes::{Field, FieldRef};
use snafu::ResultExt;

use crate::array_decoder::{derive_present_vec, populate_lengths_with_nulls};
use crate::column::{get_present_vec, Column};
use crate::encoding::get_unsigned_rle_reader;
use crate::proto::stream::Kind;

use crate::error::{ArrowSnafu, Result};
use crate::stripe::Stripe;

use super::{array_decoder_factory, ArrayBatchDecoder};

pub struct ListArrayDecoder {
    inner: Box<dyn ArrayBatchDecoder>,
    present: Option<Box<dyn Iterator<Item = bool> + Send>>,
    lengths: Box<dyn Iterator<Item = Result<i64>> + Send>,
    field: FieldRef,
}

impl ListArrayDecoder {
    pub fn new(column: &Column, field: Arc<Field>, stripe: &Stripe) -> Result<Self> {
        let present = get_present_vec(column, stripe)?
            .map(|iter| Box::new(iter.into_iter()) as Box<dyn Iterator<Item = bool> + Send>);

        let child = &column.children()[0];
        let inner = array_decoder_factory(child, field.clone(), stripe)?;

        let reader = stripe.stream_map().get(column, Kind::Length);
        let lengths = get_unsigned_rle_reader(column, reader);

        Ok(Self {
            inner,
            present,
            lengths,
            field,
        })
    }
}

impl ArrayBatchDecoder for ListArrayDecoder {
    fn next_batch(
        &mut self,
        batch_size: usize,
        parent_present: Option<&[bool]>,
    ) -> Result<ArrayRef> {
        let present = derive_present_vec(&mut self.present, parent_present, batch_size);

        // How many lengths we need to fetch
        let elements_to_fetch = if let Some(present) = &present {
            present.iter().filter(|&&is_present| is_present).count()
        } else {
            batch_size
        };
        let lengths = self
            .lengths
            .by_ref()
            .take(elements_to_fetch)
            .collect::<Result<Vec<_>>>()?;
        debug_assert_eq!(
            lengths.len(),
            elements_to_fetch,
            "less lengths than expected in ListArray"
        );
        let total_length: i64 = lengths.iter().sum();
        // Fetch child array as one Array with total_length elements
        let child_array = self.inner.next_batch(total_length as usize, None)?;
        let lengths = populate_lengths_with_nulls(lengths, batch_size, &present);
        let offsets = OffsetBuffer::from_lengths(lengths);
        let null_buffer = present.map(NullBuffer::from);

        let array = ListArray::try_new(self.field.clone(), offsets, child_array, null_buffer)
            .context(ArrowSnafu)?;
        let array = Arc::new(array);
        Ok(array)
    }
}
