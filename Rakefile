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

task dart: 'dart/lib/src/language.dart'
file 'dart/lib/src/language.dart' => 'data/languages.tsv' do |t|
  File.open(t.name, 'w') do |out|
    out.puts codegen_languages(:dart) # TODO: `dart format`
  end
end

task python: 'python/src/known_languages/__init__.py'
file 'python/src/known_languages/__init__.py' => 'data/languages.tsv' do |t|
  File.open(t.name, 'w') do |out|
    out.puts codegen_languages(:python)
  end
end

task ruby: 'ruby/lib/known/languages.rb'
file 'ruby/lib/known/languages.rb' => 'data/languages.tsv' do |t|
  File.open(t.name, 'w') do |out|
    out.puts codegen_languages(:ruby)
  end
end

task rust: 'rust/src/language.rs'
file 'rust/src/language.rs' => 'data/languages.tsv' do |t|
  File.open(t.name, 'w') do |out|
    out.puts codegen_languages(:rust) # TODO: `rustfmt`
  end
end

def parse_tsv(path)
  CSV.parse(File.read(path), headers: false)
end

def codegen_languages(target)
  template = Liquid::Template.parse(File.read(".config/codegen/#{target}/language.liquid"))
  languages = parse_tsv('data/languages.tsv')
    .map { |(code, name)| Language.new(code, name) }
  template.render!({ 'languages' => languages },
    { error_mode: :strict, strict_variables: true, strict_filters: true })
end
