# snake-http

## A recruitement task

In the end, working a full-time job I wasn't able to write it in time, but decided to finish it anyways. It was pretty fun and I learned a lot along the way.

Specification from the recruiter below:

### Translation to english:

1. There is a single global board on which a snake moves. The user can view this board (in the form of ASCII art) by sending a GET request to /snake.
2. The snake moves at a fixed interval based on commands received from the user. Commands are sent as POST requests to /snake/:direction, where :direction can be one of left, right, bottom, or top. When the snake passes through a wall, it appears on the opposite side of the board.
3. After moving, a fruit may appear on an empty space.
4. If the snake's head lands on the same space as a fruit, it is eaten, and the snake grows by one unit.
5. If the snake's head lands on the same space as the rest of its body, the game ends, and the board is reset to its initial state.

The selection of the next move is done as follows:

1. Discard all directions opposite to the current one.
2. If there are no directions left, the snake moves in the same direction as before.
3. From the remaining directions (including repetitions), choose one randomly.

For example, if the snake is moving left and we receive [right, left, left, down, up], we discard right (opposite to the current direction) and randomly choose from the remaining 4 directions. The chance of choosing left is 50%, while the chance of choosing down or up is 25%.

### Original:

1. Istnieje jedna globalna plansza, po której porusza się wąż. Użytkownik może zobaczyć tę planszę (w formie ASCII-arta) poprzez wysłanie zapytania GET /snake.
2. Ruch węża odbywa się co pewien stały okres na podstawie zebranych od użytkownika poleceń. Polecenia wysyłane są jako POST /snake/:direction, gdzie :direction może być jedno z left, right, bottom, top. Po przejściu przez ścianę wąż pojawia się po przeciwnej stornie planszy.
3. Po wykonaniu ruchu możliwe jest pojawienie się owocu na pustym polu.
4. Jeśli po ruchu głowa węża znajdzie się na tym samym polu co owoc, jest on zjadany i wąż rośnie o jedną jednostkę.
5. Jeśli po ruchu głowa węża znajdzie się na tym samym polu co reszta ciała węża, gra się kończy i stan planszy jest resetowany do stanu początkowego.

Wybór kolejnego ruchu odbywa się w następujący sposób:

1. Odrzuć wszystkie kierunki przeciwne do obecnego.
2. Jeśli nie pozostało żadnych, to wąż porusza się w tym samym kierunku co ostatnio.
3. Z pośród wszystkich pozostałych kierunków (wraz z powtórzeniami), wylosuj jeden.

Np. wąż porusza się w lewo, dostaliśmy [right, left, left, down, up]. Odrzucamy right (przeciwny do obecnego) i z pozostałych 4 losujemy 1 wartość. Szansa na wybranie left to 50%, natomiast na down oraz up 25%.
