# Main save file layout

The save file layout of the game is planned to be like this:

|Full description of data type             |Represented value                         |
|------------------------------------------|------------------------------------------|
| String (taken from save file name)       | Player name                              |
| Unsigned integer (32 bits)               | Amount of created games                  |
| Signed integer (64 bits)                 | Bank balance                             |
| Unsigned integer (8 bits)                | Length of company name string            |
| String                                   | Company name                             |

Then, for each game developed, there will be a file containing its custom game
struct data (might become compressed in the future).

For info on the game struct, see [the notes about it](gamestruct.md).