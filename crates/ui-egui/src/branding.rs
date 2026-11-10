//! User-facing names for the independent forks. Internal command ids and file formats stay stable.
use std::borrow::Cow;

pub const APP_NAME: &str = "Nodus";

/// Apply branding after translation, without altering translation keys or user document names.
pub fn text(value: &str) -> Cow<'_, str> {
    if !["PdfCraft", "PhotoCraft", "VectorCraft"].iter().any(|name| value.contains(name)) {
        return Cow::Borrowed(value);
    }
    Cow::Owned(value.replace("PdfCraft", "Aevum").replace("PhotoCraft", "Iride").replace("VectorCraft", "Nodus"))
}

/// Localized wording owned by this fork, separate from upstream translation keys.
/// English is the fallback; existing app translations continue through `i18n`.
pub fn fork_label(source: &str) -> &str {
    fork_label_in(crate::i18n::current().code(), source)
}

fn fork_label_in<'a>(locale: &str, source: &'a str) -> &'a str {
    match (locale, source) {
        ("zh-hans", "Independent macOS ARM build") => "独立维护的 macOS ARM 版本",
        ("zh-hant", "Independent macOS ARM build") => "獨立維護的 macOS ARM 版本",
        ("zh-hans", "Independent macOS ARM fork") => "独立维护的 macOS ARM 分支",
        ("zh-hant", "Independent macOS ARM fork") => "獨立維護的 macOS ARM 分支",
        ("zh-hans", "Fork source and upstream project resources.") => "本分支源代码与上游项目资源。",
        ("zh-hant", "Fork source and upstream project resources.") => "本分支原始碼與上游專案資源。",
        ("zh-hans", "Upstream community") => "上游社区（Discord）",
        ("zh-hant", "Upstream community") => "上游社群（Discord）",
        ("zh-hans", "Upstream website") => "上游网站",
        ("zh-hant", "Upstream website") => "上游網站",
        ("zh-hans", "Upstream ArtCraft website") => "上游 ArtCraft 网站",
        ("zh-hant", "Upstream ArtCraft website") => "上游 ArtCraft 網站",
        ("zh-hans", "Independent ribbon suite") => "色带系列标志",
        ("zh-hant", "Independent ribbon suite") => "色帶系列標誌",
        ("zh-hans", "Upstream VectorCraft website") => "上游 VectorCraft 网站",
        ("zh-hant", "Upstream VectorCraft website") => "上游 VectorCraft 網站",
        ("zh-hans", "Nodus source code") => "Nodus 源代码",
        ("zh-hant", "Nodus source code") => "Nodus 原始碼",
        (
            "zh-hans",
            "Based on upstream VectorCraft. MIT OR Apache-2.0. Fonts: Source Sans 3, Inter, JetBrains Mono (OFL). Icons: Lucide (ISC) + VectorCraft.",
        ) => {
            "基于上游 VectorCraft。许可：MIT 或 Apache-2.0。字体：Source Sans 3、Inter、JetBrains Mono（OFL）。界面图标：Lucide（ISC）及 VectorCraft。"
        }
        (
            "zh-hant",
            "Based on upstream VectorCraft. MIT OR Apache-2.0. Fonts: Source Sans 3, Inter, JetBrains Mono (OFL). Icons: Lucide (ISC) + VectorCraft.",
        ) => {
            "基於上游 VectorCraft。授權：MIT 或 Apache-2.0。字型：Source Sans 3、Inter、JetBrains Mono（OFL）。介面圖示：Lucide（ISC）及 VectorCraft。"
        }
        _ => source,
    }
}

/// Link labels identify the fork and the upstream destination without changing command ids.
pub fn link_text(command: &str) -> Option<&'static str> {
    let source = match command {
        "help.discord" => "Upstream community",
        "help.github" => "Nodus source code",
        "help.appPage" => "Upstream VectorCraft website",
        "help.website" => "Upstream ArtCraft website",
        _ => return None,
    };
    Some(fork_label(source))
}

/// Rebrand owned commands without altering document names, presets or file-format labels.
pub fn menu_text<'a>(command: &str, value: &'a str) -> Cow<'a, str> {
    if let Some(label) = link_text(command) {
        Cow::Borrowed(label)
    } else if command == "help.about" {
        text(value)
    } else {
        Cow::Borrowed(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fork_labels_localize_without_renaming_upstream_or_user_content() {
        for (code, suffix) in [("zh-hans", "源代码"), ("zh-hant", "原始碼")] {
            let source = format!("{APP_NAME} source code");
            assert!(fork_label_in(code, &source).ends_with(suffix));
            assert!(fork_label_in(code, "Upstream ArtCraft website").contains("ArtCraft"));
            assert!(!fork_label_in(code, "Independent macOS ARM build").starts_with("Independent"));
        }
        assert_eq!(fork_label_in("fr", "Unknown text"), "Unknown text");
        assert_eq!(menu_text("file.openRecent", "PhotoCraft draft.pdf"), "PhotoCraft draft.pdf");
        assert_eq!(menu_text("presets.choose", "PhotoCraft Defaults"), "PhotoCraft Defaults");
    }

    #[test]
    fn product_names_change_after_translation_but_normal_text_stays_borrowed() {
        assert_eq!(text("关于 PhotoCraft"), "关于 Iride");
        assert_eq!(text("About PdfCraft / VectorCraft"), "About Aevum / Nodus");
        assert!(matches!(text("Open…"), Cow::Borrowed(_)));
    }
}
