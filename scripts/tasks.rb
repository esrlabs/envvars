namespace :build do
  desc 'Rust'
  task :rust do
    Shell.sh 'rustup install stable'
    Shell.sh 'rustup default stable'
  end

  desc 'Build'
  task :lib do
    Shell.sh 'cargo build --release'
    Reporter.add(Jobs::Building, Owner::Lib, 'built', '')
  end

  desc 'build'
  task envvars: ['build:rust', 'build:lib'] do
    Reporter.print
  end
end

namespace :test do
  desc 'Build'
  task :lib do
    Shell.sh 'cargo test -- --nocapture'
    Reporter.add(Jobs::Test, Owner::Lib, 'tested', '')
  end

  desc 'Cargo packing check'
  task :packing do
    Rake::Task['build:rust'].invoke
    Shell.sh 'cargo publish --dry-run'
    Reporter.add(Jobs::Test, Owner::Lib, 'cargo publich has been checked', '')
  end

  desc 'test'
  task envvars: ['build:envvars', 'test:lib', 'test:packing'] do
    Reporter.print
  end
end

namespace :clippy do
  desc 'Clippy update to nightly'
  task :nightly do
    Shell.sh 'rustup install nightly'
    Shell.sh 'rustup default nightly'
    Shell.sh 'rustup component add --toolchain=nightly clippy-preview'
  end

  desc 'Clippy extractor'
  task :lib do
    Shell.sh Paths::CLIPPY_NIGHTLY
    Reporter.add(Jobs::Clippy, Owner::Lib, 'checked', '')
  end

  desc 'Clippy all'
  task envvars: ['clippy:nightly', 'clippy:lib'] do
    Reporter.print
  end
end

namespace :clean do
  desc 'Clean lib'
  task :lib do
    Shell.rm_rf('./target')
    Reporter.add(Jobs::Clearing, Owner::Lib, 'removed: ./target', '')
  end

  desc 'Clean all'
  task envvars: ['clean:lib'] do
    Reporter.print
  end
end

task :default do
  Rake::Task['clippy:envvars'].invoke
  Rake::Task['test:envvars'].invoke
  Rake::Task['build:envvars'].invoke
end
