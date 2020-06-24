import React, { useEffect, useState } from 'react';
import { Form, Input, Grid, Card, Statistic } from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';
import{ blake2AsHex } from '@polkadot/util-crypto';

function Main (props) {
  const { api } = useSubstrate();
  const { accountPair } = props;

  // The transaction submission status
  const [status, setStatus] = useState('');
  const [digest, setDigest] = useState('');
  const [owner, setOwner] = useState('');
  const [blockNumber, setBlockNumber] = useState(0);
  const [dest, setDest] = useState('');
  const [price, setPrice] = useState(0);
  

  useEffect(() => {
    let unsubscribe;
    //debugger;
    api.query.poeModule.proofs(digest, (result) => {
      console.log(JSON.stringify(result))
      setOwner(result[0].toString());
      setBlockNumber(result[1].toNumber());
    }).then(unsub => {
      unsubscribe = unsub;
    })
      .catch(console.error);

    return () => unsubscribe && unsubscribe();
  }, [digest, api.query.poeModule]);


  const handleFileChosen = (file) => {
    const fileReader = new FileReader();

    const bufferToDigest = () => {
      const content = Array.from(new Uint8Array(fileReader.result))
        .map((b) => b.toString(16).padStart(2, '0'))
        .join('');

      const hash = blake2AsHex(content, 256);

      setDigest(hash);
    };

    fileReader.onloadend = bufferToDigest;

    fileReader.readAsArrayBuffer(file);
  };

  return (
    <Grid.Column width={8}>
      <h1>Proof of Existence</h1>
      <Form>
        <Form.Field>
          <Input 
            type = 'file'
            id = 'file'
            lable = 'your file'
            onChange = {(e) => handleFileChosen(e.target.files[0])} 
          />
        </Form.Field>
        {/* <Form.Field>
          <Input
            type = 'text'
            id = 'price'
            lable = 'set your price'
            onChange = {(e) => setPrice(e.value)}
          />

        </Form.Field> */}
        <Form.Field>
          <Input
            type = 'text'
            id = 'dest'
            placeholder = 'destination account'
            value = {dest}
            onChange = {(e) => setDest(e.target.value)}
          />

        </Form.Field>
        <Form.Field>
          <TxButton
            accountPair = {accountPair}
            label = 'Create Claim'
            setStatus = {setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'createClaim',
              inputParams: [digest],
              paramFields: [true]
            }}
          />
          <TxButton
            accountPair = {accountPair}
            label = 'Revoke Claim'
            setStatus = {setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'revokeClaim',
              inputParams: [digest],
              paramFields: [true]
            }}
          />
          <TxButton
            accountPair = {accountPair}
            label = 'Transfer Claim'
            setStatus = {setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'transferClaim',
              inputParams: [digest, dest],
              paramFields: [true]
            }}
          />
          {/* <TxButton
            accountPair = {accountPair}
            label = 'Set price'
            setStatus = {setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'setPrice',
              inputParams: [digest, price],
              paramFields: [true]
            }}
          />
          <TxButton
            accountPair = {accountPair}
            label = 'Revoke Claim'
            setStatus = {setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'buyClaim',
              inputParams: [digest],
              paramFields: [true]
            }}
          /> */}
        </Form.Field>
          <div>{status}</div>
        <div>Block Number is {blockNumber}, Owner is {dest}</div>

      </Form>
    </Grid.Column>
  );
}

export default function PoeModule (props) {
  const { api } = useSubstrate();
  return (api.query.poeModule && api.query.poeModule.proofs
    ? <Main {...props} /> : null);
}
