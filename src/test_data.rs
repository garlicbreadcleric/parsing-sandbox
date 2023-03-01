pub static BENCHMARK_INPUT: &str = LONG_MULTILINE_INPUT;

pub static SHORT_ASCII_INPUT: &str = "foo [bar] baz";
pub static SHORT_UNICODE_INPUT: &str = "йцу [фыв] ячс";

pub static LONG_ASCII_INPUT: &str =
  "qwerty qwerty qwetry qwerty qwerty qwetry [asd asd asd] zxcvb zxcvb zxcvb zxcvb zxcvb zxcvb";
pub static LONG_UNICODE_INPUT: &str =
  "йцуке йцуке йцуке йцуке йцуке йцуке [фыв фыв фыв] ячсми ячсми ячсми ячсми ячсми ячсми";

pub static SHORT_MULTILINE_INPUT: &str = "
# Starfinder

## Кампании

- [Мышеловка](20220820_mousetrap-campaign-starfinder.md)

## Персонажи

- [Кнопка](20220813_knopka-character-starfinder.md)
  - [Ведро (стелс-дрон Кнопки)](20220817_knopka-stealth-drone.md)

## Правила

- [Состояния](20220918_conditions-starfinder.md)
- [Бой](20220918_combat-starfinder.md)
- [Космический бой](20220827_space-fight-starfinder.md)
  - Действия экипажа
    - [Бортинженер](20220827_engineer-role-starfinder.md)
    - [Офицер по науке](20220827_science-officer-role-starfinder.md)
";

pub static LONG_MULTILINE_INPUT: &str = "
# Grune, Dick, and Ceriel J. H. Jacobs. _Parsing Techniques: A Practical Guide_. 1990.

**Определение**: _Парсинг_ --- процесс структурирования линейного предствления в соответствии с некоторой грамматикой [@grune_parsingtechniques_en_1990, 1].


## Грамматики как средства генерации предложений

Два типа символов [@grune_parsingtechniques_en_1990, 13]:

- **Определение**: _Терминальный символ_ --- символ, буквально присутствующий в предложениях языка (например, `42`)
- **Определение**: _Нетерминальный символ_ --- символ, заменяющий собой некоторые части предложений (например, `сложение`)

**Определение**: _Начальный символ_ --- нетерминальный символ, с которого начинается генерация предложений (например, `выражение`)

**Определение**: _Грамматика с фразовой структурой_ --- набор $(V_N, V_T, R, S)$ такой, что:

1. $V_N$ и $V_T$ --- конечные множества символов;
2. $V_N \\cap V_T = \\empty$;
3. $R$ --- множество пар $(P, Q)$ таких, что
   i. $P \\in (V_N \\cup V_T)^+$;
   ii. $Q \\in (V_N \\cup V_T)^*$;
4. $S \\in V_N$ [@grune_parsingtechniques_en_1990, 14].

**Определение**: _Иерархия Хомского_ --- классификация формальных грамматик по количеству ограничений [@grune_parsingtechniques_en_1990, 19].

**Определение**: _Тип 0_ --- класс неограниченных грамматик с фазовой структурой (см. выше).

**Определение**: _Тип 1 (монотонные)_ --- класс грамматик, у которых не правил, левая часть которых состояла бы из большего числа символов, чем правая ($\\forall (P, Q) \\in R, 1 \\leq P \\leq Q$).

**Определение**: _Тип 1 (контекстно-зависимые, context-sensitive, CS)_ --- класс грамматик, все правила которых являются контекстно-зависимыми, то есть в них только один нетерминальный символ из левой части заменяется непустым наборов символов в правой части, а другие остаются в том же составе и порядке ($(P, Q) = (\\alpha A \\beta, \\alpha \\gamma \\beta), A \\in V = V_N \\cup V_T, \\alpha, \\beta \\in V^*, \\gamma \\in V^+$).

**Определение**: _Тип 2 (контекстно-независимые, context-free, CF)_ --- класс контекстно-зависимых грамматик с пустым контекстом ($\\forall (P, Q) \\in R, (P, Q) = (A, \\gamma), A \\in V, \\gamma \\in V^+$) [@grune_parsingtechniques_en_1990, 23].

**Определение**: Нетерминальный символ, язык которого содержит $\\epsilon$, называется _обнуляемым (nullable)_.xxxx
";
