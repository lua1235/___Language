/*===- TableGen'erated file -------------------------------------*- C++ -*-===*\
|*                                                                            *|
|* Dialect Declarations                                                       *|
|*                                                                            *|
|* Automatically generated file, do not edit!                                 *|
|* From: UnderscoreOps.td                                                     *|
|*                                                                            *|
\*===----------------------------------------------------------------------===*/

namespace underscore {

/// High level dialect in correspondance with the Underscore AST
class UnderscoreDialect : public ::mlir::Dialect {
  explicit UnderscoreDialect(::mlir::MLIRContext *context);

  void initialize();
  friend class ::mlir::MLIRContext;
public:
  ~UnderscoreDialect() override;
  static constexpr ::llvm::StringLiteral getDialectNamespace() {
    return ::llvm::StringLiteral("underscore");
  }
};

} // namespace underscore
MLIR_DECLARE_EXPLICIT_TYPE_ID(underscore::UnderscoreDialect)
