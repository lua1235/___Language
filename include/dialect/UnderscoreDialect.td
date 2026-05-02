#ifndef MYLANG_DIAG
#define MYLANG_DIAG

include "mlir/IR/DialectBase.td"

def Underscore_Dialect : Dialect {
  let name = "underscore";
  let summary = "High level dialect in correspondance with the Underscore AST";
  let cppNamespace = "underscore";
}

#endif
