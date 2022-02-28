import { createXcm, getApi, nextNonce, sovereignAccountOf } from '../../utils'
import { Command, CreateCommandParameters, program } from '@caporal/core'
import { ApiPromise, Keyring, WsProvider } from '@polkadot/api'

export default function ({ createCommand }: CreateCommandParameters): Command {
  return createCommand('accept hrmp channel from specific chain')
    .argument('<source>', 'paraId of source chain', {
      validator: program.NUMBER
    })
    .argument('<target>', 'paraId of target chain', {
      validator: program.NUMBER
    })
    .option('-r, --relay-ws [url]', 'The Relaychain API endpoint', {
      default: 'wss://kusama-rpc.polkadot.io'
    })
    .option('-p, --para-ws [url]', 'The Parachain API endpoint', {
      default: 'ws://127.0.0.1:9948'
    })
    .action(async actionParameters => {
      const {
        logger,
        args: { source, target },
        options: { relayWs, paraWs }
      } = actionParameters

      const encoded = await ApiPromise.create({
        provider: new WsProvider(relayWs.toString())
      })
        .then(api => api.tx.hrmp.hrmpAcceptOpenChannel(source.valueOf() as number).toHex())
        .then(hex => `0x${hex.slice(6)}`)
      const api = await getApi(paraWs.toString())
      const signer = new Keyring({ type: 'sr25519', ss58Format: 110 }).addFromUri('//Dave')
      await api.tx.sudo
        .sudo(
          api.tx.polkadotXcm.send(
            {
              V1: {
                parents: 1,
                interior: 'Here'
              }
            },
            createXcm(encoded, sovereignAccountOf(target.valueOf() as number))
          )
        )
        .signAndSend(signer, { nonce: await nextNonce(api, signer) })
        .then(() => process.exit(0))
        .catch(err => {
          logger.error(err.message)
          process.exit(1)
        })
    })
}
