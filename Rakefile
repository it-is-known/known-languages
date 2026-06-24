require 'csv'
require 'lvr'

Language = Data.define(:code, :name) do
  def to_liquid = self.to_h.transform_keys(&:to_s)
end

task :default => %w[dart python ruby rust]

task dart: %w[dart/README.md dart/lib/src/language.dart]

file 'dart/README.md' => 'data/languages.csv' do |t|
  File.open(t.name, 'w') { it.puts codegen_readme(:dart) }
end

file 'dart/lib/src/language.dart' => 'data/languages.csv' do |t|
  File.open(t.name, 'w') { it.puts codegen_languages(:dart) } # TODO: `dart format`
end

task python: %w[python/README.md python/src/known_languages/__init__.py]

file 'python/README.md' => 'data/languages.csv' do |t|
  File.open(t.name, 'w') { it.puts codegen_readme(:python) }
end

file 'python/src/known_languages/__init__.py' => 'data/languages.csv' do |t|
  File.open(t.name, 'w') { it.puts codegen_languages(:python) }
end

task ruby: %w[ruby/README.md ruby/lib/known/languages.rb]

file 'ruby/README.md' => 'data/languages.csv' do |t|
  File.open(t.name, 'w') { it.puts codegen_readme(:ruby) }
end

file 'ruby/lib/known/languages.rb' => 'data/languages.csv' do |t|
  File.open(t.name, 'w') { it.puts codegen_languages(:ruby) }
end

task rust: %w[rust/README.md rust/src/language.rs]

file 'rust/README.md' => 'data/languages.csv' do |t|
  File.open(t.name, 'w') { it.puts codegen_readme(:rust) }
end

file 'rust/src/language.rs' => 'data/languages.csv' do |t|
  File.open(t.name, 'w') { it.puts codegen_languages(:rust) } # TODO: `rustfmt`
end

def codegen_readme(target) = Lvr::Template
  .load(".config/codegen/#{target}/README.md.liquid")
  .render(languages: load_languages())

def codegen_languages(target) = Lvr::Template
  .load(".config/codegen/#{target}/language.liquid")
  .render(languages: load_languages())

def load_languages() = parse_csv('data/languages.csv')
  .map { |(code, name)| Language.new(code, name) }

def parse_csv(path) = CSV.parse(File.read(path), headers: false)
