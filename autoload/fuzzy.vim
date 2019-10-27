let s:source = ''
let s:match = ''
let s:bufnr = -1
let s:MPT = SpaceVim#api#import('prompt')
let s:BUFFER = SpaceVim#api#import('vim#buffer')

function! s:fuzzy_handle(expr) abort
    if s:source.async
        let result = eval(s:source.result)
    else
        let result = s:source.result
    endif
    let result = filter(deepcopy(result), 'call(s:match, [v:val, a:expr])')
    call s:BUFFER.buf_set_lines(s:bufnr, 0 , -1, 0, result)
    redrawstatus
endfunction

function! fuzzy#redraw()
  call s:fuzzy_handle(s:MPT._prompt.begin . s:MPT._prompt.cursor .s:MPT._prompt.end)
endfunction

function! s:close_buffer() abort
    call fuzzy#window#close()
endfunction

let s:MPT._onclose = function('s:close_buffer')

function! fuzzy#start(...)
    let ext = get(a:000, 0, 'file')
    let s:source = fuzzy#ext#{ext}#sources()
    let s:bufnr = fuzzy#window#open()
    let s:match =  'fuzzy#ext#'. ext . '#match'
    let s:MPT._handle_fly = function('s:fuzzy_handle')
    if !s:source.async
        call fuzzy#redraw()
    endif
    call s:MPT.open()
endfunction

function! s:next_item() abort
    if line('.') == line('$')
        normal! gg
    else
        normal! j
    endif
    call s:MPT._build_prompt()
    redrawstatus
endfunction
function! s:previous_item() abort
    if line('.') == 1
        normal! G
    else
        normal! k
    endif
    call s:MPT._build_prompt()
    redrawstatus
endfunction
function! s:open_item() abort
    if getline('.') !=# ''
        call s:MPT._clear_prompt()
        let s:MPT._quit = 1
        let line = getline('.')
        call fuzzy#window#close()
        noautocmd normal! :
        call call('fuzzy#ext#' . s:source.name . '#accept', [line])
    endif
endfunction
let s:MPT._function_key = {
            \ "\<Tab>" : function('s:next_item'),
            \ "\<C-j>" : function('s:next_item'),
            \ "\<ScrollWheelDown>" : function('s:next_item'),
            \ "\<S-tab>" : function('s:previous_item'),
            \ "\<C-k>" : function('s:previous_item'),
            \ "\<ScrollWheelUp>" : function('s:previous_item'),
            \ "\<Return>" : function('s:open_item'),
            \ }
if has('nvim')
    call extend(s:MPT._function_key, 
                \ {
                \ "\x80\xfdJ" : function('s:previous_item'),
                \ "\x80\xfc \x80\xfdJ" : function('s:previous_item'),
                \ "\x80\xfc@\x80\xfdJ" : function('s:previous_item'),
                \ "\x80\xfc`\x80\xfdJ" : function('s:previous_item'),
                \ "\x80\xfdK" : function('s:next_item'),
                \ "\x80\xfc \x80\xfdK" : function('s:next_item'),
                \ "\x80\xfc@\x80\xfdK" : function('s:next_item'),
                \ "\x80\xfc`\x80\xfdK" : function('s:next_item'),
                \ }
                \ )
endif

function! fuzzy#statusline()
    let rst = ''
    if line('$') > 1
        let rst = '(' . line('.') . '/' . line('$') . ')'
    endif
    return s:source.name . rst
endfunction

function! fuzzy#complete(...)
    let exts = split(globpath(&rtp, 'autoload/fuzzy/ext/*'), "\n")
    return join(map(exts,"fnamemodify(v:val, ':t:r')"), "\n")
endfunction
