# SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

add_subdirectory(standalone)

function(add_acceptance_tests TEST_NAME)
    set(TARGET_NAME tests_${TEST_NAME})

    set(CPP_TEST_NAME test_${TEST_NAME}_cpp_tests)
    add_test(NAME ${CPP_TEST_NAME} COMMAND $<TARGET_FILE:${TARGET_NAME}>)

    set_tests_properties(
       ${CPP_TEST_NAME}
       PROPERTIES
       ENVIRONMENT_MODIFICATION "${RUNTIME_ENV}"
    )
endfunction()

add_acceptance_tests(standalone)
