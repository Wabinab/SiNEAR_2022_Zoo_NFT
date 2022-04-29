module CardsHelper
  def get_token_by_id(token_id)
    JSON.parse(@query.function(
      @contract2,
      'nft_token',
      {
        "token_id": token_id,
        "limit": 10
      }
    )["result"]["result"].pack('c*'))
  end

  def cards_to_disable(share_length)
    if share_length <= 0 
      "hidden"
    else
      ""
    end
  end
end
