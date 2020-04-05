let s:JOB = SpaceVim#api#import('job')

let g:fuzzy#ext#help#sources = []

let s:collect_tags_exe = fnamemodify(expand('<sfile>'), ':p:h:h:h:h') . '\fuzzy\target\release\collect_tags.exe'
let s:fuzzy_exe = fnamemodify(expand('<sfile>'), ':p:h:h:h:h') . '\fuzzy\target\release\fuzzy.exe'

function! s:stdout(id, data, event) abort
    let g:fuzzy#ext#help#sources += filter(a:data, '!empty(v:val)')
endfunction
function! s:exit(...) abort
    call fuzzy#redraw()
endfunction
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
    let g:fuzzy#ext#help#sources = []
    call s:JOB.start([s:collect_tags_exe, &rtp],
                \ {
                \ 'on_stdout' : function('s:stdout'),
                \ 'on_exit' : function('s:exit'),
                \ }
                \ )
    return {
                \ 'name' : 'help',
                \ 'async' : v:true,
                \ 'result' : 'g:fuzzy#ext#help#sources'
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

