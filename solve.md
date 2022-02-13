# Zadání 
1. V postupnosti 1, 9, 8, 9, 7, 3, 7, 6, 3, 9 sa kaˇzd ́y nasleduj ́uci ˇclen
zaˇc ́ınaj ́uc piatym ˇclenom rovn ́a poslednej cifre s ́uˇctu pˇredoˇsl ́ych
ˇstyroch ˇclenov. Zistite, ˇci sa vyskytn ́u eˇste raz ˇc ́ıslice 1, 9, 8, 9
bezprostredne za sebou.
2. V postupnosti 1, 9, 8, 9, 7, 3, 7, 6, 3, 9 sa kaˇzd ́y nasleduj ́uci ˇclen
   zaˇc ́ınaj ́uc piatym ˇclenom rovn ́a poslednej cifre s ́uˇctu pˇredoˇsl ́ych
   ˇstyroch ˇclenov. Zistite, ˇci sa vyskytn ́u eˇste raz ˇc ́ıslice 3, 0, 7, 1
   bezprostredne za sebou.

# Řešení
Začal jsem počítat sekvenci dále, ale byl jsem si méně a méně jist, že jsem neudělal chybu, proto jsem se rozhodl, že napíšu program, který bude počítat za mě.
## Pozorování
K sekvenci jsem si modře vypsal i součty a zeleně rozdíly dvou následujících součtů. Následně jsem vypozoroval, že v sekvenci se dá postupovat i zpětně pomocí odečtení součtu tří prvních prvků z libovolné čtveřice [a1, a2, a3, a4] od sumy všech jejich členů.