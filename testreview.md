The tests in tests/robot/swing , tests/robot/swt and tests/robot/rcp are the proper acceptance tests.
But there are also tests in tests/robot/swing_testapp and tests/robot/swt_testapp
Please review the tests in tests/robot/swing_testapp and tests/robot/swt_testapp and check if they can be deleted.
If those tests contain additional features not covered by the real acceptance tests, please add them to the acceptance tests and rerun the tests via "uv run" (no dry run)
If the tests in tests/robot/swing_testapp and tests/robot/swt_testapp do not cover any additional features, please delete them