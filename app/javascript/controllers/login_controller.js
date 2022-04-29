import { Controller } from "@hotwired/stimulus"

export default class extends Controller {
  static targets = [ "slide", "name" ]

  static values = {current: Boolean}

  initialize() {
    this.currentValue = window.walletConnection.isSignedIn()
    this.showCurrentLogin()
    this.showCurrentName()
  }

  switch() {
    if (this.currentValue) {
      window.logout()
    } else {
      window.login()
    }

    this.currentValue = !this.currentValue
    this.showCurrentLogin()
    this.showCurrentName()
  }

  showCurrentLogin() {
    this.slideTargets.forEach((element, _index) => {
      if (this.currentValue) {
        element.innerText = "Logout"
      } else {
        element.innerText = "Login"
      }
    })
  }

  showCurrentName() {
    const account_id = window.walletConnection.getAccountId()

    this.nameTargets.forEach((element, _index) => {
      element.hidden = !this.currentValue
      element.innerText = account_id
      element.href = "/users/" + account_id.replaceAll('.', '-');
    })
  }
}