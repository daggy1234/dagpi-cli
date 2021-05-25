class Dagpi < Formula
  desc "A cli for Managing https://dagpi.xyz"
  homepage "https://github.com/daggy1234/dagpi-cli"
  url "https://github.com/daggy1234/dagpi-cli/archive/v0.2.02.tar.gz"
  sha256 "e4aea37ce06efaf2dc410086d286b3750fe03ef837477701d4490fa09b7a9b24"
  version "0.2.0"
  license "MIT"

  depends_on "rust" => :build

  def install
     system "cargo", "install", *std_cargo_args
  end
end
