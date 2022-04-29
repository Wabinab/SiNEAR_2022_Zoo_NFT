import { Controller } from "@hotwired/stimulus"

export default class extends Controller {
  static targets = [ "owner", "sharer" ]

  static values = {current: String}

  initialize() {
    window.currentValue = this.currentValue;
    this.showCurrentName()
  }

  showCurrentName() {
    const account_id = window.walletConnection.getAccountId()

    this.ownerTargets.forEach((element, _index) => {
      element.hidden = this.currentValue != account_id
    })

    this.sharerTargets.forEach((element, _index) => {
      element.hidden = this.currentValue == account_id
    })
  }
}