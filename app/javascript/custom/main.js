import { connect, Contract, keyStores, WalletConnection, utils } from 'near-api-js';
import getConfig from './config.js';


const nearConfig = getConfig('development', 'zoo_marketplace.wabinab.testnet')
const near = await connect(Object.assign({ deps: { keyStore: new keyStores.BrowserLocalStorageKeyStore() } }, nearConfig));

const nearConfig2 = getConfig('development', 'zoo_nft.wabinab.testnet')
const near2 = await connect(Object.assign({ deps: { keyStore: new keyStores.BrowserLocalStorageKeyStore() } }, nearConfig));


window.nearConfig = nearConfig
window.near = near

window.nearConfig2 = nearConfig2
window.near2 = near2

window.walletConnection = new WalletConnection(near)

window.accountId = window.walletConnection.getAccountId()

window.contract = await new Contract(window.walletConnection.account(), nearConfig.contractName, {
  changeMethods: ['pay_and_mint_unsafe', 'generate_template'],
})

window.contract_nft = await new Contract(window.walletConnection.account(), nearConfig2.contractName, {
  changeMethods: ['nft_approve'],
})


function logout() {
  window.walletConnection.signOut()
  window.location.replace(window.location.origin + window.location.pathname)
}

function login() {
  window.walletConnection.requestSignIn(nearConfig.contractName)
}

function detect_path_name() {
  if (window.walletConnection.isSignedIn()) {
    // We don't have extra security login. If they login at browser, we assumed they have
    // at least partial access to their account; so we log them in. 
    window.location.replace(
      window.location.pathname + '?account_id=' + window.walletConnection.getAccountId()
    );
  } else {
    alert('Not signed in. Please login to your account.');
  }
}


function movie_ticket(token_id) {
  window.contract.pay_and_mint_unsafe(
    {
      "nft_contract_id": "zoo_nft.wabinab.testnet",
      "price": utils.format.parseNearAmount("1"),  // to be changed.
      "token_id": token_id,
      "issued_at": Math.floor(Date.now() / 1000),
    },
    "30000000000000",  // 30 TGas
    utils.format.parseNearAmount("1.1")
  ).then(
    window.location.replace(
      window.location.origin + "/cards/" + token_id
    )
  );
}


function zoo_ticket() {

}


function generate_template() {
    window.contract.generate_template(
      {
        "template_owner": window.walletConnection.getAccountId(),
        "template_id": template_id,
        "max_num_of_mint": max_num_of_mint,
        "metadata": {
          "title": title,
          "description": description,
          "media": media,
        },
        "size": size,
      },
      "30000000000000", // 30 TGas
      utils.format.parseNearAmount("0.1")
    ).then(
      window.location.reload()
    );
}





window.detect_path_name = detect_path_name
window.movie_ticket = movie_ticket
window.zoo_ticket = zoo_ticket
window.generate_template = generate_template
window.logout = logout
window.login = login