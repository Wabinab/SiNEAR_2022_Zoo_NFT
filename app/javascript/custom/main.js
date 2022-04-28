import { connect, Contract, keyStores, WalletConnection } from 'near-api-js';
import getConfig from './config.js';


const nearConfig = getConfig('development', 'greeter.wabinab.testnet')
const near = await connect(Object.assign({ deps: { keyStore: new keyStores.BrowserLocalStorageKeyStore() } }, nearConfig));

window.nearConfig = nearConfig
window.near = near

window.walletConnection = new WalletConnection(near)

window.accountId = window.walletConnection.getAccountId()

window.contract = await new Contract(window.walletConnection.account(), nearConfig.contractName, {
  changeMethods: ['set_greeting', 'set_greeting_for_others'],
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


function movie_ticket() {

}


function zoo_ticket() {

}




window.detect_path_name = detect_path_name
window.movie_ticket = movie_ticket
window.zoo_ticket = zoo_ticket
window.logout = logout
window.login = login