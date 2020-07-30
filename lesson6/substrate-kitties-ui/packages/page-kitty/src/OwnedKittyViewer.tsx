// Copyright 2017-2020 @polkadot/app-123code authors & contributors
// This software may be modified and distributed under the terms
// of the Apache-2.0 license. See the LICENSE file for details.
import React, { useState } from 'react';
import styled from 'styled-components';
import { withCalls } from '@polkadot/react-api/hoc';
import { Option } from '@polkadot/types';
import { Codec } from '@polkadot/types/types';
import { AccountId } from '@polkadot/types/interfaces';
import { useApi } from '@polkadot/react-hooks';
import { KittyLinkedItem, KittyIndex } from './types';
import KittyCard from './KittyCard';

const KittiesWrapper = styled.div`
  display: flex;
  flex-wrap: wrap;

  &:empty {
    &:after {
      content: "You don't have any kitties";
      margin-bottom: 15px;
      font-size: 1.2em;
    }
  }
`;

type OwnedKittiesKey = [AccountId, Option<KittyIndex>] & Codec;

type LinkedKittyCardProps = {
  accountKey: OwnedKittiesKey,
  item?: Option<KittyLinkedItem>,
  onNextOwnedKittiesKey: (key: OwnedKittiesKey) => void
};

const LinkedKittyCardComp = ({ accountKey, item, onNextOwnedKittiesKey }: LinkedKittyCardProps) => {
  const { api } = useApi();

  if (item && item.isSome) {
    const next = item.unwrap().next;

    if (next.isSome) {
      const nextKey = api.createType('(AccountId, Option<KittyIndex>)' as any, [accountKey[0], next]) as OwnedKittiesKey;

      setTimeout(() => onNextOwnedKittiesKey(nextKey));
    }
  }

  if (accountKey[1].isSome) {
    return <KittyCard
      accountId={accountKey[0].toString()}
      kittyId={accountKey[1].unwrap()}
      showUnlist={true}
    />;
  }

  return null;
};

const LinkedKittyCard = withCalls<LinkedKittyCardProps>(
  ['query.kitties.ownedKitties', { paramName: 'accountKey', propName: 'item' }]
)(LinkedKittyCardComp);

 type Props = {
   accountId: string | null,
 };

const OwnedKittyViewer: React.FC<Props> = ({ accountId }: Props) => {
  const [keys, setKeys] = useState<OwnedKittiesKey[]>([]);
  const { api } = useApi();

  const addKey = (i: number, key: OwnedKittiesKey): void => {
    const newKeys = [...keys];
    const old = newKeys[i];

    if (!key.eq(old)) {
      newKeys[i] = key;
      setKeys(newKeys);
    }
  };

  if (!accountId) {
    return <span />;
  }

  return (
    <div>
      <h2>My kitties</h2>
      <KittiesWrapper>
        <LinkedKittyCard
          accountKey={api.createType('(AccountId, Option<KittyIndex>)' as any, [accountId, null]) as OwnedKittiesKey}
          onNextOwnedKittiesKey={(key) => addKey(0, key)}
        />
        {keys.map((key, i) => (
          <LinkedKittyCard
            accountKey={key}
            key={key.toString()}
            onNextOwnedKittiesKey={(key) => addKey(i + 1, key)}
          />
        ))}
      </KittiesWrapper>
    </div>
  );
};

export default OwnedKittyViewer;
