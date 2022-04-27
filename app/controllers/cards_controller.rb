class CardsController < ApplicationController
  def new
  end

  def show
    @token_id = token_id
  end

  private

    def token_id
      params.require(:token_id)
    end
end
