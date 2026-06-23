// This is free and unencumbered software released into the public domain.

/// A language (based on ISO 639-1).
enum Language {

  /// Arabic ("ar" in ISO 639-1)
  arabic("ar"),

  /// Bengali ("bn" in ISO 639-1)
  bengali("bn"),

  /// English ("en" in ISO 639-1)
  english("en"),

  /// Esperanto ("eo" in ISO 639-1)
  esperanto("eo"),

  /// Spanish ("es" in ISO 639-1)
  spanish("es"),

  /// French ("fr" in ISO 639-1)
  french("fr"),

  /// Hindi ("hi" in ISO 639-1)
  hindi("hi"),

  /// Indonesian ("id" in ISO 639-1)
  indonesian("id"),

  /// Portuguese ("pt" in ISO 639-1)
  portuguese("pt"),

  /// Urdu ("ur" in ISO 639-1)
  urdu("ur"),

  /// Chinese ("zh" in ISO 639-1)
  chinese("zh"),;

  /// The language code (ISO 639-1).
  final String code;

  /// Defines a new language.
  const Language(this.code);

  /// Creates a [Language] from the given ISO 639-1 code.
  factory Language.fromCode(String code) {
    return Language.values.firstWhere(
      (lang) => lang.code == code,
      orElse: () => throw ArgumentError('Unknown ISO-639 code: $code'),
    );
  }
}
