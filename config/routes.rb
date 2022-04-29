Rails.application.routes.draw do
  get 'home', to: 'static_pages#home'
  get 'movie_ticket', to: 'static_pages#movie_ticket'
  get 'future_ticket', to: 'static_pages#future_ticket'

  post '/login', to: 'sessions#create'
  delete '/logout', to: 'sessions#destroy'


  get '/cards/:token_id', to: 'cards#show'
  get 'cards', to: 'cards#new'

  get 'users', to: 'users#index'
  get '/users/:account_id', to: 'users#show'
  post 'users', to: 'users#create'
  # get '@:account_id', to: 'users#index'

  root 'static_pages#home'

  resources :users
end
