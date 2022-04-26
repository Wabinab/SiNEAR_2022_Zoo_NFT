class User < ApplicationRecord
  validates :account_id, presence: true, uniqueness: true
  validates :public_key, :all_keys, presence: true
end
