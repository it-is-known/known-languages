# This is free and unencumbered software released into the public domain.

module Known; end
module Known::Languages; end

##
# A language (based on ISO 639-1).
class Known::Languages::Language
  ##
  # The language code (ISO 639-1).
  #
  # @return [Symbol]
  attr_reader :code

  private_class_method :new

  ##
  # Defines a new language.
  #
  # @param code [#to_sym] The language code (ISO 639-1).
  def initialize(code)
    @code = code.to_sym
    self.freeze
  end

  ##
  # Arabic ("ar" in ISO 639-1)
  #
  # @return [Language]
  ARABIC = new(:ar)

  ##
  # Bengali ("bn" in ISO 639-1)
  #
  # @return [Language]
  BENGALI = new(:bn)

  ##
  # English ("en" in ISO 639-1)
  #
  # @return [Language]
  ENGLISH = new(:en)

  ##
  # Esperanto ("eo" in ISO 639-1)
  #
  # @return [Language]
  ESPERANTO = new(:eo)

  ##
  # Spanish ("es" in ISO 639-1)
  #
  # @return [Language]
  SPANISH = new(:es)

  ##
  # French ("fr" in ISO 639-1)
  #
  # @return [Language]
  FRENCH = new(:fr)

  ##
  # Hindi ("hi" in ISO 639-1)
  #
  # @return [Language]
  HINDI = new(:hi)

  ##
  # Indonesian ("id" in ISO 639-1)
  #
  # @return [Language]
  INDONESIAN = new(:id)

  ##
  # Portuguese ("pt" in ISO 639-1)
  #
  # @return [Language]
  PORTUGUESE = new(:pt)

  ##
  # Urdu ("ur" in ISO 639-1)
  #
  # @return [Language]
  URDU = new(:ur)

  ##
  # Chinese ("zh" in ISO 639-1)
  #
  # @return [Language]
  CHINESE = new(:zh)
end # Known::Languages::Language
