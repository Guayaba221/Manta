// Copyright 2020-2024 Manta Network.
// This file is part of Manta.
//
// Manta is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Manta is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Manta.  If not, see <http://www.gnu.org/licenses/>.

//! Inherents used for randomness
use codec::{Decode, Encode};
use sp_inherents::{Error, InherentData, InherentIdentifier, IsFatalError};
use sp_runtime::RuntimeString;

#[derive(Encode)]
#[cfg_attr(feature = "std", derive(Debug, Decode))]
/// Error type for missing mandatory inherent of pallet_randomness
pub enum InherentError {
    /// Takes an error explanation as string
    Other(RuntimeString),
}

impl IsFatalError for InherentError {
    fn is_fatal_error(&self) -> bool {
        match *self {
            InherentError::Other(_) => true,
        }
    }
}

impl InherentError {
    /// Try to create an instance ouf of the given identifier and data.
    #[cfg(feature = "std")]
    pub fn try_from(id: &InherentIdentifier, data: &[u8]) -> Option<Self> {
        if id == &INHERENT_IDENTIFIER {
            <InherentError as codec::Decode>::decode(&mut &*data).ok()
        } else {
            None
        }
    }
}

/// The InherentIdentifier to set the babe randomness results
pub const INHERENT_IDENTIFIER: InherentIdentifier = *b"baberand";

/// A bare minimum inherent data provider that provides no real data.
/// The inherent is simply used as a way to kick off some computation
/// until https://github.com/paritytech/substrate/pull/10128 lands.
pub struct InherentDataProvider;

#[cfg(feature = "std")]
#[async_trait::async_trait]
impl sp_inherents::InherentDataProvider for InherentDataProvider {
    async fn provide_inherent_data(&self, inherent_data: &mut InherentData) -> Result<(), Error> {
        inherent_data.put_data(INHERENT_IDENTIFIER, &())
    }

    async fn try_handle_error(
        &self,
        identifier: &InherentIdentifier,
        _error: &[u8],
    ) -> Option<Result<(), sp_inherents::Error>> {
        // Don't process modules from other inherents
        if *identifier != INHERENT_IDENTIFIER {
            return None;
        }

        // All errors with the randomness inherent are fatal
        Some(Err(Error::Application(Box::from(String::from(
            "Error processing dummy randomness inherent",
        )))))
    }
}
