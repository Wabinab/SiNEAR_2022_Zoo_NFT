class AccountIdUniqueify < ActiveRecord::Migration[7.0]
  def change
    add_index :users, :account_id, unique: true
  end
end
