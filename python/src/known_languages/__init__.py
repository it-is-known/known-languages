# This is free and unencumbered software released into the public domain.

from enum import StrEnum

# A language (based on ISO 639-1).
class Language(StrEnum):
    """A language"""

    ARABIC = "ar"
    """Arabic ("ar" in ISO 639-1)"""

    BENGALI = "bn"
    """Bengali ("bn" in ISO 639-1)"""

    ENGLISH = "en"
    """English ("en" in ISO 639-1)"""

    ESPERANTO = "eo"
    """Esperanto ("eo" in ISO 639-1)"""

    SPANISH = "es"
    """Spanish ("es" in ISO 639-1)"""

    FRENCH = "fr"
    """French ("fr" in ISO 639-1)"""

    HINDI = "hi"
    """Hindi ("hi" in ISO 639-1)"""

    INDONESIAN = "id"
    """Indonesian ("id" in ISO 639-1)"""

    PORTUGUESE = "pt"
    """Portuguese ("pt" in ISO 639-1)"""

    URDU = "ur"
    """Urdu ("ur" in ISO 639-1)"""

    CHINESE = "zh"
    """Chinese ("zh" in ISO 639-1)"""
