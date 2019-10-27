let s:CMP = SpaceVim#api#import('vim#compatible')
let s:STRING =SpaceVim#api#import('data#string')

function! s:get_buffer_list() abort
    let buffers = split(s:CMP.execute('buffers'), '\n')
    let lines = []
    for line in buffers
        let nr = split(line)[0]
        let b = s:STRING.fill_left(nr, len(string(bufnr('$'))))
        call add(lines, b . '  ' . fnamemodify(bufname(str2nr(nr)), ':~:.'))
    endfor
    return lines
endfunction


function! fuzzy#ext#buffer#sources()
    return {
                \ 'name' : 'buffer',
                \ 'async' : v:false,
                \ 'result' : s:get_buffer_list()
                \ }
endfunction


function! fuzzy#ext#buffer#syntax()
    syntax clear
endfunction

function! fuzzy#ext#buffer#match(str, expr)
    if a:str =~ a:expr
        return v:true
    endif
endfunction

function! fuzzy#ext#buffer#accept(line)
endfunction


