### What is fgdb?

Fgdb is a library component shared among the various fg applications: fgcli, fgcolors, fggui and fgsvr: "fg" stands
for "filter-generator," and each of the applications contribute towards generating item filters for Path of Exile 1
and Path of Exile 2.

Fgdb uses the SQLite relational database as a data store.  Database update is accomplished using JSON; a separate JSON 
file exists for each table in the database.  When Fgdb::update is called, Fgdb compares the version of the JSON files 
currently in use with the current version stored in a master repository.  If the local JSON is out-of-date, the current 
version is downloaded from the master repository, the database is dropped, and a new database is created using the 
current JSON.

Fgdb maintains pricing data in a manner different to that described above.  During update, pricing data is obtained 
from POE.ninja, and converted to JSON using a format compatible with the pricing tables.  These tables are then deleted
and up-to-date pricing is loaded using the converted JSON.

The master repository is maintained by fgsvr.  Fgsvr can be configured to upload JSON for the database to a website
or to a local file store.  In turn, fgdb can be configured to obtain JSON from the web or from files in a local
drive.  For general use, fgdb should be configured to obtain JSON from the web; local repositories are used only for
testing purposes and will not work unless fgsvr is installed and configured to maintain the local store.

### Disclaimer

Fgdb is tested, but unstable, subject to frequent breaking changes.

### License

fgdb is licensed under the MIT License.
See [LICENSE](LICENSE).
