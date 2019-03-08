# ps05 - thegrep by The Brogrammers(Daniel Evora and Peter Morrow)
ps05-thegrep-the-brogrammers created by GitHub Classroom

**Design**:
thegrep(Tar Heel egrep) is based off the class grep pattern-matching search tool created by Ken Thompson. 
Currently, our implementation of this classic program is split into three files: main, tokenizer, and parser.
The Tokenizer is responsible for taking a regular expression from the command line and turning it into meaningful tokens 
to be used by the Parser. The Parser then parses these tokens and creates an Abstract Syntax Tree (AST) to be used
in a future part of the program.

**Contributions**:
The work for this project was split evenly. Daniel set up the support for help flag flag in main and established the tokenizer
file. Peter aided in the creation of the Tokenizer and also established the Parser, which Daniel helped complete. Both members
created unit tests to test the functionality of both the Tokenizer and Parser and their accompanying helper methods. 
Collobration was done in person with both members alternating between who was coding and who was providing ideas of what
to implement. 
