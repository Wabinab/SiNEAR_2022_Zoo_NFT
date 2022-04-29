module StaticPagesHelper
  def get_tickets_left(template_id)
    @query.function(
      @contract,
      'get_tickets_left',
      {
        "template_id": template_id
      }
    )["result"]["result"].pack('c*')
  end

  def get_total_tickets(template_id)
    @query.function(
      @contract,
      'get_total_tickets',
      {
        "template_id": template_id
      }
    )["result"]["result"].pack('c*')
  end
end
