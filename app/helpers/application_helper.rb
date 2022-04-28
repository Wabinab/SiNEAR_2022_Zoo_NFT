module ApplicationHelper
  def define_constants
    @node_url = 'https://rpc.testnet.near.org/'
    @conf = NearApi::Config.new(node_url: @node_url)
    @query = NearApi::Query.new(config = @conf)

    @contract = 'greeter.wabinab.testnet'
    @contract2 = 'zoo_nft.wabinab.testnet'
  end
end
