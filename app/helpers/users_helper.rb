require 'near_api'

module UsersHelper

  def gravatar_for(user)
    gravatar_id = Digest::MD5::hexdigest(user.account_id)
    gravatar_url = "https://secure.gravatar.com/avatar/#{gravatar_id}?d=identicon&r=PG"
    image_tag(gravatar_url, class: "gravatar")
  end

  def get_greeting(user)
    account_id = user.account_id
    @query.function(
      @contract,
      'get_greeting',
      {
        "account_id": account_id
      }
    )["result"]["result"].pack('c*')
  end

  def get_others_set_greeting(user)
    account_id = user.account_id
    @query.function(
      @contract,
      'get_others_set_greeting',
      {
        "account_id": account_id
      }
    )["result"]["result"].pack('c*')
  end

  def get_tokens(account_id)
    JSON.parse(@query.function(
      @contract2,
      'nft_tokens_for_owner',
      {
        "account_id": account_id,
        "limit": 10
      }
    )["result"]["result"].pack('c*'))
  end
end
