const fs = require("fs");

const snippet = `# Grune, Dick, and Ceriel J. H. Jacobs. _Parsing Techniques: A Practical Guide_. 1990.

**Определение**: _Парсинг_ --- процесс структурирования линейного предствления в соответствии с некоторой грамматикой [@grune_parsingtechniques_en_1990, 1].


## Грамматики как средства генерации предложений

Два типа символов [@grune_parsingtechniques_en_1990, 13]:

- **Определение**: _Терминальный символ_ --- символ, буквально присутствующий в предложениях языка (например, \`42\`)
- **Определение**: _Нетерминальный символ_ --- символ, заменяющий собой некоторые части предложений (например, \`сложение\`)

**Определение**: _Начальный символ_ --- нетерминальный символ, с которого начинается генерация предложений (например, \`выражение\`)

**Определение**: _Грамматика с фразовой структурой_ --- набор $(V_N, V_T, R, S)$ такой, что:

1. $V_N$ и $V_T$ --- конечные множества символов;
2. $V_N \cap V_T = \empty$;
3. $R$ --- множество пар $(P, Q)$ таких, что
   i. $P \in (V_N \cup V_T)^+$;
   ii. $Q \in (V_N \cup V_T)^*$;
4. $S \in V_N$ [@grune_parsingtechniques_en_1990, 14].

**Определение**: _Иерархия Хомского_ --- классификация формальных грамматик по количеству ограничений [@grune_parsingtechniques_en_1990, 19].

**Определение**: _Тип 0_ --- класс неограниченных грамматик с фазовой структурой (см. выше).

**Определение**: _Тип 1 (монотонные)_ --- класс грамматик, у которых не правил, левая часть которых состояла бы из большего числа символов, чем правая ($\forall (P, Q) \in R, 1 \leq P \leq Q$).

**Определение**: _Тип 1 (контекстно-зависимые, context-sensitive, CS)_ --- класс грамматик, все правила которых являются контекстно-зависимыми, то есть в них только один нетерминальный символ из левой части заменяется непустым наборов символов в правой части, а другие остаются в том же составе и порядке ($(P, Q) = (\alpha A \beta, \alpha \gamma \beta), A \in V = V_N \cup V_T, \alpha, \beta \in V^*, \gamma \in V^+$).

**Определение**: _Тип 2 (контекстно-независимые, context-free, CF)_ --- класс контекстно-зависимых грамматик с пустым контекстом ($\forall (P, Q) \in R, (P, Q) = (A, \gamma), A \in V, \gamma \in V^+$) [@grune_parsingtechniques_en_1990, 23].

**Определение**: Нетерминальный символ, язык которого содержит $\epsilon$, называется _обнуляемым (nullable)_.
# Grune, Dick, and Ceriel J. H. Jacobs. _Parsing Techniques: A Practical Guide_. 1990.

**Определение**: _Парсинг_ --- процесс структурирования линейного предствления в соответствии с некоторой грамматикой [@grune_parsingtechniques_en_1990, 1].`;

for (let i = 0; i < 100; i++) {
  if (fs.existsSync(`input.txt`)) {
    fs.rmSync(`input.txt`);
  }

  const f = fs.openSync(`input/input-${i}.txt`, "w");
  for (let j = 0; j < 1000; j++) {
    fs.writeSync(f, snippet);
  }
  fs.closeSync(f);
}
