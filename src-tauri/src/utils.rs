use serde_json::json;

pub fn get_google_language_code(lang: &str) -> &'static str {
    match lang {
        "Automatic" => "auto",
        "English" => "en",
        "Spanish" => "es",
        "French" => "fr",
        "German" => "de",
        "Italian" => "it",
        "Dutch" => "nl",
        "Portuguese" => "pt",
        "Russian" => "ru",
        "Japanese" => "ja",
        "Chinese" => "zh",
        "Korean" => "ko",
        "Arabic" => "ar",
        "Turkish" => "tr",
        "Polish" => "pl",
        "Swedish" => "sv",
        "Danish" => "da",
        "Finnish" => "fi",
        "Norwegian" => "no",
        "Greek" => "el",
        "Hebrew" => "he",
        "Indonesian" => "id",
        "Ukrainian" => "uk",
        "Thai" => "th",
        "Czech" => "cs",
        "Hindi" => "hi",
        "Bengali" => "bn",
        "Croatian" => "hr",
        "Hungarian" => "hu",
        "Malay" => "ms",
        "Romanian" => "ro",
        "Slovak" => "sk",
        "Vietnamese" => "vi",
        "Catalan" => "ca",
        "Filipino" => "fil",
        "Serbian" => "sr",
        "Lithuanian" => "lt",
        "Slovenian" => "sl",
        "Latvian" => "lv",
        "Estonian" => "et",
        "Maltese" => "mt",
        "Icelandic" => "is",
        "Albanian" => "sq",
        "Macedonian" => "mk",
        "Swahili" => "sw",
        "Welsh" => "cy",
        "Basque" => "eu",
        "Galician" => "gl",
        "Scots Gaelic" => "gd",
        "Breton" => "br",
        "Corsican" => "co",
        "Azerbaijani" => "az",
        "Armenian" => "hy",
        "Georgian" => "ka",
        "Kazakh" => "kk",
        _ => "en"
    }
}

pub fn get_bcp47_language_code(lang: &str) -> &'static str {
    match lang {
        "Automatic" => "",
        "English" => "en-US",
        "Spanish" => "es-ES",
        "French" => "fr-FR",
        "German" => "de-DE",
        "Italian" => "it-IT",
        "Dutch" => "nl-NL",
        "Portuguese" => "pt-PT",
        "Russian" => "ru-RU",
        "Japanese" => "ja-JP",
        "Chinese" => "zh-CN",
        "Chinese Traditional" => "zh-TW",
        "Korean" => "ko-KR",
        "Arabic" => "ar-SA",
        "Turkish" => "tr-TR",
        "Polish" => "pl-PL",
        "Swedish" => "sv-SE",
        "Danish" => "da-DK",
        "Finnish" => "fi-FI",
        "Norwegian" => "nb-NO",
        "Greek" => "el-GR",
        "Hebrew" => "he-IL",
        "Indonesian" => "id-ID",
        "Ukrainian" => "uk-UA",
        "Thai" => "th-TH",
        "Czech" => "cs-CZ",
        "Hindi" => "hi-IN",
        "Bengali" => "bn-IN",
        "Croatian" => "hr-HR",
        "Hungarian" => "hu-HU",
        "Malay" => "ms-MY",
        "Romanian" => "ro-RO",
        "Slovak" => "sk-SK",
        "Vietnamese" => "vi-VN",
        "Catalan" => "ca-ES",
        "Filipino" => "fil-PH",
        _ => ""
    }
}

pub fn get_platform_cropping(platform: &str) -> (f64, f64, f64, f64) {
    match platform {
        "Netflix" => (0.1, 0.04, 0.8, 0.84),
        "Hulu" => (0.29, 0.6, 0.42, 0.37),
        "Amazon Prime Video" => (0.25, 0.04, 0.50, 0.92),
        "Disney+" => (0.15, 0.03, 0.7, 0.94),
        "Max" => (0.15, 0.03, 0.7, 0.91),
        "YouTube" => (0.24, 0.7, 0.52, 0.3),
        "VLC" => (0.20, 0.7, 0.60, 0.22),
        "AppleTV" => (0.23, 0.03, 0.54, 0.90),
        _ => (0.15, 0.03, 0.7, 0.94)
    }
}

pub fn get_request_google_ocr(b64_encoded_img: String, lang: &String) -> serde_json::Value {
    let non_latin_scripts_languages = vec!["Chinese", "Chinese Traditional", "Japanese",
                                           "Korean", "Russian", "Arabic", "Greek", "Hebrew",
                                           "Ukrainian", "Hindi", "Bengali"];
    if non_latin_scripts_languages.contains(&lang.as_str()) {
        json!({
            "requests": [
                {
                  "image": {
                    "content": b64_encoded_img
                  },
                  "features": [
                    {
                      "type": "TEXT_DETECTION"
                    }
                  ],
                  "imageContext": {
                    "languageHints": [get_bcp47_language_code(lang)]
                  }
                }
            ]
        })
    } else {
        json!({
            "requests": [
                {
                  "image": {
                    "content": b64_encoded_img
                  },
                  "features": [
                    {
                      "type": "TEXT_DETECTION"
                    }
                  ]
                }
            ]
        })
    }
}

pub fn get_request_google_translate(text: String, target_lang: &str, origin_lang: &str) -> serde_json::Value {
    if origin_lang == "auto" || origin_lang == "Automatic" || origin_lang == "" {
        return json!({
            "q": text,
            "target": get_google_language_code(target_lang)
        })
    } else {
        json!({
        "q": text,
        "source": get_google_language_code(origin_lang),
        "target": get_google_language_code(target_lang)
    })
    }
}