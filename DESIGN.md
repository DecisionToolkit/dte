# Handling insertions

**Information item name** (A)

- `P` is present
- `N` is not present

**RowIndex** (B)

- `I` inside information item name
- `B` inside decision table's body

**JoinLine** (C)

- `S` shorter than the decision table's body
- `F` exactly the same length as decision table's body (full)

| (A) | (B) | (C) | Where to insert whitespace | Start row index | End row index  |
|-----|-----|-----|----------------------------|-----------------|----------------|
| `P` | `I` | `S` | information item name      | first           | join_row_index |
| `P` | `I` | `F` | whole decision table       | first           | last           |
| `P` | `B` | `S` | decision table's body      | join_row_index  | last           |
| `P` | `B` | `F` | decision table's body      | join_row_index  | last           |
| `-` | `-` | `-` | decision table's body      | first           | last           |

# Handling deletions

**Information item name** (A)

- `P` is present
- `N` is not present

**RowIndex** (B)

- `I` inside information item name
- `B` inside decision table's body

**JoinLine** (C)

- `S` shorter than the decision table's body
- `F` exactly the same length as decision table's body (full)

| (A) | (B) | (C) | Where to delete whitespaces if present          |
|-----|-----|-----|-------------------------------------------------|
| `P` | `I` | `S` | information item name                           |
| `P` | `I` | `F` | information item name and body when appropriate |
| `P` | `B` | `S` | decision table's body                           |
| `P` | `B` | `F` | whole decision table                            |
| `-` | `-` | `-` | decision table's body                           |
