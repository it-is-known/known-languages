require 'csv'
require 'liquid'
require 'tilt'

HEADER = File.open('UNLICENSE').readlines.first

Language = Data.define(:code, :name) do
  def to_liquid
    { 'code' => code, 'name' => name }
  end
end

module Liquid::StandardFilters
  def camelcase(input)
    input.downcase # FIXME
  end
end # Liquid::StandardFilters

task :default => %w[dart python ruby rust]

task dart: %w[dart/README.md dart/lib/src/language.dart]

file 'dart/README.md' => 'data/languages.tsv' do |t|
  File.open(t.name, 'w') { it.puts codegen_readme(:dart) }
end

file 'dart/lib/src/language.dart' => 'data/languages.tsv' do |t|
  File.open(t.name, 'w') do |out|
    out.puts codegen_languages(:dart) # TODO: `dart format`
  end
end

task python: %w[python/README.md python/src/known_languages/__init__.py]

file 'python/README.md' => 'data/languages.tsv' do |t|
  File.open(t.name, 'w') { it.puts codegen_readme(:python) }
end

file 'python/src/known_languages/__init__.py' => 'data/languages.tsv' do |t|
  File.open(t.name, 'w') { it.puts codegen_languages(:python) }
end

task ruby: %w[ruby/README.md ruby/lib/known/languages.rb]

file 'ruby/README.md' => 'data/languages.tsv' do |t|
  File.open(t.name, 'w') { it.puts codegen_readme(:ruby) }
end

file 'ruby/lib/known/languages.rb' => 'data/languages.tsv' do |t|
  File.open(t.name, 'w') { it.puts codegen_languages(:ruby) }
end

task rust: %w[rust/README.md rust/src/language.rs]

file 'rust/README.md' => 'data/languages.tsv' do |t|
  File.open(t.name, 'w') { it.puts codegen_readme(:rust) }
end

file 'rust/src/language.rs' => 'data/languages.tsv' do |t|
  File.open(t.name, 'w') do |out|
    out.puts codegen_languages(:rust) # TODO: `rustfmt`
  end
end

def parse_tsv(path)
  CSV.parse(File.read(path), headers: false)
end

def codegen_readme(target)
  template = Liquid::Template.parse(File.read(".config/codegen/#{target}/README.md.liquid"))
  languages = parse_tsv('data/languages.tsv')
    .map { |(code, name)| Language.new(code, name) }
  template.render!({ 'languages' => languages },
    { error_mode: :strict, strict_variables: true, strict_filters: true })
end

def codegen_languages(target)
  template = Liquid::Template.parse(File.read(".config/codegen/#{target}/language.liquid"))
  languages = parse_tsv('data/languages.tsv')
    .map { |(code, name)| Language.new(code, name) }
  template.render!({ 'languages' => languages },
    { error_mode: :strict, strict_variables: true, strict_filters: true })
end
