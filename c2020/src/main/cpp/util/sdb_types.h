#pragma once

#include <string>
#include <type_traits>

#include <frc/smartdashboard/SmartDashboard.h>
#include <wpi/StringRef.h>

namespace team114 {
namespace c2020 {

// no string literals in templates yet, so have to resort to an ugly macro
// defining an anonymous type. When/if those come, we can unify all types
// into one beautiful SFINAE dance, without any macros

#define SDB_NUMERIC(type, key_ident)                               \
    struct __SdbKey_##key_ident {                                  \
        static const std::string GetName() { return #key_ident; }; \
    };                                                             \
    team114::c2020::SdbNumeric<type, __SdbKey_##key_ident>

template <typename NumericTy, typename KeyTy>
struct SdbNumeric {
    SdbNumeric() : value() { Update(); };
    SdbNumeric(const NumericTy& val) : value(val) { Update(); }
    const NumericTy& operator=(const NumericTy& val) {
        value = val;
        Update();
        return val;
    }
    operator NumericTy() const { return value; }

   private:
    NumericTy value;
    void Update() { frc::SmartDashboard::PutNumber(KeyTy::GetName(), value); }
};

#define SDB_BOOL(key_ident)                                        \
    struct __SdbKey_##key_ident {                                  \
        static const std::string GetName() { return #key_ident; }; \
    };                                                             \
    team114::c2020::SdbBool<__SdbKey_##key_ident>

template <typename KeyTy>
struct SdbBool {
    SdbBool() : value() { Update(); };
    SdbBool(const bool& val) : value(val) { Update(); }
    const bool& operator=(const bool& val) {
        value = val;
        Update();
        return val;
    }
    operator bool() const { return value; }

   private:
    bool value;
    void Update() { frc::SmartDashboard::PutNumber(KeyTy::GetName(), value); }
};

}  // namespace c2020
}  // namespace team114
