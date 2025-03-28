try
    if !check_session#Check()
        finish
    endif
catch
    finish
endtry

nnoremap <buffer><silent>hl :HistoryIndent<CR>
nnoremap <buffer><silent><leader>r :HistoryToRepeater<CR>
nnoremap <buffer><silent><leader>z :HistoryToFuzz<CR>
nnoremap <buffer><silent><leader>q :HistoryToSql<CR>

nnoremap <buffer><silent><leader>af :ApplyFilters<CR>
nnoremap <buffer><silent><leader>sf :ShowFilters<CR>

nnoremap <buffer><silent><leader>ah :AddToHostScope<CR>
nnoremap <buffer><silent><leader>ch :ClearHostScope<CR>
nnoremap <buffer><silent><leader>eh :EditHostScope<CR>
nnoremap <buffer><silent><leader>sh :ShowHostScope<CR>

nnoremap <buffer><silent><leader>cs :ClearScode<CR>
nnoremap <buffer><silent><leader>es :EditScode<CR>
nnoremap <buffer><silent><leader>ss :ShowScode<CR>

nnoremap <buffer><silent><leader>ec :EditConfig<CR>
nnoremap <buffer><silent><leader>rc :ReloadConfig<CR>

let g:conceal=50
