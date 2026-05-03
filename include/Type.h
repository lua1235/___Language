#pragma once

#include <cstdint>
#include <memory>
#include <string>
#include <vector>

class TypeAllocator;
enum PrimTypes {
    Unknown = 0, // All instances of unknown should be eliminated by typechecking pass
    Void,
    Bool,
    Char,
    Int,
    Real,
    Struct,
    Func // Special, just cause it's convenient to think of functions as a type
};

struct Type {
    PrimTypes base = Unknown;
    std::vector<std::shared_ptr<Type>> inner;

    //This allows types to be stored in a set
    bool operator<(const Type&) const;


private :
    Type(PrimTypes base=Unknown, std::vector<std::shared_ptr<Type>> inner = {}) :
        base{base}, inner{inner} {}
};
