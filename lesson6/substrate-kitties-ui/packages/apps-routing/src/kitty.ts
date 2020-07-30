// Copyright 2017-2020 @polkadot/apps-routing authors & contributors
// This software may be modified and distributed under the terms
// of the Apache-2.0 license. See the LICENSE file for details.

import { Route } from './types';

import Template from '@polkadot/app-kitty';

export default function create (t: <T = string> (key: string, text: string, options: { ns: string }) => T): Route {
  return {
    Component: Template,
    display: {
      isHidden: false,
      needsAccounts: true,
      needsApi: [
        'tx.kitties.create'
      ]
    },
    icon: 'th',
    name: 'Kitties',
    text: t<string>('nav.kitty', 'Kitties', { ns: 'apps-routing' })
  };
}
