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
  def constcase(input)
    input.split(' ').map(&:upcase).join('_')
  end

  def camelcase(input)
    output = pascalcase(input)
    output[0].downcase + output[1..]
  end

  def pascalcase(input)
    input.split(' ').map(&:capitalize).join('')
  end
end # Liquid::StandardFilters

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

def codegen_readme(target)
  languages = load_languages()
  template = load_template(".config/codegen/#{target}/README.md.liquid")
  template.render!({ 'languages' => languages },
    { error_mode: :strict, strict_variables: true, strict_filters: true })
end

def codegen_languages(target)
  languages = load_languages()
  template = load_template(".config/codegen/#{target}/language.liquid")
  template.render!({ 'languages' => languages },
    { error_mode: :strict, strict_variables: true, strict_filters: true })
end

def load_template(path) = Liquid::Template.parse(File.read(path))

def load_languages() = parse_csv('data/languages.csv')
  .map { |(code, name)| Language.new(code, name) }

def parse_csv(path) = CSV.parse(File.read(path), headers: false)
