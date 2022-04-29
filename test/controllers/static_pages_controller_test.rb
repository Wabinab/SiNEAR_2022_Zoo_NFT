require "test_helper"

class StaticPagesControllerTest < ActionDispatch::IntegrationTest
  test "should get root" do 
    get root_url 
    assert_response :success
  end

  test "should get home" do
    get home_url
    assert_response :success
  end

  test "should get movie ticket" do 
    get movie_ticket_url
    assert_response :success
  end

  test "should get future ticket" do 
    get future_ticket_url 
    assert_response :success
  end
end
