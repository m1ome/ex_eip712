defmodule ExEip712.MixProject do
  use Mix.Project

  def project do
    [
      app: :ex_eip712,
      version: "0.1.0",
      elixir: "~> 1.13",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      description: description(),
      package: package(),
      name: "EIP712",
      source_url: "https://github.com/m1ome/ex_eip712",
      docs: [
        main: "EIP712",
        extras: ["README.md"]
      ]
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.23.0"},
      {:ex_doc, "~> 0.27", only: :dev, runtime: false}
    ]
  end

  defp description() do
    "Package for simplifying EIP712 signature creation from Elixir"
  end

  defp package() do
    [
      # This option is only needed when you don't want to use the OTP application name
      name: "ex_eip712",
      # These are the default files included in the package
      files: ~w(lib native/eip712/Cargo.* native/eip712/crates/** native/eip712/src/*.rs .formatter.exs mix.exs README.md),
      licenses: ["MIT"],
      links: %{"GitHub" => "https://github.com/m1ome/ex_eip712"}
    ]
  end
end
