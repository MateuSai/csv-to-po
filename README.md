# csv-to-po
Convert csv files to po files plus the pot template file

This simple program generates a pot file and the corresponding po files from a a csv file with the translations

The usage is the following:
```
cargo run -- path_to_csv path_to_output_directory -p project_id
```

- The -p argument is opcional.
- if no output directory is provided, the current directory will be used
