#!/bin/bash

yarn build:css
rails assets:precompile
bin/dev