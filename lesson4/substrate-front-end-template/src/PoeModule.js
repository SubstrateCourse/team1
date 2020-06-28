import React, { useEffect, useState } from 'react';
import { Form, Input, Grid, Card, Statistic, Button } from 'semantic-ui-react';

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
  const [note, setNote] = useState('');
  const [user, setUser] = useState('');
  const [showResults, setShowResults] = useState(false);
  const [reuslts, setResults] = useState([]);
  const [showNotification, setShowNotification] = useState(false);
  const [unsub, setUnsub] = useState(null);
  

  useEffect(() => {
    let unsubscribe;
    //debugger;
    api.query.poeModule.proofsHash(digest, (result) => {
      console.log(JSON.stringify(result))
      setOwner(result[0].toString());
      setBlockNumber(result[1].toNumber());
    }).then(unsub => {
      unsubscribe = unsub;
    }).catch(console.error);

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

  const setExtrinsicStatus = (data) => {
    console.log(data);
    //console.log(data.indexOf('Finalized'));
    if (data.indexOf('Finalized') !== -1) {
      setShowNotification(true);
      setTimeout(() => setShowNotification(false), 50000);
    }
    setStatus(data);
  }

  const Notification = (props) => {
    const {digest, note} = props;
    const notificationStyle = {
      marginTop: 10,
      border: '1px solid green',
      backgroundColor: 'lightgreen',
      color: 'darkgreen',
      borderRadius: 5,
      padding: 10
    };
    return (
      <div style={notificationStyle}>
         You have successfully claimed file with
         <div>hash <strong>{digest}</strong></div>
         <div>note <strong>"{note}"</strong></div>
         <div>Block Number is {blockNumber}, Owner is {owner}</div>
         <div>{status}</div>         
      </div>
    );
  }

  const convertToString = (hex) => {
    if (hex && hex.length) {
      return decodeURIComponent(hex.replace(/\s+/g, '').replace(/[0-9a-f]{2}/g, '%$&')).substr(2);
    }
    return '';
  };

  const handleClick = () => {
    
    unsub && unsub();
    api.query.poeModule.ownedProofsHashList(user, (result) => {
      setResults([]);
      if (result && result.length) {
        const docs = [];
        result.forEach((digest) => api.query.poeModule.proofsHash(digest.toString(), (res) => {
          var date = new Date(parseInt(res[2]))
          var datetime = date.toLocaleDateString() + ' ' + date.toLocaleTimeString()
          docs.push({
            digest: digest.toString(),
            blockNumber: res[1].toNumber(),
            // createTime: new Date(parseInt(res[2])),
            createTime: datetime,
            note: convertToString(res[3].toString())
          });
          if (docs.length === result.length) {
            setResults(docs);
            setShowResults(true);
            setTimeout(() => setShowResults(false), 20000);
          }
        }));
      } else {
        setShowResults(true);
        setTimeout(() => setShowResults(false), 5000);
      }
    }).then(unsub => setUnsub(unsub))
      .catch(console.error);

    setShowResults(true);
  }

  const UserDocs = (props) => {
    const userDocsStyle = {
      marginTop: 10,
      border: '1px solid green',
      backgroundColor: 'lightgreen',
      color: 'darkgreen',
      borderRadius: 5,
      padding: 10
    };
    const { docs } = props;
    if (docs && docs.length) {
      return (
        <div style={ userDocsStyle }>
          <ol>
            {docs.map((doc, index) => <li key={index}>{doc.digest} =&gt; ({doc.createTime}, {doc.note}, {doc.blockNumber})</li>)}
          </ol>
        </div>
      );
    } else {
      return (
        <div style={ userDocsStyle }>No docs found...</div>
      );
    }
  };

  return (
    <Grid.Column>  
        <Grid.Row width={8}>
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
            <Form.Field>
              <Input
                type = 'text'
                id = 'note'
                placeholder = 'Note'
                value = {note}
                onChange = {(e) => setNote(e.target.value)}
              />

            </Form.Field>
            <Form.Field>
              <TxButton
                accountPair = {accountPair}
                label = 'Create Claim With Note'
                setStatus = {setExtrinsicStatus}
                type='SIGNED-TX'
                attrs={{
                  palletRpc: 'poeModule',
                  callable: 'createClaimWithNote',
                  inputParams: [digest, note],
                  paramFields: [true, true]
                }}
              />
            </Form.Field>
            {showNotification && <Notification digest={digest} note={note}/>}
          </Form>
        </Grid.Row>
        <Grid.Row width={8}>
        <h1>Proof of Existence - User Info</h1>
          <Form>
            <Form.Field>
              <Input
                type = 'text'
                id = 'user_info'
                placeholder = '0xab12345'
                value = {user}
                onChange = {(e) => setUser(e.target.value)}
              />

            </Form.Field>
            <Form.Field>
              <Button onClick={handleClick}>Query Info</Button>
            </Form.Field>
            {showResults && <UserDocs docs={reuslts}/>}   
          </Form>
      </Grid.Row>
    </Grid.Column>
  );
}

export default function PoeModule (props) {
  const { api } = useSubstrate();
  return (api.query.poeModule && api.query.poeModule.proofs
    ? <Main {...props} /> : null);
}
