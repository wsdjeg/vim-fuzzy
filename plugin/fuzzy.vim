let s:save_cpo = &cpo
set cpo&vim

command! -nargs=* -complete=custom,fuzzy#complete Fuzzy call fuzzy#start(<q-args>)

let &cpo = s:save_cpo
unlet s:save_cpo
