defmodule EIP712 do
  use Rustler,
    otp_app: :ex_eip712,
    crate: "eip712"

  @moduledoc """
  This module is a wrapper around Rust implementation of [EIP-721](https://eips.ethereum.org/EIPS/eip-712).
  """

  @doc """
  Encodes provided `message` with a `secret` key.

  Returns `{:ok, String.t()}` on success and `{:error, binary()}` on failure.
  """

  @spec sign(String.t(), String.t()) :: {:ok, String.t()} | {:error, binary()}
  def sign(_message, _secret), do: :erlang.nif_error(:nif_not_loaded)
end
