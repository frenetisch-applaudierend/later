use std::{collections::HashMap, fs, path::Path, sync::OnceLock};

use fluent_bundle::{bundle::FluentBundle, FluentResource};
use intl_memoizer::concurrent::IntlLangMemoizer;
use unic_langid::{langid, LanguageIdentifier};

static TRANSLATOR: OnceLock<Translator> = OnceLock::new();

type Bundle = FluentBundle<FluentResource, IntlLangMemoizer>;

pub struct Translator {
    languages: HashMap<LanguageIdentifier, Bundle>,
}

impl Translator {
    pub fn setup() {
        _ = TRANSLATOR.set(Self::new());
    }

    pub fn get() -> &'static Self {
        TRANSLATOR.get().expect("You need to call setup first")
    }

    pub fn get_lang(langid: &LanguageIdentifier) -> &'static FluentBundle<FluentResource, IntlLangMemoizer> {
        let translator = Self::get();
        let mut langid: LanguageIdentifier = langid.clone();

        loop {
            if let Some(bundle) = translator.languages.get(&langid) {
                return bundle;
            }

            if langid.script.is_some() {
                langid.script = None;
                continue;
            }

            if langid.region.is_some() {
                langid.region = None;
                continue;
            }

            break;
        }

        // Fallback to default language
        translator
            .languages
            .get(&langid!("en"))
            .unwrap_or(translator.languages.values().next().unwrap())
    }

    fn new() -> Self {
        let mut languages = HashMap::new();

        let en = Self::load_lang(langid!("en"));
        languages.insert(en.0, en.1);

        let de = Self::load_lang(langid!("de"));
        languages.insert(de.0, de.1);

        Translator { languages }
    }

    fn load_lang(lang: LanguageIdentifier) -> (LanguageIdentifier, Bundle) {
        tracing::info!("Loading language {}", lang);

        let mut bundle = FluentBundle::new_concurrent(vec![lang.clone()]);

        let path = format!("locales/{}/main.ftl", lang);
        let path = Path::new(&path);
        let content = fs::read_to_string(path).expect("Could not read translations file");

        let res = FluentResource::try_new(content).expect("What could fail?");

        bundle.add_resource(res).expect("Why would this fail?");

        (lang, bundle)
    }
}
