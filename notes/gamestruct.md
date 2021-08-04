# The game struct

This struct is used to represent a game. It holds
all the info about it that are important to the
gameplay. It's planned to be like this:

|Full description of data type |Represented value             |
|------------------------------|------------------------------|
| Unsigned integer (8 bits)    | Review score                 |
| Unsigned integer (32 bits)   | Game budget                  |
| Unsigned integer (32 bits)   | Game earnings                |
| Custom enum                  | Released platforms           |
| Boolean                      | Uses engine?                 |
| Custom struct                | Used engine                  |