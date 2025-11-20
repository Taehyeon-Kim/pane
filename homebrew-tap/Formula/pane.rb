# Homebrew Formula for Pane
# A blazing-fast TUI skill launcher for developers
class Pane < Formula
  desc "Terminal skill launcher with fuzzy search"
  homepage "https://github.com/Taehyeon-Kim/pane"
  license "MIT"
  version "0.1.0"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/Taehyeon-Kim/pane/releases/download/v0.1.0/pane-v0.1.0-aarch64-apple-darwin.tar.gz"
      sha256 "5a5eb0d69b0fe89f5e94c71d09361ec01fa7c5620b0a8c4857000dae7ec3bbb8"
    else
      url "https://github.com/Taehyeon-Kim/pane/releases/download/v0.1.0/pane-v0.1.0-x86_64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_X86_64_DARWIN_SHA256"
    end
  end

  on_linux do
    url "https://github.com/Taehyeon-Kim/pane/releases/download/v0.1.0/pane-v0.1.0-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "PLACEHOLDER_X86_64_LINUX_SHA256"
  end

  def install
    # Install binaries from bin/ directory in archive
    bin.install "bin/pane"
    bin.install "bin/claude-tips"

    # Install skill files from share/ directory in archive
    # This maps to $PREFIX/share/pane/skills/claude-tips/
    (share/"pane/skills/claude-tips").install "share/pane/skills/claude-tips/pane-skill.yaml"
    (share/"pane/skills/claude-tips/data").install "share/pane/skills/claude-tips/data/claude-tips.yaml"
  end

  test do
    # Verify main binary executes and returns version
    assert_match version.to_s, shell_output("#{bin}/pane --version")

    # Verify claude-tips binary exists and is executable
    assert_predicate bin/"claude-tips", :exist?
    assert_predicate bin/"claude-tips", :executable?

    # Verify skill files were installed correctly
    assert_predicate share/"pane/skills/claude-tips/pane-skill.yaml", :exist?
    assert_predicate share/"pane/skills/claude-tips/data/claude-tips.yaml", :exist?
  end
end
