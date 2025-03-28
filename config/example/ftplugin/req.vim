try
    if !check_session#Check()
        finish
    endif
catch
    finish
endtry

nnoremap <buffer><silent><leader>e :EditBufVar<CR>
nnoremap <buffer><silent><leader>r :RequestToRepeater<CR>
nnoremap <buffer><silent><leader>z :RequestToFuzz<CR>
nnoremap <buffer><silent><leader>q :RequestToSql<CR>
