// Copyright 2017-2020 @polkadot/app-123code authors & contributors
// This software may be modified and distributed under the terms
// of the Apache-2.0 license. See the LICENSE file for details.

import React from 'react';
import styled from 'styled-components';
import BN from 'bn.js';
import { withCalls } from '@polkadot/react-api/hoc';

import KittyCard from './KittyCard';
import OwnedKittyViewer from './OwnedKittyViewer';

const Wrapper = styled.section``;
const KittiesWrapper = styled.div`
  display: flex;
  flex-wrap: wrap;
`;

type Props = {
  kittiesCount?: BN,
  accountId: string | null,
};

const KittyViewer: React.FC<Props> = ({ accountId, kittiesCount }: Props) => {
  const count = kittiesCount ? kittiesCount.toNumber() : 0;
  const kitties = [];

  for (let i = 0; i < count; ++i) {
    kitties.push(
      <KittyCard
        accountId={accountId}
        key={i}
        kittyId={new BN(i)}
        showBuy={true}
      />
    );
  }

  return (
    <Wrapper>
      <h1>Substrate Kitties</h1>
      <OwnedKittyViewer
        accountId={accountId}
        key={accountId || ''}
      />
      <div>
        <h2>
           Total kitties count: {count}
        </h2>
        <KittiesWrapper>
          { kitties }
        </KittiesWrapper>
      </div>
    </Wrapper>
  );
};

export default withCalls<Props>(
  ['query.kitties.kittiesCount', { propName: 'kittiesCount' }]
)(KittyViewer);
