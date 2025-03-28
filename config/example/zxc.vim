try
    if !check_session#Check() || !exists("g:zxc_winame")
        finish
    endif
catch
    finish
endtry

func LoadUserConfig()
    if g:zxc_winame =~ "interceptor"
        nnoremap <silent><leader>i :InterToggle<CR>
        nnoremap <silent><leader>f :InterForward<CR>
        nnoremap <silent><leader>fa :InterForwardAll<CR>
        nnoremap <silent><leader>fr :InterForwardWithRes<CR>
        nnoremap <silent><leader>s :Showq<CR>
        nnoremap <silent><leader>d :DropMsg<CR>
    elseif g:zxc_winame =~ "repeater"
        autocmd BufWritePost *.req,*.wreq RepeaterSend
    endif
endfunc

" Codec
vnoremap <leader>eb "xc<scriptcmd>:EBase64<CR><esc>
vnoremap <leader>db "xc<scriptcmd>:DBase64<CR><esc>
vnoremap <leader>eu "xc<scriptcmd>:EUrl<CR><esc>
vnoremap <leader>du "xc<scriptcmd>:DUrl<CR><esc>
vnoremap <leader>eku "xc<scriptcmd>:EUrlAll<CR><esc>
vnoremap <leader>dku "xc<scriptcmd>:DUrlAll<CR><esc>

augroup UserConfig
    autocmd VimEnter * call LoadUserConfig()
augroup End

