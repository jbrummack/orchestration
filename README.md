# orchestration
_orchestration_ is a simple database server that can distribute tasks to multiple workers.
The integrated SQLite backend keeps your results safe and can be dumped via `.mode csv` to be used by standard tools.

## batch processing with persistence
Upload a batch of `/task`s by `post`ing them. _orchestration_ keeps track which unit of work has already been completed. 
## distribute tasks to multiple workers
Every worker can `get` tasks from the api via `/task` and `put` the results back into the database.

### Todos:
- user authentication
- cursors for syncing results
- improve the API
- write tests

### Possible features:
- add MySQL and Postgres support

