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


function set_greeting() {
  var message = document.getElementById("form_message").value;
  window.contract.set_greeting({"message": message})
  .then(
    value => {
      alert("Successful set_greeting for yourself.");
      window.location.reload();
    },
    err => alert(err),
  );
}


function set_greeting_for_others(target) {
  var message = document.getElementById("someone_message").value;
  window.contract.set_greeting_for_others({
    "target": target,
    "message": message
  }).then(
    value => {
      alert("Successful called set_greeting_for_others.");
      window.location.reload();
    },
    err => alert(err),
  );
}



window.set_greeting = set_greeting
window.set_greeting_for_others = set_greeting_for_others
window.logout = logout
window.login = login