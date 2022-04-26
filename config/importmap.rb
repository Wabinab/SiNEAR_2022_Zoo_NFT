# Pin npm packages by running ./bin/importmap

pin "application", preload: true
pin "@hotwired/turbo-rails", to: "turbo.min.js", preload: true
pin "@hotwired/stimulus", to: "stimulus.min.js", preload: true
pin "@hotwired/stimulus-loading", to: "stimulus-loading.js", preload: true
pin_all_from "app/javascript/controllers", under: "controllers"
pin_all_from "app/javascript/custom", under: "custom"
pin "near-api-js", to: "https://ga.jspm.io/npm:near-api-js@0.44.2/lib/browser-index.js"
pin "base-x", to: "https://ga.jspm.io/npm:base-x@3.0.9/src/index.js"
pin "bn.js", to: "https://ga.jspm.io/npm:bn.js@5.2.0/lib/bn.js"
pin "borsh", to: "https://ga.jspm.io/npm:borsh@0.6.0/lib/index.js"
pin "bs58", to: "https://ga.jspm.io/npm:bs58@4.0.1/index.js"
pin "buffer", to: "https://ga.jspm.io/npm:@jspm/core@2.0.0-beta.19/nodelibs/browser/buffer.js"
pin "capability", to: "https://ga.jspm.io/npm:capability@0.2.5/index.js"
pin "capability/es5", to: "https://ga.jspm.io/npm:capability@0.2.5/es5.js"
pin "crypto", to: "https://ga.jspm.io/npm:@jspm/core@2.0.0-beta.19/nodelibs/browser/crypto.js"
pin "depd", to: "https://ga.jspm.io/npm:depd@2.0.0/lib/browser/index.js"
pin "error-polyfill", to: "https://ga.jspm.io/npm:error-polyfill@0.1.3/index.js"
pin "http-errors", to: "https://ga.jspm.io/npm:http-errors@1.8.1/index.js"
pin "inherits", to: "https://ga.jspm.io/npm:inherits@2.0.4/inherits_browser.js"
pin "js-sha256", to: "https://ga.jspm.io/npm:js-sha256@0.9.0/src/sha256.js"
pin "mustache", to: "https://ga.jspm.io/npm:mustache@4.2.0/mustache.js"
pin "o3", to: "https://ga.jspm.io/npm:o3@1.0.3/index.js"
pin "process", to: "https://ga.jspm.io/npm:@jspm/core@2.0.0-beta.19/nodelibs/browser/process-production.js"
pin "safe-buffer", to: "https://ga.jspm.io/npm:safe-buffer@5.2.1/index.js"
pin "setprototypeof", to: "https://ga.jspm.io/npm:setprototypeof@1.2.0/index.js"
pin "statuses", to: "https://ga.jspm.io/npm:statuses@1.5.0/index.js"
pin "text-encoding-utf-8", to: "https://ga.jspm.io/npm:text-encoding-utf-8@1.0.2/lib/encoding.lib.js"
pin "toidentifier", to: "https://ga.jspm.io/npm:toidentifier@1.0.1/index.js"
pin "tweetnacl", to: "https://ga.jspm.io/npm:tweetnacl@1.0.3/nacl-fast.js"
pin "u3", to: "https://ga.jspm.io/npm:u3@0.1.1/index.js"
pin "bootstrap", to: "https://ga.jspm.io/npm:bootstrap@5.1.3/dist/js/bootstrap.esm.js"
pin "@popperjs/core", to: "https://ga.jspm.io/npm:@popperjs/core@2.11.5/lib/index.js"
