import { Controller } from "@hotwired/stimulus"

export default class extends Controller {
  static targets = [ "owner", "sharer", "noone" ]

  static values = {current: String, length: Number}

  initialize() {
    window.lengthValue = this.lengthValue;
    this.showCurrentName()
    this.showShareButton()
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

  showShareButton() {
    this.ownerTargets.forEach((element, _index) => {
      element.hidden = this.lengthValue == 0
    })

    this.sharerTargets.forEach((element, _index) => {
      element.hidden = this.lengthValue == 0
    })

    this.nooneTargets.forEach((element, _index) => {
      element.hidden = this.lengthValue != 0
    })
  }
}