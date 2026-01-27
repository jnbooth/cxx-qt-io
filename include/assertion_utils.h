#include <cxx-qt-lib/assertion_utils.h>

#define assert_shared_pointer_type(typeName)                                   \
  assert_alignment_and_size(typeName, { ::std::size_t a0; });                  \
                                                                               \
  static_assert(!::std::is_trivially_copy_assignable<typeName>::value);        \
  static_assert(!::std::is_trivially_copy_constructible<typeName>::value);     \
  static_assert(!::std::is_trivially_destructible<typeName>::value);           \
  static_assert(::std::is_move_constructible<typeName>::value);                \
  static_assert(QTypeInfo<typeName>::isRelocatable);

#define assert_plain_struct(typeName, members)                                 \
  assert_alignment_and_size(typeName, members);                                \
  static_assert(::std::is_trivially_copy_assignable<typeName>::value);         \
  static_assert(::std::is_trivially_copy_constructible<typeName>::value);      \
  static_assert(::std::is_trivially_destructible<typeName>::value);            \
  static_assert(::std::is_move_constructible<typeName>::value);                \
  static_assert(QTypeInfo<typeName>::isRelocatable);

#define assert_iter_type(typeName)                                             \
  assert_alignment_and_size(typeName, {                                        \
    ::std::size_t iter;                                                        \
    ::std::size_t end;                                                         \
  });                                                                          \
  static_assert(!::std::is_trivially_copy_assignable<typeName>::value);        \
  static_assert(!::std::is_trivially_copy_constructible<typeName>::value);     \
  static_assert(::std::is_trivially_destructible<typeName>::value);            \
  static_assert(::std::is_move_constructible<typeName>::value);                \
  static_assert(QTypeInfo<typeName>::isRelocatable);
