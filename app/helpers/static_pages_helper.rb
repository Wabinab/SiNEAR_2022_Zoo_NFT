module StaticPagesHelper
  def get_tickets_left(template_id)
    JSON.parse(@query.function(
      @contract,
      'get_tickets_left',
      {
        "template_id": template_id
      }
    )["result"]["result"].pack('c*'))
  end

  def get_total_tickets(template_id)
    JSON.parse(@query.function(
      @contract,
      'get_total_tickets',
      {
        "template_id": template_id
      }
    )["result"]["result"].pack('c*'))
  end

  def to_disable(template_id)
    if get_tickets_left(template_id) >= get_total_tickets(template_id)
      "disabled"
    else
      ""
    end
  end
end
