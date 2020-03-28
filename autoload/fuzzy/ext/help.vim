let s:fuzzy_exe = fnamemodify(expand('<sfile>'), ':p:h:h:h:h') . '\fuzzy\target\release\fuzzy.exe'
function! s:get_help_tags() abort
    let tags = []
    for file in split(&rtp, ',')
        if filereadable(file . '/doc/tags')
            let lines = readfile(file . '/doc/tags')
            for line in lines
                let tag= split(line)[0]
                call add(tags, tag)
            endfor
        endif
    endfor
    return tags
endfunction

function! fuzzy#ext#help#sources()
    return {
                \ 'name' : 'help',
                \ 'async' : v:false,
                \ 'result' : s:get_help_tags()
                \ }
endfunction


function! fuzzy#ext#help#syntax()
    syntax clear
endfunction

function! fuzzy#ext#help#match(items, expr)
    if empty(a:expr)
        return a:items
    endif
    let cmd = s:fuzzy_exe . ' full_string ' . a:expr
    return systemlist(cmd, join(a:items, "\n"))
endfunction

function! fuzzy#ext#help#accept(line)
    exe 'help ' . a:line
endfunction

