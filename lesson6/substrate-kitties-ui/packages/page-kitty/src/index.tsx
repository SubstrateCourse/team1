// Copyright 2017-2020 @polkadot/app-123code authors & contributors
// This software may be modified and distributed under the terms
// of the Apache-2.0 license. See the LICENSE file for details.

// global app props
import { AppProps as Props } from '@polkadot/react-components/types';
import { registry } from '@polkadot/react-api';
import KittyViewer from './KittyViewer';
import KittyActions from './KittyActions';

// external imports (including those found in the packages/*
// of this repo)
import React, { useState } from 'react';

// local imports and components
import AccountSelector from './AccountSelector';

registry.register({
  Kitty: '[u8; 16]',
  KittyIndex: 'u32',
  KittyLinkedItem: {
    prev: 'Option<KittyIndex>',
    next: 'Option<KittyIndex>' // eslint-disable-line
  }
});

function TemplateApp ({ className }: Props): React.ReactElement<Props> {
  const [accountId, setAccountId] = useState<string | null>(null);

  return (
    // in all apps, the main wrapper is setup to allow the padding
    // and margins inside the application. (Just from a consistent pov)
    <main className={className}>
      <AccountSelector onChange={setAccountId} />
      <KittyActions accountId={accountId} />
      <KittyViewer accountId={accountId} />
    </main>
  );
}

export default React.memo(TemplateApp);
