let s:JOB = SpaceVim#api#import('job')

let g:fuzzy#ext#file#sources = []

function! s:stdout(id, data, event) abort
    let g:fuzzy#ext#file#sources += filter(a:data, '!empty(v:val)')
endfunction
function! s:exit(...) abort
    call fuzzy#redraw()
endfunction

function! fuzzy#ext#file#sources()
    let g:fuzzy#ext#file#sources = []
    call s:JOB.start(['rg', '--hidden', '--files', '--glob', '!.git', '--glob', ''],
                \ {
                \ 'on_stdout' : function('s:stdout'),
                \ 'on_exit' : function('s:exit'),
                \ }
                \ )
    return {
                \ 'name' : 'file',
                \ 'async' : v:true,
                \ 'result' : 'g:fuzzy#ext#file#sources'
                \ }
endfunction


function! fuzzy#ext#file#syntax()
    syntax clear
    syntax case ignore
    syn match FileName /\([A-Z]:\)\?[^:]*:\d\+:\(\d\+:\)\?/
    hi def link FileName Comment
endfunction

function! fuzzy#ext#file#match(items, expr)
    let cmd = 'fuzzy\target\debug\fuzzy.exe file ' . a:expr
    return systemlist(cmd, join(a:items, "\n"))
endfunction

function! fuzzy#ext#file#accept(line)

    exe 'e ' . a:line

endfunction
