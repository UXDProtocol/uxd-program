#!/bin/sh

# initialize the Controller for UXD
yarn ts-mocha --require tests/fixtures.ts ./tsconfig.json -t 500000 tests/tests_regular/test_controller_uxd.ts --reporter mochawesome --require mochawesome/register --reporter-options quiet=true,reportTitle=uxdprogram-tests-controller --trace-warnings

# Parallel tests
yarn ts-mocha --require tests/fixtures.ts --require tests/tests_parallel/hooks.ts --parallel ./tsconfig.json -t 500000 tests/tests_parallel/*.ts --reporter mochawesome --require mochawesome/register --reporter-options quiet=true,reportTitle=uxdprogram-tests-parallel --trace-warnings 

# Test the bits that need both the controller and depositories, but cannot be done in parallel
yarn ts-mocha --require tests/fixtures.ts ./tsconfig.json -t 500000 tests/tests_regular/test_controller_uxd.ts --reporter mochawesome --require mochawesome/register --reporter-options quiet=true,reportTitle=uxdprogram-tests-depositories-controller-interactions --trace-warnings
