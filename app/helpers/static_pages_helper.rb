module StaticPagesHelper
  def get_movie_tickets_left
    @query.function(
      @contract,
      'get_tickets_left',
      {
        "template_id": "movie_tickets"
      }
    )["result"]["result"].pack('c*')
  end
end
