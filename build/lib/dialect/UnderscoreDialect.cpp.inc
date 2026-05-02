/*===- TableGen'erated file -------------------------------------*- C++ -*-===*\
|*                                                                            *|
|* Dialect Definitions                                                        *|
|*                                                                            *|
|* Automatically generated file, do not edit!                                 *|
|* From: UnderscoreOps.td                                                     *|
|*                                                                            *|
\*===----------------------------------------------------------------------===*/

MLIR_DEFINE_EXPLICIT_TYPE_ID(underscore::UnderscoreDialect)
namespace underscore {


UnderscoreDialect::UnderscoreDialect(::mlir::MLIRContext *context)
    : ::mlir::Dialect(getDialectNamespace(), context, ::mlir::TypeID::get<UnderscoreDialect>())
    
     {
  
  initialize();
}

UnderscoreDialect::~UnderscoreDialect() = default;


} // namespace underscore
