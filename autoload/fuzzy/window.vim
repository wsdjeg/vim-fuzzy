let s:bufnr = 0

function! fuzzy#window#open(...)

  if s:bufnr != 0 && bufexists(s:bufnr)
    exe 'bd ' . s:bufnr
  endif
  botright split __fuzzy__
  let lines = &lines * 30 / 100
  exe 'resize ' . lines
  setlocal buftype=nofile bufhidden=wipe nobuflisted nolist noswapfile nowrap cursorline nospell nonu norelativenumber winfixheight nomodifiable
  set filetype=Fuzzy
  let s:bufnr = bufnr('%')
  return s:bufnr
endfunction


function! fuzzy#window#close()

    close

endfunction
