require 'csv'
require 'lvr'

Language = Data.define(:code, :name) do
  def to_liquid = self.to_h.transform_keys(&:to_s)
end

codegen = ->(t, _) { Lvr.codegen(t.name, t.source, languages: load_languages) }
copy = ->(t, _) { cp t.source, t.name }

task :default => %w[dart python ruby rust]

task dart: %w[dart/README.md dart/lib/src/language.dart]
file 'dart/README.md' => %w[.config/codegen/dart/README.md.liquid data/languages.csv], &codegen
file 'dart/lib/src/language.dart' => %w[.config/codegen/dart/language.liquid data/languages.csv], &codegen # TODO: `dart format`

task python: %w[python/README.md python/src/known_languages/__init__.py]
file 'python/README.md' => %w[.config/codegen/python/README.md.liquid data/languages.csv], &codegen
file 'python/src/known_languages/__init__.py' => %w[.config/codegen/python/language.liquid data/languages.csv], &codegen

task ruby: %w[ruby/README.md ruby/lib/known/languages.rb]
file 'ruby/README.md' => %w[.config/codegen/ruby/README.md.liquid data/languages.csv], &codegen
file 'ruby/lib/known/languages.rb' => %w[.config/codegen/ruby/language.liquid data/languages.csv], &codegen
file 'ruby/CHANGES.md' => %w[CHANGES.md], &copy
file 'ruby/VERSION' => %w[VERSION], &copy

task rust: %w[rust/README.md rust/src/language.rs]
file 'rust/README.md' => %w[.config/codegen/rust/README.md.liquid data/languages.csv], &codegen
file 'rust/src/language.rs' => %w[.config/codegen/rust/language.liquid data/languages.csv], &codegen

def load_languages() = parse_csv('data/languages.csv')
  .map { |(code, name)| Language.new(code, name) }

def parse_csv(path) = CSV.parse(File.read(path), headers: false)
