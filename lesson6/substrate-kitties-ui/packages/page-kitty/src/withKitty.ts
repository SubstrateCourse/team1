// Copyright 2017-2020 @polkadot/app-123code authors & contributors
// This software may be modified and distributed under the terms
// of the Apache-2.0 license. See the LICENSE file for details.

/* eslint-disable camelcase */

import BN from 'bn.js';
import { Option } from '@polkadot/types/codec';
import { AccountId, Balance } from '@polkadot/types/interfaces';
import { withCalls } from '@polkadot/react-api/hoc';
import { Kitty } from './types';

export type Props = {
  kittyId: BN,
  kitty?: Option<Kitty>
  owner?: Option<AccountId>,
  price?: Option<Balance>,
};

export default withCalls<Props>(
  ['query.kitties.kitties', { paramName: 'kittyId', propName: 'kitty' }],
  ['query.kitties.kittyOwners', { paramName: 'kittyId', propName: 'owner' }],
  ['query.kitties.kittyPrices', { paramName: 'kittyId', propName: 'price' }]
);
