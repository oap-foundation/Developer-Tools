class OapCli < Formula
  desc "Command Line Interface for the Open Agent Protocol"
  homepage "https://oap.foundation"
  url "https://github.com/oap-foundation/oap-cli/releases/download/v0.1.0/oap-macos-amd64"
  sha256 "REPLACE_WITH_SHA256"
  version "0.1.0"

  def install
    bin.install "oap"
    
    # Install completions
    generate_completions_from_executable(bin/"oap", "completions")
  end

  test do
    assert_match "Open Agent Protocol CLI", shell_output("#{bin}/oap --help")
  end
end
