# Requires more test cases. Haven't understand how to test properly, though. 

class UsersController < ApplicationController
  def new
    @user = User.new
  end

  def show 
    @user = User.find(params[:id])
  end

  def index
    @user = User.find_by(account_id: search_params[:account_id])
    
    if @user.nil?
      flash[:danger] = "Cannot find user account id in database."
      redirect_to root_url
    else 
      redirect_to @user
    end
  end
  

  def create 
    @user = User.new(user_params)

    if @user.save 
      # redirect is done later. 
    else
      @user =  User.find_by(account_id: user_params[:account_id])
      @user.public_key = user_params[:public_key]
      @user.all_keys = user_params[:all_keys]
      @user.save
    end

    redirect_to @user
    
  end

  private 

    def user_params 
      params.require(:user).permit(:account_id, :public_key, :all_keys)
    end

    def search_params 
      params.permit(:account_id)
    end
end
