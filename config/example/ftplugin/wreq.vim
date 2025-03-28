try
    if !check_session#Check()
        finish
    endif
catch
    finish
endtry

nnoremap <buffer> <silent><leader>r :WsSendToRepeater<CR>
