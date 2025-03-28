" Place this file in $HOME/.vim/plugin

" ----- .his -----
" Method
hi ZXCHisHttpMethod             guifg=Green

" Protocol
hi ZXCHisProtocolHttps          guifg=Blue
hi ZXCHisProtocolHttp           guifg=Red

" Status Code
hi ZXCHisScodeSwitch            guifg=Blue
hi ZXCHisScodeSuccess           guifg=Green
hi ZXCHisScodeRedirect          guifg=Yellow
hi ZXCHisScodeClientError       guifg=Red
hi ZXCHisScodeServerError       guifg=Red

" Content Length
hi ZXChttpContentLength         guifg=Blue

" Host
hi ZXCHost                      guifg=Grey

" Uri
hi ZXCUri                       guifg=Yellow

" ----- .req -----
" Method
hi ZXCReqMethod                 guifg=Green

" URI
hi ZXCReqUri                    guifg=Orange

" Version
hi ZXCReqHttpVersion            guifg=Blue

" Header Key
hi ZXCReqHeaderKey              guifg=Yellow

" Header Value
hi ZXCReqHeaderValue            guifg=Orange

" ----- .res -----
" Status Code
hi ZXCResScodeSwitch            guifg=Blue
hi ZXCResScodeSuccess           guifg=Green
hi ZXCResScodeRedirect          guifg=Yellow 
hi ZXCResScodeClientError       guifg=Red
hi ZXCResScodeServerError       guifg=Red

" Http version
hi ZXCResHttpVersion            guifg=Blue

" Header Key
hi ZXCResHeaderKey              guifg=Orange

" Header Value
hi ZXCResHeaderValue            guifg=Yellow

" ----- Interceptor -----
hi ZXCIStatusLineOn             guibg=red
hi ZXCIStatusLineOff            guibg=black
