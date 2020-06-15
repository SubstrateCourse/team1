import { ApiPromise, WsProvider } from '@polkadot/api';

// config
const WEB_SOCKET = 'ws://localhost:9944';

async function connect() {
  // Construct
  const wsProvider = new WsProvider(WEB_SOCKET);
  const api = await ApiPromise.create({ provider: wsProvider });
  return api;
}

async function submitDocInfo(filePath, comment) {
  console.debug(`submitDocInfo: ${filePath}, ${comment}`);
  try {
    const api = await connect();

    // 学员们在这里追加逻辑

  } catch (err) {
    console.log(`Connect to Substrate error:`, err);
    process.exit(1);
  }

  process.exit(0);
}

async function getUserDocs(acct) {
  console.debug(`getUserDocs: ${acct}`);
  try {
    const api = await connect();

    // 学员们在这里追加逻辑

  } catch (err) {
    console.log(`Connect to Substrate error:`, err);
  }

  process.exit(0);
}

function main() {
  const args = process.argv.slice(2, 5);
  switch (args[0]) {
    case 'submitDocInfo':
      submitDocInfo(args[1], args[2])
      break;
    case 'getUserDocs':
      getUserDocs(args[1]);
      break;
    default:
      console.log('Unknown subcommand. Please use `submitDocInfo` or `getUserDocs` only.')
  }
}

main();
