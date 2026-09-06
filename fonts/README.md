# Vendored fonts (D0235 A8 / robotoVendoring task)

Roboto v3.016 static TTFs from googlefonts/roboto-classic release v3.016 (android/static).
License: SIL Open Font License 1.1 (OFL.txt, verified via the repo's declared SPDX OFL-1.1).
Cut set: Regular, Italic, Medium, MediumItalic, Bold, BoldItalic, Light, LightItalic, Thin,
Condensed-Regular, Condensed-Light.
The PDF reference renderer uses ONLY this directory (--font-path fonts --ignore-system-fonts):
Arial/Helvetica are not redistributable and never apply to the vendored PDF path (D0235 A8).

Noto fallbacks (added for D0236 expressiveness): Noto Sans Math (arrows, operators),
Noto Sans Symbols (enclosed alphanumerics), Noto Sans Symbols 2 (warning/ballot signs),
Noto Emoji (monochrome emoji - deliberate: print-consistent, professional documents).
All Noto: SIL OFL 1.1 (OFL-Noto.txt), from google/fonts@main ofl/ paths.
