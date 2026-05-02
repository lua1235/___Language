#include "dialect/UnderscoreDialect.h"

void underscore::UnderscoreDialect::initialize() {
  // This registers every Op defined in TableGen with the MLIR context
  addOperations<
#define GET_OP_LIST
#include "dialect/UnderscoreOps.cpp.inc"
  >();
}
