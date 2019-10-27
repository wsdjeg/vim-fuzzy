let s:CMP = SpaceVim#api#import('vim#compatible')

function! fuzzy#ext#message#sources()
    return {
                \ 'name' : 'message',
                \ 'async' : v:false,
                \ 'result' : split(s:CMP.execute('message'), '\n')
                \ }
endfunction


function! fuzzy#ext#message#syntax()
    syntax clear
endfunction

function! fuzzy#ext#message#match(str, expr)
    if a:str =~ a:expr
        return v:true
    endif
endfunction

function! fuzzy#ext#message#accept(line)
    let @" = a:line
    echohl ModeMsg
    echo 'Yanked'
    echohl None
endfunction

