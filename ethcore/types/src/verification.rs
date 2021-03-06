// Copyright 2015-2019 Parity Technologies (UK) Ltd.
// This file is part of Parity Ethereum.

// Parity Ethereum is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Ethereum is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Ethereum.  If not, see <http://www.gnu.org/licenses/>.

//! Verification types

use crate::{
	header::Header,
	transaction::UnverifiedTransaction,
};
use bytes::Bytes;
use parity_util_mem::MallocSizeOf;

/// Verification queue status
#[derive(Debug, Clone)]
pub struct VerificationQueueInfo {
	/// Number of queued items pending verification
	pub unverified_queue_size: usize,
	/// Number of verified queued items pending import
	pub verified_queue_size: usize,
	/// Number of items being verified
	pub verifying_queue_size: usize,
	/// Configured maximum number of items in the queue
	pub max_queue_size: usize,
	/// Configured maximum number of bytes to use
	pub max_mem_use: usize,
	/// Heap memory used in bytes
	pub mem_used: usize,
}

impl VerificationQueueInfo {
	/// The total size of the queues.
	pub fn total_queue_size(&self) -> usize { self.unverified_queue_size + self.verified_queue_size + self.verifying_queue_size }

	/// Indicates that queue is full
	pub fn is_full(&self) -> bool {
		self.unverified_queue_size + self.verified_queue_size + self.verifying_queue_size > self.max_queue_size ||
			self.mem_used > self.max_mem_use
	}

	/// Indicates that queue is empty
	pub fn is_empty(&self) -> bool {
		self.unverified_queue_size + self.verified_queue_size + self.verifying_queue_size == 0
	}
}

/// An unverified block.
#[derive(Clone, PartialEq, Debug, MallocSizeOf)]
pub struct Unverified {
	/// Unverified block header.
	pub header: Header,
	/// Unverified block transactions.
	pub transactions: Vec<UnverifiedTransaction>,
	/// Unverified block uncles.
	pub uncles: Vec<Header>,
	/// Raw block bytes.
	pub bytes: Bytes,
}

impl Unverified {
	/// Create an `Unverified` from raw bytes.
	pub fn from_rlp(bytes: Bytes) -> Result<Self, rlp::DecoderError> {
		use rlp::Rlp;
		let (header, transactions, uncles) = {
			let rlp = Rlp::new(&bytes);
			let header = rlp.val_at(0)?;
			let transactions = rlp.list_at(1)?;
			let uncles = rlp.list_at(2)?;
			(header, transactions, uncles)
		};

		Ok(Unverified {
			header,
			transactions,
			uncles,
			bytes,
		})
	}
}

