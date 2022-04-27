Rails.application.routes.draw do
  get 'cards/new'
  get 'cards/show'

  get '/cards/:token_id', to: 'cards#show'
  get 'cards', to: 'cards#new'

  get 'users', to: 'users#index'
  post 'users', to: 'users#create'
  # get '@:account_id', to: 'users#index'

  root 'users#new'

  resources :users
end
