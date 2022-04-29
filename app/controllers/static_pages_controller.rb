class StaticPagesController < ApplicationController
  def home
  end

  def movie_ticket
    @token_id = "movie_ticket_" + Time.now.to_f.to_s.gsub('.', '_') + '_' + ('a'..'z').to_a.shuffle[0, 5].join
  end

  def future_ticket
    @token_id = "entrance_" + Time.now.to_f.to_s.gsub('.', '_') + '_' + ('a'..'z').to_a.shuffle[0, 5].join
  end
end
