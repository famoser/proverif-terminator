# ProVerif Terminator

[![Cargo](https://github.com/famoser/proverif-terminator/actions/workflows/build.yml/badge.svg)](https://github.com/famoser/proverif-terminator/actions/workflows/build.yml)

Pipe the output of ProVerif (with `set verboseRules = true.` or even `set verboseBase = true`) into `proverif-terminator`, and enjoy some condensed information, which (may) help to debug non-termination issues.

Default output, which focuses on the selected hypothesis (often enough to detect whether ProVerif is looping):

```
493 (92c, 356h, 17q)	Selected: hypothesis output2(v_8,res_GVI(e_id[!1 = @sid],6))
494 (92c, 357h, 16q)	Selected: hypothesis output2(res_GVI(e_id[!1 = @sid],6),v_8)
495 (92c, 358h, 15q)	Selected: hypothesis mess2(cell_BB(e_id[!1 = @sid],6),v_8,cell_BB(e_id[!1 = @sid],6),(@1_j_1,@1_k_1,@1_d_1))
496 (92c, 359h, 14q)	Selected: hypothesis mess2(cell_BB(e_id[!1 = @sid],6),(@1_j_1,@1_k_1,@1_d_1),cell_BB(e_id[!1 = @sid],6),v_8)
497 (92c, 359h, 14q)	Selected: hypothesis mess2(cell_ident(e_id[!1 = @sid]),5,cell_ident(e_id[!1 = @sid]),5)
498 (92c, 359h, 14q)	Selected: hypothesis mess2(cell_ident(e_id[!1 = @sid]),5,cell_ident(e_id[!1 = @sid]),5) (again)
```

Features:
- Condense output of ProVerif, to make log analysis more practical (e.g, see above)
- Additionally print the full selected query, or the new queue entries
- Best-effort recovery of where clause originates (however, based on string matching, hence not very effective)
- Best-effort detection of high counters and cycles


Future ideas:
- Patch ProVerif to print query before renaming the variables, which would make history reconstruction easier
- Detect high choice constructs (e.g. `choice[20,20]`) as another way of detecting counters