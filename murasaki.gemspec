require './lib/murasaki/version'

Gem::Specification.new do |s|
  s.name                     = 'murasaki'
  s.version                  = Murasaki::VERSION
  s.required_ruby_version    = '>=2.2.6'
  s.date                     = Time.now.strftime('%Y-%m-%d')
  s.summary                  = 'Event Kernel for Ruby'
  s.description              = 'Event Kernel for Ruby, power for midori project'
  s.authors                  = ['HeckPsi Lab']
  s.email                    = 'business@heckpsi.com'
  s.require_paths            = ['lib']
  s.files                    = `git ls-files -z`.split("\x0").reject { |f| f.match(%r{^(spec|.resources)/}) } \
    - %w(README.md CONTRIBUTOR_COVENANT_CODE_OF_CONDUCT.md Gemfile Rakefile em-midori.gemspec .gitignore .rspec .codeclimate.yml .rubocop.yml .travis.yml logo.png Rakefile Gemfile)
  s.homepage                 = 'https://github.com/heckpsi-lab/murasaki'
  s.metadata                 = { 'issue_tracker' => 'https://github.com/heckpsi-lab/murasaki/issues' }
  s.license                  = 'MIT'
  s.add_runtime_dependency     'nio4r', '~> 2.0'
end
