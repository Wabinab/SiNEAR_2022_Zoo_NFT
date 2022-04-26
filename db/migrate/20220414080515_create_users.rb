class CreateUsers < ActiveRecord::Migration[7.0]
  def change
    create_table :users do |t|
      t.string :account_id
      t.string :public_key
      t.string :all_keys

      t.timestamps
    end
  end
end
