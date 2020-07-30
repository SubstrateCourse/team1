// Copyright 2017-2020 @polkadot/app-123code authors & contributors
// This software may be modified and distributed under the terms
// of the Apache-2.0 license. See the LICENSE file for details.

/* eslint-disable */

import { u32, Bytes, Struct, Option } from '@polkadot/types';

export interface Kitty extends Bytes {}
export interface KittyIndex extends u32 {}
export interface KittyLinkedItem extends Struct {
  prev: Option<KittyIndex>;
  next: Option<KittyIndex>;
}
