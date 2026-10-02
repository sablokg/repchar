# repchar

- Microbial genotyping and repeat analyzer for AMR
- plotting also included
- extension of cag-repeat analyzer for all types of repeats 
- Only supports whole genome. 


```
cargo build
```

```
                                                                                                                                       
                                                                                                       _|                              
                                                           _|  _|_|     _|_|     _|_|_|       _|_|_|   _|_|_|       _|_|_|   _|  _|_|  
                                                           _|_|       _|_|_|_|   _|    _|   _|         _|    _|   _|    _|   _|_|      
                                                           _|         _|         _|    _|   _|         _|    _|   _|    _|   _|        
                                                           _|           _|_|_|   _|_|_|       _|_|_|   _|    _|     _|_|_|   _|        
                                                                                 _|                                                    
                                                                                 _|                                                    
Microbial Repeat Analyzer
       ************************************************
       Author Gaurav Sablok,
       Email: gsablok@proton.me
      ************************************************

Usage: repchar <COMMAND>

Commands:
  window           Window Repeat Analyzer
  specific-window  Specific Repeat Analyzer
  table            Table analyzer
  count            Stretch Count analyzer
  help             Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Examples

All commands read a plain FASTA-style file (headers starting with `>` are
skipped, every other line is treated as sequence).

Example input, `genome.fasta`:

```
>contig_1
ATGCGATCGATCGATCGAAATTTAAATTTGGCC
>contig_2
CGATCGATCGTAGCTAGCTAGCTTTTAAAAGGCC
```

### `window` — count every k-length window across the whole file

```
repchar window genome.fasta 4 4
```

- `genome.fasta` — input sequence file
- `4` — window size (k-mer length)
- `4` — number of threads

Writes `counts.txt`, one `<window>\t<count>` pair per line, e.g.:

```
ATCG	5
CGAT	4
TTTA	2
```

### `specific-window` — count occurrences of one exact repeat/motif

```
repchar specific-window genome.fasta ATCG 4
```

- `genome.fasta` — input sequence file
- `ATCG` — the exact motif to search for
- `4` — number of threads

Prints the total number of matches found, e.g.:

```
The count of the specific genotyping ATCG is 5
```

### `table` — group and tabulate all k-length windows

```
repchar table genome.fasta 3 4
```

- `genome.fasta` — input sequence file
- `3` — window size (k-mer length)
- `4` — number of threads

Writes `path.txt` with each window, its group members, and group size.

### `count` — find stretches of a repeated nucleotide/motif

```
repchar count genome.fasta A 4
```

- `genome.fasta` — input sequence file
- `A` — nucleotide or motif to look for runs of
- `4` — required run length (e.g. `AAAA`)

Writes `pathrep.txt` with `<match>\t<start>\t<end>` for every stretch found
in every sequence line, e.g.:

```
AAAA	18	22
AAAA	26	30
```

Gaurav Sablok \
gsablok@proton.me
