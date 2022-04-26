require "test_helper"

class UserTest < ActiveSupport::TestCase
  def setup
    @user = User.new(
      account_id: "somebodyelse.testnet",
      public_key: "ed25519:ErQ2SMEEReEQXVcNFc2opZPn22dZfGCESQo5NCBpZixW",
      all_keys: "ed25519:DxNEvHbqYWLVZ1rG9Bio3fXSXQEz3nMhQgzCq4qne4eG"
    )
  end

  test "should be valid" do
    assert @user.valid?
  end

  test "account_id should be present" do
    @user.account_id = "   "
    assert_not @user.valid?
  end

  test "public_key should be present" do 
    @user.public_key = "   "
    assert_not @user.valid?
  end

  test "all_keys should be present" do 
    @user.all_keys = "   "
    assert_not @user.valid?
  end

  test "account_id should be unique" do
    dup_user = @user.dup 
    @user.save 
    assert_not dup_user.valid?
  end
end
