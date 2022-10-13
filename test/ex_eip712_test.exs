defmodule EIP712Test do
  use ExUnit.Case
  doctest EIP712

  test "signs typed data" do
    secret = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"

    message = """
      {"types":{"EIP712Domain":[{"name":"name","type":"string"},{"name":"version","type":"string"},{"name":"chainId","type":"uint256"},{"name":"verifyingContract","type":"address"}],"Nft":[{"name":"tokenHash","type":"string"},{"name":"price","type":"uint256"},{"name":"receivers","type":"address[]"},{"name":"percents","type":"uint256[]"}]},"primaryType":"Nft","domain":{"name":"Gallery","version":"4","chainId":"0x7A69","verifyingContract":"0x5FbDB2315678afecb367f032d93F642f64180aa3"},"message":{"tokenHash":"1e59237e-f0f6-48b5-b384-17270eab0abb","price":"0x6F05B59D3B20000","receivers":["0x70997970C51812dc3A010C7d01b50e0d17dc79C8","0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC"],"percents":["0x7D0","0x3E8"]}}
    """

    assert {:ok, signature} = EIP712.sign(message, secret)

    assert signature ==
             "0x678ae8dc200da9410bec6e218ecce1b3795ce21ccab1438757abad08e282ec826a8b1b04fe55b24c9763b62015deeb079e9695a979d187d0ae73942dab3905e41c"
  end

  test "signs message" do
    secret = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
    message = "Hello, world!"
    assert {:ok, signature} = EIP712.sign_message(message, secret)

    assert signature ==
             "0x9dba579c978220c653b992b02817dc676a08486dc12f9fb0e9898b506a92fae97770a8198ae4915e9e43504d32cefaf5f88b4e6c1ad02d0d0f8604880de2aa251b"
  end
end
