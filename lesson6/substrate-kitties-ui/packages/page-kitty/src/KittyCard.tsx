// Copyright 2017-2020 @polkadot/app-123code authors & contributors
// This software may be modified and distributed under the terms
// of the Apache-2.0 license. See the LICENSE file for details.

import React from 'react';
import styled from 'styled-components';
import { AddressMini, TxButton } from '@polkadot/react-components';
import { u8aToHex, formatBalance } from '@polkadot/util';

import KittyAvatar from './KittyAvatar';
import withKitty, { Props as WithKittyProps } from './withKitty';

const Wrapper = styled.div`
  border: 2px solid #eee;
  padding: 10px;
  border-radius: 8px;
  margin: 10px;
  width: 333px;
`;

const StyledKittyAvatar = styled(KittyAvatar)`
  margin: auto;
`;

const Line = styled.div`
  height: 2px;
  background: #eee;
  margin: 10px -10px;
`;

type Props = WithKittyProps & {
  showUnlist?: boolean,
  showBuy?: boolean,
  accountId: string | null,
};

const Price: React.FC<Props> = ({ accountId, kittyId, price, showBuy, showUnlist }: Props) => {
  if (price && price.isSome) {
    const value = price.unwrap();

    return (
      <>
        <label>Price: {formatBalance(value)}</label>
        {showUnlist &&
          <TxButton
            accountId={accountId}
            label='Unlist'
            params={[kittyId, null]}
            tx='kitties.ask'
          />
        }
        {showBuy &&
           <TxButton
             accountId={accountId}
             label='Buy'
             params={[kittyId, value]}
             tx='kitties.buy'
           />
        }
      </>
    );
  }

  return <label>Not for sale</label>;
};

const KittyCard: React.FC<Props> = ({ kitty, kittyId, owner, ...others }: Props) => {
  if (kitty && kitty.isSome) {
    const dna = kitty.unwrap().toU8a();

    return (
      <Wrapper>
        <StyledKittyAvatar dna={dna} />
        <Line />
        <label>Kitty ID: {kittyId.toString()}</label>
        <label>
           Owner:
          <AddressMini
            value={owner && owner.unwrap()}
          />
        </label>
        <label>DNA: {u8aToHex(dna)}</label>
        <Price {...{ kittyId, ...others }}/>
      </Wrapper>
    );
  }

  return <div>Loading...</div>;
};

export default withKitty(KittyCard as React.FC<WithKittyProps>) as React.ComponentType<Props>;
