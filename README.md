# Polya
A polygon designer.

## Algorytm
Przy każdym ruchu wierzchołka (lub punktu kontrolnego krzywej Béziera) wywoływana
jest funkcja `resolve_constraints`, która stara się ustawić wierzchołki w
sposób spełniający wszystkie ustawione ograniczenia.

Wykonaniem tego zajmują się pętle, które w pseudokodzie można przedstawić następująco:
```python
v = start_v
while v.next != starting_v:
    if !v.check_constraints():
        v.apply_constraints()
    v = v.next
v = start_v
while v.prev != starting_v:
    if !v.check_constraints():
        v.apply_constraints()
    v = v.prev
```
Innymi słowy, idziemy od startowego wierzchołka w jedną stronę i
jeżeli napotkamy jakiś wierzchołek, którego pozycja nie zgadza się z jego ogarniczeniami
(np. ma różną współrzędną `x` od poprzedniego wierzchołka, mimo że łącząca je krawędź ma
być pionowa), to przesuwamy go tak, aby to ograniczenie spełnić. Następnie przechodzimy
w drugą stronę i wykonujemy analogiczne poprawki.

Przy niektórych bardziej skomplikowanych zestawach ograniczeń jedno takie "dwustronne"
przejście może nie wystarczyć do przesunięcia wszystkich wierzchołków w odpowiednie pozycje.
Algorytm wykonuje więc tyle takich przejść, ile potrzeba - z limitem 64 iteracji
aby nie utknąć w nieskończonej pętli. Jeżeli po którejś iteracji wszystkie warunki będą
spełnione, algorytm kończy działania. W przeciwnym przypadku, żądany ruch wierzchołkiem
nie wykonuje się.
