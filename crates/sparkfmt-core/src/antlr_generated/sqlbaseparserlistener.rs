#![allow(nonstandard_style)]
// Generated from /home/runner/work/ms-spark-formatter/ms-spark-formatter/grammar/SqlBaseParser.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::sqlbaseparser::*;

pub trait SqlBaseParserListener<'input> : ParseTreeListener<'input,SqlBaseParserContextType>{
/**
 * Enter a parse tree produced by {@link SqlBaseParser#compoundOrSingleStatement}.
 * @param ctx the parse tree
 */
fn enter_compoundOrSingleStatement(&mut self, _ctx: &CompoundOrSingleStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#compoundOrSingleStatement}.
 * @param ctx the parse tree
 */
fn exit_compoundOrSingleStatement(&mut self, _ctx: &CompoundOrSingleStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#singleCompoundStatement}.
 * @param ctx the parse tree
 */
fn enter_singleCompoundStatement(&mut self, _ctx: &SingleCompoundStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#singleCompoundStatement}.
 * @param ctx the parse tree
 */
fn exit_singleCompoundStatement(&mut self, _ctx: &SingleCompoundStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#beginEndCompoundBlock}.
 * @param ctx the parse tree
 */
fn enter_beginEndCompoundBlock(&mut self, _ctx: &BeginEndCompoundBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#beginEndCompoundBlock}.
 * @param ctx the parse tree
 */
fn exit_beginEndCompoundBlock(&mut self, _ctx: &BeginEndCompoundBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#compoundBody}.
 * @param ctx the parse tree
 */
fn enter_compoundBody(&mut self, _ctx: &CompoundBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#compoundBody}.
 * @param ctx the parse tree
 */
fn exit_compoundBody(&mut self, _ctx: &CompoundBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#compoundStatement}.
 * @param ctx the parse tree
 */
fn enter_compoundStatement(&mut self, _ctx: &CompoundStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#compoundStatement}.
 * @param ctx the parse tree
 */
fn exit_compoundStatement(&mut self, _ctx: &CompoundStatementContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code setVariableInsideSqlScript}
 * labeled alternative in {@link SqlBaseParser#setStatementInsideSqlScript}.
 * @param ctx the parse tree
 */
fn enter_setVariableInsideSqlScript(&mut self, _ctx: &SetVariableInsideSqlScriptContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code setVariableInsideSqlScript}
 * labeled alternative in {@link SqlBaseParser#setStatementInsideSqlScript}.
 * @param ctx the parse tree
 */
fn exit_setVariableInsideSqlScript(&mut self, _ctx: &SetVariableInsideSqlScriptContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#sqlStateValue}.
 * @param ctx the parse tree
 */
fn enter_sqlStateValue(&mut self, _ctx: &SqlStateValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#sqlStateValue}.
 * @param ctx the parse tree
 */
fn exit_sqlStateValue(&mut self, _ctx: &SqlStateValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#declareConditionStatement}.
 * @param ctx the parse tree
 */
fn enter_declareConditionStatement(&mut self, _ctx: &DeclareConditionStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#declareConditionStatement}.
 * @param ctx the parse tree
 */
fn exit_declareConditionStatement(&mut self, _ctx: &DeclareConditionStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#conditionValue}.
 * @param ctx the parse tree
 */
fn enter_conditionValue(&mut self, _ctx: &ConditionValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#conditionValue}.
 * @param ctx the parse tree
 */
fn exit_conditionValue(&mut self, _ctx: &ConditionValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#conditionValues}.
 * @param ctx the parse tree
 */
fn enter_conditionValues(&mut self, _ctx: &ConditionValuesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#conditionValues}.
 * @param ctx the parse tree
 */
fn exit_conditionValues(&mut self, _ctx: &ConditionValuesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#declareHandlerStatement}.
 * @param ctx the parse tree
 */
fn enter_declareHandlerStatement(&mut self, _ctx: &DeclareHandlerStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#declareHandlerStatement}.
 * @param ctx the parse tree
 */
fn exit_declareHandlerStatement(&mut self, _ctx: &DeclareHandlerStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#whileStatement}.
 * @param ctx the parse tree
 */
fn enter_whileStatement(&mut self, _ctx: &WhileStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#whileStatement}.
 * @param ctx the parse tree
 */
fn exit_whileStatement(&mut self, _ctx: &WhileStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#ifElseStatement}.
 * @param ctx the parse tree
 */
fn enter_ifElseStatement(&mut self, _ctx: &IfElseStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#ifElseStatement}.
 * @param ctx the parse tree
 */
fn exit_ifElseStatement(&mut self, _ctx: &IfElseStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#repeatStatement}.
 * @param ctx the parse tree
 */
fn enter_repeatStatement(&mut self, _ctx: &RepeatStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#repeatStatement}.
 * @param ctx the parse tree
 */
fn exit_repeatStatement(&mut self, _ctx: &RepeatStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#leaveStatement}.
 * @param ctx the parse tree
 */
fn enter_leaveStatement(&mut self, _ctx: &LeaveStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#leaveStatement}.
 * @param ctx the parse tree
 */
fn exit_leaveStatement(&mut self, _ctx: &LeaveStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#iterateStatement}.
 * @param ctx the parse tree
 */
fn enter_iterateStatement(&mut self, _ctx: &IterateStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#iterateStatement}.
 * @param ctx the parse tree
 */
fn exit_iterateStatement(&mut self, _ctx: &IterateStatementContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code searchedCaseStatement}
 * labeled alternative in {@link SqlBaseParser#caseStatement}.
 * @param ctx the parse tree
 */
fn enter_searchedCaseStatement(&mut self, _ctx: &SearchedCaseStatementContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code searchedCaseStatement}
 * labeled alternative in {@link SqlBaseParser#caseStatement}.
 * @param ctx the parse tree
 */
fn exit_searchedCaseStatement(&mut self, _ctx: &SearchedCaseStatementContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code simpleCaseStatement}
 * labeled alternative in {@link SqlBaseParser#caseStatement}.
 * @param ctx the parse tree
 */
fn enter_simpleCaseStatement(&mut self, _ctx: &SimpleCaseStatementContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code simpleCaseStatement}
 * labeled alternative in {@link SqlBaseParser#caseStatement}.
 * @param ctx the parse tree
 */
fn exit_simpleCaseStatement(&mut self, _ctx: &SimpleCaseStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#loopStatement}.
 * @param ctx the parse tree
 */
fn enter_loopStatement(&mut self, _ctx: &LoopStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#loopStatement}.
 * @param ctx the parse tree
 */
fn exit_loopStatement(&mut self, _ctx: &LoopStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#forStatement}.
 * @param ctx the parse tree
 */
fn enter_forStatement(&mut self, _ctx: &ForStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#forStatement}.
 * @param ctx the parse tree
 */
fn exit_forStatement(&mut self, _ctx: &ForStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#singleStatement}.
 * @param ctx the parse tree
 */
fn enter_singleStatement(&mut self, _ctx: &SingleStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#singleStatement}.
 * @param ctx the parse tree
 */
fn exit_singleStatement(&mut self, _ctx: &SingleStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#beginLabel}.
 * @param ctx the parse tree
 */
fn enter_beginLabel(&mut self, _ctx: &BeginLabelContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#beginLabel}.
 * @param ctx the parse tree
 */
fn exit_beginLabel(&mut self, _ctx: &BeginLabelContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#endLabel}.
 * @param ctx the parse tree
 */
fn enter_endLabel(&mut self, _ctx: &EndLabelContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#endLabel}.
 * @param ctx the parse tree
 */
fn exit_endLabel(&mut self, _ctx: &EndLabelContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#singleExpression}.
 * @param ctx the parse tree
 */
fn enter_singleExpression(&mut self, _ctx: &SingleExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#singleExpression}.
 * @param ctx the parse tree
 */
fn exit_singleExpression(&mut self, _ctx: &SingleExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#singleTableIdentifier}.
 * @param ctx the parse tree
 */
fn enter_singleTableIdentifier(&mut self, _ctx: &SingleTableIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#singleTableIdentifier}.
 * @param ctx the parse tree
 */
fn exit_singleTableIdentifier(&mut self, _ctx: &SingleTableIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#singleMultipartIdentifier}.
 * @param ctx the parse tree
 */
fn enter_singleMultipartIdentifier(&mut self, _ctx: &SingleMultipartIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#singleMultipartIdentifier}.
 * @param ctx the parse tree
 */
fn exit_singleMultipartIdentifier(&mut self, _ctx: &SingleMultipartIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#singleFunctionIdentifier}.
 * @param ctx the parse tree
 */
fn enter_singleFunctionIdentifier(&mut self, _ctx: &SingleFunctionIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#singleFunctionIdentifier}.
 * @param ctx the parse tree
 */
fn exit_singleFunctionIdentifier(&mut self, _ctx: &SingleFunctionIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#singleDataType}.
 * @param ctx the parse tree
 */
fn enter_singleDataType(&mut self, _ctx: &SingleDataTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#singleDataType}.
 * @param ctx the parse tree
 */
fn exit_singleDataType(&mut self, _ctx: &SingleDataTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#singleTableSchema}.
 * @param ctx the parse tree
 */
fn enter_singleTableSchema(&mut self, _ctx: &SingleTableSchemaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#singleTableSchema}.
 * @param ctx the parse tree
 */
fn exit_singleTableSchema(&mut self, _ctx: &SingleTableSchemaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#singleRoutineParamList}.
 * @param ctx the parse tree
 */
fn enter_singleRoutineParamList(&mut self, _ctx: &SingleRoutineParamListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#singleRoutineParamList}.
 * @param ctx the parse tree
 */
fn exit_singleRoutineParamList(&mut self, _ctx: &SingleRoutineParamListContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code statementDefault}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_statementDefault(&mut self, _ctx: &StatementDefaultContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code statementDefault}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_statementDefault(&mut self, _ctx: &StatementDefaultContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code visitExecuteImmediate}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_visitExecuteImmediate(&mut self, _ctx: &VisitExecuteImmediateContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code visitExecuteImmediate}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_visitExecuteImmediate(&mut self, _ctx: &VisitExecuteImmediateContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code dmlStatement}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_dmlStatement(&mut self, _ctx: &DmlStatementContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code dmlStatement}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_dmlStatement(&mut self, _ctx: &DmlStatementContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code use}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_use(&mut self, _ctx: &UseContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code use}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_use(&mut self, _ctx: &UseContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code useNamespace}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_useNamespace(&mut self, _ctx: &UseNamespaceContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code useNamespace}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_useNamespace(&mut self, _ctx: &UseNamespaceContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code setCatalog}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_setCatalog(&mut self, _ctx: &SetCatalogContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code setCatalog}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_setCatalog(&mut self, _ctx: &SetCatalogContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code createNamespace}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_createNamespace(&mut self, _ctx: &CreateNamespaceContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code createNamespace}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_createNamespace(&mut self, _ctx: &CreateNamespaceContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code setNamespaceProperties}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_setNamespaceProperties(&mut self, _ctx: &SetNamespacePropertiesContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code setNamespaceProperties}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_setNamespaceProperties(&mut self, _ctx: &SetNamespacePropertiesContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code unsetNamespaceProperties}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_unsetNamespaceProperties(&mut self, _ctx: &UnsetNamespacePropertiesContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code unsetNamespaceProperties}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_unsetNamespaceProperties(&mut self, _ctx: &UnsetNamespacePropertiesContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code setNamespaceCollation}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_setNamespaceCollation(&mut self, _ctx: &SetNamespaceCollationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code setNamespaceCollation}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_setNamespaceCollation(&mut self, _ctx: &SetNamespaceCollationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code setNamespaceLocation}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_setNamespaceLocation(&mut self, _ctx: &SetNamespaceLocationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code setNamespaceLocation}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_setNamespaceLocation(&mut self, _ctx: &SetNamespaceLocationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code dropNamespace}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_dropNamespace(&mut self, _ctx: &DropNamespaceContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code dropNamespace}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_dropNamespace(&mut self, _ctx: &DropNamespaceContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code showNamespaces}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_showNamespaces(&mut self, _ctx: &ShowNamespacesContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code showNamespaces}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_showNamespaces(&mut self, _ctx: &ShowNamespacesContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code createTable}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_createTable(&mut self, _ctx: &CreateTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code createTable}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_createTable(&mut self, _ctx: &CreateTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code createTableLike}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_createTableLike(&mut self, _ctx: &CreateTableLikeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code createTableLike}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_createTableLike(&mut self, _ctx: &CreateTableLikeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code replaceTable}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_replaceTable(&mut self, _ctx: &ReplaceTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code replaceTable}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_replaceTable(&mut self, _ctx: &ReplaceTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code analyze}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_analyze(&mut self, _ctx: &AnalyzeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code analyze}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_analyze(&mut self, _ctx: &AnalyzeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code analyzeTables}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_analyzeTables(&mut self, _ctx: &AnalyzeTablesContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code analyzeTables}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_analyzeTables(&mut self, _ctx: &AnalyzeTablesContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code addTableColumns}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_addTableColumns(&mut self, _ctx: &AddTableColumnsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code addTableColumns}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_addTableColumns(&mut self, _ctx: &AddTableColumnsContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code renameTableColumn}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_renameTableColumn(&mut self, _ctx: &RenameTableColumnContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code renameTableColumn}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_renameTableColumn(&mut self, _ctx: &RenameTableColumnContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code dropTableColumns}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_dropTableColumns(&mut self, _ctx: &DropTableColumnsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code dropTableColumns}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_dropTableColumns(&mut self, _ctx: &DropTableColumnsContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code renameTable}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_renameTable(&mut self, _ctx: &RenameTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code renameTable}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_renameTable(&mut self, _ctx: &RenameTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code setTableProperties}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_setTableProperties(&mut self, _ctx: &SetTablePropertiesContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code setTableProperties}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_setTableProperties(&mut self, _ctx: &SetTablePropertiesContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code unsetTableProperties}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_unsetTableProperties(&mut self, _ctx: &UnsetTablePropertiesContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code unsetTableProperties}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_unsetTableProperties(&mut self, _ctx: &UnsetTablePropertiesContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code alterTableAlterColumn}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_alterTableAlterColumn(&mut self, _ctx: &AlterTableAlterColumnContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code alterTableAlterColumn}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_alterTableAlterColumn(&mut self, _ctx: &AlterTableAlterColumnContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code hiveChangeColumn}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_hiveChangeColumn(&mut self, _ctx: &HiveChangeColumnContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code hiveChangeColumn}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_hiveChangeColumn(&mut self, _ctx: &HiveChangeColumnContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code hiveReplaceColumns}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_hiveReplaceColumns(&mut self, _ctx: &HiveReplaceColumnsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code hiveReplaceColumns}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_hiveReplaceColumns(&mut self, _ctx: &HiveReplaceColumnsContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code setTableSerDe}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_setTableSerDe(&mut self, _ctx: &SetTableSerDeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code setTableSerDe}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_setTableSerDe(&mut self, _ctx: &SetTableSerDeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code addTablePartition}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_addTablePartition(&mut self, _ctx: &AddTablePartitionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code addTablePartition}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_addTablePartition(&mut self, _ctx: &AddTablePartitionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code renameTablePartition}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_renameTablePartition(&mut self, _ctx: &RenameTablePartitionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code renameTablePartition}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_renameTablePartition(&mut self, _ctx: &RenameTablePartitionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code dropTablePartitions}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_dropTablePartitions(&mut self, _ctx: &DropTablePartitionsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code dropTablePartitions}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_dropTablePartitions(&mut self, _ctx: &DropTablePartitionsContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code setTableLocation}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_setTableLocation(&mut self, _ctx: &SetTableLocationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code setTableLocation}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_setTableLocation(&mut self, _ctx: &SetTableLocationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code recoverPartitions}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_recoverPartitions(&mut self, _ctx: &RecoverPartitionsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code recoverPartitions}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_recoverPartitions(&mut self, _ctx: &RecoverPartitionsContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code alterClusterBy}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_alterClusterBy(&mut self, _ctx: &AlterClusterByContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code alterClusterBy}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_alterClusterBy(&mut self, _ctx: &AlterClusterByContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code alterTableCollation}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_alterTableCollation(&mut self, _ctx: &AlterTableCollationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code alterTableCollation}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_alterTableCollation(&mut self, _ctx: &AlterTableCollationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code addTableConstraint}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_addTableConstraint(&mut self, _ctx: &AddTableConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code addTableConstraint}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_addTableConstraint(&mut self, _ctx: &AddTableConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code dropTableConstraint}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_dropTableConstraint(&mut self, _ctx: &DropTableConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code dropTableConstraint}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_dropTableConstraint(&mut self, _ctx: &DropTableConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code dropTable}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_dropTable(&mut self, _ctx: &DropTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code dropTable}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_dropTable(&mut self, _ctx: &DropTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code dropView}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_dropView(&mut self, _ctx: &DropViewContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code dropView}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_dropView(&mut self, _ctx: &DropViewContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code createView}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_createView(&mut self, _ctx: &CreateViewContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code createView}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_createView(&mut self, _ctx: &CreateViewContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code createMetricView}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_createMetricView(&mut self, _ctx: &CreateMetricViewContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code createMetricView}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_createMetricView(&mut self, _ctx: &CreateMetricViewContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code createTempViewUsing}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_createTempViewUsing(&mut self, _ctx: &CreateTempViewUsingContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code createTempViewUsing}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_createTempViewUsing(&mut self, _ctx: &CreateTempViewUsingContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code alterViewQuery}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_alterViewQuery(&mut self, _ctx: &AlterViewQueryContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code alterViewQuery}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_alterViewQuery(&mut self, _ctx: &AlterViewQueryContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code alterViewSchemaBinding}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_alterViewSchemaBinding(&mut self, _ctx: &AlterViewSchemaBindingContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code alterViewSchemaBinding}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_alterViewSchemaBinding(&mut self, _ctx: &AlterViewSchemaBindingContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code createFunction}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_createFunction(&mut self, _ctx: &CreateFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code createFunction}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_createFunction(&mut self, _ctx: &CreateFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code createUserDefinedFunction}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_createUserDefinedFunction(&mut self, _ctx: &CreateUserDefinedFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code createUserDefinedFunction}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_createUserDefinedFunction(&mut self, _ctx: &CreateUserDefinedFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code dropFunction}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_dropFunction(&mut self, _ctx: &DropFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code dropFunction}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_dropFunction(&mut self, _ctx: &DropFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code createVariable}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_createVariable(&mut self, _ctx: &CreateVariableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code createVariable}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_createVariable(&mut self, _ctx: &CreateVariableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code dropVariable}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_dropVariable(&mut self, _ctx: &DropVariableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code dropVariable}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_dropVariable(&mut self, _ctx: &DropVariableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code explain}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_explain(&mut self, _ctx: &ExplainContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code explain}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_explain(&mut self, _ctx: &ExplainContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code showTables}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_showTables(&mut self, _ctx: &ShowTablesContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code showTables}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_showTables(&mut self, _ctx: &ShowTablesContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code showTableExtended}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_showTableExtended(&mut self, _ctx: &ShowTableExtendedContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code showTableExtended}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_showTableExtended(&mut self, _ctx: &ShowTableExtendedContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code showTblProperties}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_showTblProperties(&mut self, _ctx: &ShowTblPropertiesContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code showTblProperties}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_showTblProperties(&mut self, _ctx: &ShowTblPropertiesContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code showColumns}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_showColumns(&mut self, _ctx: &ShowColumnsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code showColumns}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_showColumns(&mut self, _ctx: &ShowColumnsContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code showViews}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_showViews(&mut self, _ctx: &ShowViewsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code showViews}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_showViews(&mut self, _ctx: &ShowViewsContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code showPartitions}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_showPartitions(&mut self, _ctx: &ShowPartitionsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code showPartitions}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_showPartitions(&mut self, _ctx: &ShowPartitionsContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code showFunctions}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_showFunctions(&mut self, _ctx: &ShowFunctionsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code showFunctions}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_showFunctions(&mut self, _ctx: &ShowFunctionsContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code showProcedures}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_showProcedures(&mut self, _ctx: &ShowProceduresContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code showProcedures}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_showProcedures(&mut self, _ctx: &ShowProceduresContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code showCreateTable}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_showCreateTable(&mut self, _ctx: &ShowCreateTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code showCreateTable}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_showCreateTable(&mut self, _ctx: &ShowCreateTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code showCurrentNamespace}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_showCurrentNamespace(&mut self, _ctx: &ShowCurrentNamespaceContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code showCurrentNamespace}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_showCurrentNamespace(&mut self, _ctx: &ShowCurrentNamespaceContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code showCatalogs}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_showCatalogs(&mut self, _ctx: &ShowCatalogsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code showCatalogs}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_showCatalogs(&mut self, _ctx: &ShowCatalogsContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code describeFunction}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_describeFunction(&mut self, _ctx: &DescribeFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code describeFunction}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_describeFunction(&mut self, _ctx: &DescribeFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code describeProcedure}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_describeProcedure(&mut self, _ctx: &DescribeProcedureContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code describeProcedure}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_describeProcedure(&mut self, _ctx: &DescribeProcedureContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code describeNamespace}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_describeNamespace(&mut self, _ctx: &DescribeNamespaceContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code describeNamespace}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_describeNamespace(&mut self, _ctx: &DescribeNamespaceContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code describeRelation}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_describeRelation(&mut self, _ctx: &DescribeRelationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code describeRelation}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_describeRelation(&mut self, _ctx: &DescribeRelationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code describeQuery}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_describeQuery(&mut self, _ctx: &DescribeQueryContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code describeQuery}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_describeQuery(&mut self, _ctx: &DescribeQueryContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code commentNamespace}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_commentNamespace(&mut self, _ctx: &CommentNamespaceContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code commentNamespace}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_commentNamespace(&mut self, _ctx: &CommentNamespaceContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code commentTable}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_commentTable(&mut self, _ctx: &CommentTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code commentTable}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_commentTable(&mut self, _ctx: &CommentTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code refreshTable}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_refreshTable(&mut self, _ctx: &RefreshTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code refreshTable}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_refreshTable(&mut self, _ctx: &RefreshTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code refreshFunction}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_refreshFunction(&mut self, _ctx: &RefreshFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code refreshFunction}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_refreshFunction(&mut self, _ctx: &RefreshFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code refreshResource}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_refreshResource(&mut self, _ctx: &RefreshResourceContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code refreshResource}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_refreshResource(&mut self, _ctx: &RefreshResourceContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code cacheTable}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_cacheTable(&mut self, _ctx: &CacheTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code cacheTable}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_cacheTable(&mut self, _ctx: &CacheTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code uncacheTable}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_uncacheTable(&mut self, _ctx: &UncacheTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code uncacheTable}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_uncacheTable(&mut self, _ctx: &UncacheTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code clearCache}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_clearCache(&mut self, _ctx: &ClearCacheContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code clearCache}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_clearCache(&mut self, _ctx: &ClearCacheContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code loadData}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_loadData(&mut self, _ctx: &LoadDataContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code loadData}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_loadData(&mut self, _ctx: &LoadDataContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code truncateTable}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_truncateTable(&mut self, _ctx: &TruncateTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code truncateTable}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_truncateTable(&mut self, _ctx: &TruncateTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code repairTable}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_repairTable(&mut self, _ctx: &RepairTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code repairTable}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_repairTable(&mut self, _ctx: &RepairTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code manageResource}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_manageResource(&mut self, _ctx: &ManageResourceContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code manageResource}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_manageResource(&mut self, _ctx: &ManageResourceContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code createIndex}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_createIndex(&mut self, _ctx: &CreateIndexContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code createIndex}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_createIndex(&mut self, _ctx: &CreateIndexContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code dropIndex}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_dropIndex(&mut self, _ctx: &DropIndexContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code dropIndex}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_dropIndex(&mut self, _ctx: &DropIndexContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code call}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_call(&mut self, _ctx: &CallContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code call}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_call(&mut self, _ctx: &CallContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code failNativeCommand}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_failNativeCommand(&mut self, _ctx: &FailNativeCommandContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code failNativeCommand}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_failNativeCommand(&mut self, _ctx: &FailNativeCommandContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code createPipelineDataset}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_createPipelineDataset(&mut self, _ctx: &CreatePipelineDatasetContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code createPipelineDataset}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_createPipelineDataset(&mut self, _ctx: &CreatePipelineDatasetContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code createPipelineInsertIntoFlow}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn enter_createPipelineInsertIntoFlow(&mut self, _ctx: &CreatePipelineInsertIntoFlowContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code createPipelineInsertIntoFlow}
 * labeled alternative in {@link SqlBaseParser#statement}.
 * @param ctx the parse tree
 */
fn exit_createPipelineInsertIntoFlow(&mut self, _ctx: &CreatePipelineInsertIntoFlowContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#materializedView}.
 * @param ctx the parse tree
 */
fn enter_materializedView(&mut self, _ctx: &MaterializedViewContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#materializedView}.
 * @param ctx the parse tree
 */
fn exit_materializedView(&mut self, _ctx: &MaterializedViewContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#streamingTable}.
 * @param ctx the parse tree
 */
fn enter_streamingTable(&mut self, _ctx: &StreamingTableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#streamingTable}.
 * @param ctx the parse tree
 */
fn exit_streamingTable(&mut self, _ctx: &StreamingTableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#createPipelineDatasetHeader}.
 * @param ctx the parse tree
 */
fn enter_createPipelineDatasetHeader(&mut self, _ctx: &CreatePipelineDatasetHeaderContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#createPipelineDatasetHeader}.
 * @param ctx the parse tree
 */
fn exit_createPipelineDatasetHeader(&mut self, _ctx: &CreatePipelineDatasetHeaderContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code streamTableName}
 * labeled alternative in {@link SqlBaseParser#streamRelationPrimary}.
 * @param ctx the parse tree
 */
fn enter_streamTableName(&mut self, _ctx: &StreamTableNameContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code streamTableName}
 * labeled alternative in {@link SqlBaseParser#streamRelationPrimary}.
 * @param ctx the parse tree
 */
fn exit_streamTableName(&mut self, _ctx: &StreamTableNameContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code failSetRole}
 * labeled alternative in {@link SqlBaseParser#setResetStatement}.
 * @param ctx the parse tree
 */
fn enter_failSetRole(&mut self, _ctx: &FailSetRoleContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code failSetRole}
 * labeled alternative in {@link SqlBaseParser#setResetStatement}.
 * @param ctx the parse tree
 */
fn exit_failSetRole(&mut self, _ctx: &FailSetRoleContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code setTimeZone}
 * labeled alternative in {@link SqlBaseParser#setResetStatement}.
 * @param ctx the parse tree
 */
fn enter_setTimeZone(&mut self, _ctx: &SetTimeZoneContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code setTimeZone}
 * labeled alternative in {@link SqlBaseParser#setResetStatement}.
 * @param ctx the parse tree
 */
fn exit_setTimeZone(&mut self, _ctx: &SetTimeZoneContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code setVariable}
 * labeled alternative in {@link SqlBaseParser#setResetStatement}.
 * @param ctx the parse tree
 */
fn enter_setVariable(&mut self, _ctx: &SetVariableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code setVariable}
 * labeled alternative in {@link SqlBaseParser#setResetStatement}.
 * @param ctx the parse tree
 */
fn exit_setVariable(&mut self, _ctx: &SetVariableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code setQuotedConfiguration}
 * labeled alternative in {@link SqlBaseParser#setResetStatement}.
 * @param ctx the parse tree
 */
fn enter_setQuotedConfiguration(&mut self, _ctx: &SetQuotedConfigurationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code setQuotedConfiguration}
 * labeled alternative in {@link SqlBaseParser#setResetStatement}.
 * @param ctx the parse tree
 */
fn exit_setQuotedConfiguration(&mut self, _ctx: &SetQuotedConfigurationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code setConfiguration}
 * labeled alternative in {@link SqlBaseParser#setResetStatement}.
 * @param ctx the parse tree
 */
fn enter_setConfiguration(&mut self, _ctx: &SetConfigurationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code setConfiguration}
 * labeled alternative in {@link SqlBaseParser#setResetStatement}.
 * @param ctx the parse tree
 */
fn exit_setConfiguration(&mut self, _ctx: &SetConfigurationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code resetQuotedConfiguration}
 * labeled alternative in {@link SqlBaseParser#setResetStatement}.
 * @param ctx the parse tree
 */
fn enter_resetQuotedConfiguration(&mut self, _ctx: &ResetQuotedConfigurationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code resetQuotedConfiguration}
 * labeled alternative in {@link SqlBaseParser#setResetStatement}.
 * @param ctx the parse tree
 */
fn exit_resetQuotedConfiguration(&mut self, _ctx: &ResetQuotedConfigurationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code resetConfiguration}
 * labeled alternative in {@link SqlBaseParser#setResetStatement}.
 * @param ctx the parse tree
 */
fn enter_resetConfiguration(&mut self, _ctx: &ResetConfigurationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code resetConfiguration}
 * labeled alternative in {@link SqlBaseParser#setResetStatement}.
 * @param ctx the parse tree
 */
fn exit_resetConfiguration(&mut self, _ctx: &ResetConfigurationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#executeImmediate}.
 * @param ctx the parse tree
 */
fn enter_executeImmediate(&mut self, _ctx: &ExecuteImmediateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#executeImmediate}.
 * @param ctx the parse tree
 */
fn exit_executeImmediate(&mut self, _ctx: &ExecuteImmediateContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#executeImmediateUsing}.
 * @param ctx the parse tree
 */
fn enter_executeImmediateUsing(&mut self, _ctx: &ExecuteImmediateUsingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#executeImmediateUsing}.
 * @param ctx the parse tree
 */
fn exit_executeImmediateUsing(&mut self, _ctx: &ExecuteImmediateUsingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#timezone}.
 * @param ctx the parse tree
 */
fn enter_timezone(&mut self, _ctx: &TimezoneContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#timezone}.
 * @param ctx the parse tree
 */
fn exit_timezone(&mut self, _ctx: &TimezoneContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#configKey}.
 * @param ctx the parse tree
 */
fn enter_configKey(&mut self, _ctx: &ConfigKeyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#configKey}.
 * @param ctx the parse tree
 */
fn exit_configKey(&mut self, _ctx: &ConfigKeyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#configValue}.
 * @param ctx the parse tree
 */
fn enter_configValue(&mut self, _ctx: &ConfigValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#configValue}.
 * @param ctx the parse tree
 */
fn exit_configValue(&mut self, _ctx: &ConfigValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#unsupportedHiveNativeCommands}.
 * @param ctx the parse tree
 */
fn enter_unsupportedHiveNativeCommands(&mut self, _ctx: &UnsupportedHiveNativeCommandsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#unsupportedHiveNativeCommands}.
 * @param ctx the parse tree
 */
fn exit_unsupportedHiveNativeCommands(&mut self, _ctx: &UnsupportedHiveNativeCommandsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#createTableHeader}.
 * @param ctx the parse tree
 */
fn enter_createTableHeader(&mut self, _ctx: &CreateTableHeaderContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#createTableHeader}.
 * @param ctx the parse tree
 */
fn exit_createTableHeader(&mut self, _ctx: &CreateTableHeaderContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#replaceTableHeader}.
 * @param ctx the parse tree
 */
fn enter_replaceTableHeader(&mut self, _ctx: &ReplaceTableHeaderContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#replaceTableHeader}.
 * @param ctx the parse tree
 */
fn exit_replaceTableHeader(&mut self, _ctx: &ReplaceTableHeaderContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#clusterBySpec}.
 * @param ctx the parse tree
 */
fn enter_clusterBySpec(&mut self, _ctx: &ClusterBySpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#clusterBySpec}.
 * @param ctx the parse tree
 */
fn exit_clusterBySpec(&mut self, _ctx: &ClusterBySpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#bucketSpec}.
 * @param ctx the parse tree
 */
fn enter_bucketSpec(&mut self, _ctx: &BucketSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#bucketSpec}.
 * @param ctx the parse tree
 */
fn exit_bucketSpec(&mut self, _ctx: &BucketSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#skewSpec}.
 * @param ctx the parse tree
 */
fn enter_skewSpec(&mut self, _ctx: &SkewSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#skewSpec}.
 * @param ctx the parse tree
 */
fn exit_skewSpec(&mut self, _ctx: &SkewSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#locationSpec}.
 * @param ctx the parse tree
 */
fn enter_locationSpec(&mut self, _ctx: &LocationSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#locationSpec}.
 * @param ctx the parse tree
 */
fn exit_locationSpec(&mut self, _ctx: &LocationSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#schemaBinding}.
 * @param ctx the parse tree
 */
fn enter_schemaBinding(&mut self, _ctx: &SchemaBindingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#schemaBinding}.
 * @param ctx the parse tree
 */
fn exit_schemaBinding(&mut self, _ctx: &SchemaBindingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#commentSpec}.
 * @param ctx the parse tree
 */
fn enter_commentSpec(&mut self, _ctx: &CommentSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#commentSpec}.
 * @param ctx the parse tree
 */
fn exit_commentSpec(&mut self, _ctx: &CommentSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#singleQuery}.
 * @param ctx the parse tree
 */
fn enter_singleQuery(&mut self, _ctx: &SingleQueryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#singleQuery}.
 * @param ctx the parse tree
 */
fn exit_singleQuery(&mut self, _ctx: &SingleQueryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#query}.
 * @param ctx the parse tree
 */
fn enter_query(&mut self, _ctx: &QueryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#query}.
 * @param ctx the parse tree
 */
fn exit_query(&mut self, _ctx: &QueryContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code insertOverwriteTable}
 * labeled alternative in {@link SqlBaseParser#insertInto}.
 * @param ctx the parse tree
 */
fn enter_insertOverwriteTable(&mut self, _ctx: &InsertOverwriteTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code insertOverwriteTable}
 * labeled alternative in {@link SqlBaseParser#insertInto}.
 * @param ctx the parse tree
 */
fn exit_insertOverwriteTable(&mut self, _ctx: &InsertOverwriteTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code insertIntoTable}
 * labeled alternative in {@link SqlBaseParser#insertInto}.
 * @param ctx the parse tree
 */
fn enter_insertIntoTable(&mut self, _ctx: &InsertIntoTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code insertIntoTable}
 * labeled alternative in {@link SqlBaseParser#insertInto}.
 * @param ctx the parse tree
 */
fn exit_insertIntoTable(&mut self, _ctx: &InsertIntoTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code insertIntoReplaceWhere}
 * labeled alternative in {@link SqlBaseParser#insertInto}.
 * @param ctx the parse tree
 */
fn enter_insertIntoReplaceWhere(&mut self, _ctx: &InsertIntoReplaceWhereContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code insertIntoReplaceWhere}
 * labeled alternative in {@link SqlBaseParser#insertInto}.
 * @param ctx the parse tree
 */
fn exit_insertIntoReplaceWhere(&mut self, _ctx: &InsertIntoReplaceWhereContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code insertOverwriteHiveDir}
 * labeled alternative in {@link SqlBaseParser#insertInto}.
 * @param ctx the parse tree
 */
fn enter_insertOverwriteHiveDir(&mut self, _ctx: &InsertOverwriteHiveDirContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code insertOverwriteHiveDir}
 * labeled alternative in {@link SqlBaseParser#insertInto}.
 * @param ctx the parse tree
 */
fn exit_insertOverwriteHiveDir(&mut self, _ctx: &InsertOverwriteHiveDirContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code insertOverwriteDir}
 * labeled alternative in {@link SqlBaseParser#insertInto}.
 * @param ctx the parse tree
 */
fn enter_insertOverwriteDir(&mut self, _ctx: &InsertOverwriteDirContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code insertOverwriteDir}
 * labeled alternative in {@link SqlBaseParser#insertInto}.
 * @param ctx the parse tree
 */
fn exit_insertOverwriteDir(&mut self, _ctx: &InsertOverwriteDirContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#partitionSpecLocation}.
 * @param ctx the parse tree
 */
fn enter_partitionSpecLocation(&mut self, _ctx: &PartitionSpecLocationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#partitionSpecLocation}.
 * @param ctx the parse tree
 */
fn exit_partitionSpecLocation(&mut self, _ctx: &PartitionSpecLocationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#partitionSpec}.
 * @param ctx the parse tree
 */
fn enter_partitionSpec(&mut self, _ctx: &PartitionSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#partitionSpec}.
 * @param ctx the parse tree
 */
fn exit_partitionSpec(&mut self, _ctx: &PartitionSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#partitionVal}.
 * @param ctx the parse tree
 */
fn enter_partitionVal(&mut self, _ctx: &PartitionValContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#partitionVal}.
 * @param ctx the parse tree
 */
fn exit_partitionVal(&mut self, _ctx: &PartitionValContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#createPipelineFlowHeader}.
 * @param ctx the parse tree
 */
fn enter_createPipelineFlowHeader(&mut self, _ctx: &CreatePipelineFlowHeaderContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#createPipelineFlowHeader}.
 * @param ctx the parse tree
 */
fn exit_createPipelineFlowHeader(&mut self, _ctx: &CreatePipelineFlowHeaderContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#namespace}.
 * @param ctx the parse tree
 */
fn enter_namespace(&mut self, _ctx: &NamespaceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#namespace}.
 * @param ctx the parse tree
 */
fn exit_namespace(&mut self, _ctx: &NamespaceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#namespaces}.
 * @param ctx the parse tree
 */
fn enter_namespaces(&mut self, _ctx: &NamespacesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#namespaces}.
 * @param ctx the parse tree
 */
fn exit_namespaces(&mut self, _ctx: &NamespacesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#variable}.
 * @param ctx the parse tree
 */
fn enter_variable(&mut self, _ctx: &VariableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#variable}.
 * @param ctx the parse tree
 */
fn exit_variable(&mut self, _ctx: &VariableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#describeFuncName}.
 * @param ctx the parse tree
 */
fn enter_describeFuncName(&mut self, _ctx: &DescribeFuncNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#describeFuncName}.
 * @param ctx the parse tree
 */
fn exit_describeFuncName(&mut self, _ctx: &DescribeFuncNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#describeColName}.
 * @param ctx the parse tree
 */
fn enter_describeColName(&mut self, _ctx: &DescribeColNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#describeColName}.
 * @param ctx the parse tree
 */
fn exit_describeColName(&mut self, _ctx: &DescribeColNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#ctes}.
 * @param ctx the parse tree
 */
fn enter_ctes(&mut self, _ctx: &CtesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#ctes}.
 * @param ctx the parse tree
 */
fn exit_ctes(&mut self, _ctx: &CtesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#namedQuery}.
 * @param ctx the parse tree
 */
fn enter_namedQuery(&mut self, _ctx: &NamedQueryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#namedQuery}.
 * @param ctx the parse tree
 */
fn exit_namedQuery(&mut self, _ctx: &NamedQueryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#tableProvider}.
 * @param ctx the parse tree
 */
fn enter_tableProvider(&mut self, _ctx: &TableProviderContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#tableProvider}.
 * @param ctx the parse tree
 */
fn exit_tableProvider(&mut self, _ctx: &TableProviderContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#createTableClauses}.
 * @param ctx the parse tree
 */
fn enter_createTableClauses(&mut self, _ctx: &CreateTableClausesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#createTableClauses}.
 * @param ctx the parse tree
 */
fn exit_createTableClauses(&mut self, _ctx: &CreateTableClausesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#propertyList}.
 * @param ctx the parse tree
 */
fn enter_propertyList(&mut self, _ctx: &PropertyListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#propertyList}.
 * @param ctx the parse tree
 */
fn exit_propertyList(&mut self, _ctx: &PropertyListContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code propertyWithKeyAndEquals}
 * labeled alternative in {@link SqlBaseParser#property}.
 * @param ctx the parse tree
 */
fn enter_propertyWithKeyAndEquals(&mut self, _ctx: &PropertyWithKeyAndEqualsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code propertyWithKeyAndEquals}
 * labeled alternative in {@link SqlBaseParser#property}.
 * @param ctx the parse tree
 */
fn exit_propertyWithKeyAndEquals(&mut self, _ctx: &PropertyWithKeyAndEqualsContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code propertyWithKeyNoEquals}
 * labeled alternative in {@link SqlBaseParser#property}.
 * @param ctx the parse tree
 */
fn enter_propertyWithKeyNoEquals(&mut self, _ctx: &PropertyWithKeyNoEqualsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code propertyWithKeyNoEquals}
 * labeled alternative in {@link SqlBaseParser#property}.
 * @param ctx the parse tree
 */
fn exit_propertyWithKeyNoEquals(&mut self, _ctx: &PropertyWithKeyNoEqualsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#propertyKey}.
 * @param ctx the parse tree
 */
fn enter_propertyKey(&mut self, _ctx: &PropertyKeyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#propertyKey}.
 * @param ctx the parse tree
 */
fn exit_propertyKey(&mut self, _ctx: &PropertyKeyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#propertyKeyOrStringLit}.
 * @param ctx the parse tree
 */
fn enter_propertyKeyOrStringLit(&mut self, _ctx: &PropertyKeyOrStringLitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#propertyKeyOrStringLit}.
 * @param ctx the parse tree
 */
fn exit_propertyKeyOrStringLit(&mut self, _ctx: &PropertyKeyOrStringLitContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#propertyKeyOrStringLitNoCoalesce}.
 * @param ctx the parse tree
 */
fn enter_propertyKeyOrStringLitNoCoalesce(&mut self, _ctx: &PropertyKeyOrStringLitNoCoalesceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#propertyKeyOrStringLitNoCoalesce}.
 * @param ctx the parse tree
 */
fn exit_propertyKeyOrStringLitNoCoalesce(&mut self, _ctx: &PropertyKeyOrStringLitNoCoalesceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#propertyValue}.
 * @param ctx the parse tree
 */
fn enter_propertyValue(&mut self, _ctx: &PropertyValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#propertyValue}.
 * @param ctx the parse tree
 */
fn exit_propertyValue(&mut self, _ctx: &PropertyValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#expressionPropertyList}.
 * @param ctx the parse tree
 */
fn enter_expressionPropertyList(&mut self, _ctx: &ExpressionPropertyListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#expressionPropertyList}.
 * @param ctx the parse tree
 */
fn exit_expressionPropertyList(&mut self, _ctx: &ExpressionPropertyListContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code expressionPropertyWithKeyAndEquals}
 * labeled alternative in {@link SqlBaseParser#expressionProperty}.
 * @param ctx the parse tree
 */
fn enter_expressionPropertyWithKeyAndEquals(&mut self, _ctx: &ExpressionPropertyWithKeyAndEqualsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code expressionPropertyWithKeyAndEquals}
 * labeled alternative in {@link SqlBaseParser#expressionProperty}.
 * @param ctx the parse tree
 */
fn exit_expressionPropertyWithKeyAndEquals(&mut self, _ctx: &ExpressionPropertyWithKeyAndEqualsContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code expressionPropertyWithKeyNoEquals}
 * labeled alternative in {@link SqlBaseParser#expressionProperty}.
 * @param ctx the parse tree
 */
fn enter_expressionPropertyWithKeyNoEquals(&mut self, _ctx: &ExpressionPropertyWithKeyNoEqualsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code expressionPropertyWithKeyNoEquals}
 * labeled alternative in {@link SqlBaseParser#expressionProperty}.
 * @param ctx the parse tree
 */
fn exit_expressionPropertyWithKeyNoEquals(&mut self, _ctx: &ExpressionPropertyWithKeyNoEqualsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#constantList}.
 * @param ctx the parse tree
 */
fn enter_constantList(&mut self, _ctx: &ConstantListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#constantList}.
 * @param ctx the parse tree
 */
fn exit_constantList(&mut self, _ctx: &ConstantListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#nestedConstantList}.
 * @param ctx the parse tree
 */
fn enter_nestedConstantList(&mut self, _ctx: &NestedConstantListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#nestedConstantList}.
 * @param ctx the parse tree
 */
fn exit_nestedConstantList(&mut self, _ctx: &NestedConstantListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#createFileFormat}.
 * @param ctx the parse tree
 */
fn enter_createFileFormat(&mut self, _ctx: &CreateFileFormatContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#createFileFormat}.
 * @param ctx the parse tree
 */
fn exit_createFileFormat(&mut self, _ctx: &CreateFileFormatContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code tableFileFormat}
 * labeled alternative in {@link SqlBaseParser#fileFormat}.
 * @param ctx the parse tree
 */
fn enter_tableFileFormat(&mut self, _ctx: &TableFileFormatContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code tableFileFormat}
 * labeled alternative in {@link SqlBaseParser#fileFormat}.
 * @param ctx the parse tree
 */
fn exit_tableFileFormat(&mut self, _ctx: &TableFileFormatContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code genericFileFormat}
 * labeled alternative in {@link SqlBaseParser#fileFormat}.
 * @param ctx the parse tree
 */
fn enter_genericFileFormat(&mut self, _ctx: &GenericFileFormatContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code genericFileFormat}
 * labeled alternative in {@link SqlBaseParser#fileFormat}.
 * @param ctx the parse tree
 */
fn exit_genericFileFormat(&mut self, _ctx: &GenericFileFormatContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#storageHandler}.
 * @param ctx the parse tree
 */
fn enter_storageHandler(&mut self, _ctx: &StorageHandlerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#storageHandler}.
 * @param ctx the parse tree
 */
fn exit_storageHandler(&mut self, _ctx: &StorageHandlerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#resource}.
 * @param ctx the parse tree
 */
fn enter_resource(&mut self, _ctx: &ResourceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#resource}.
 * @param ctx the parse tree
 */
fn exit_resource(&mut self, _ctx: &ResourceContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code singleInsertQuery}
 * labeled alternative in {@link SqlBaseParser#dmlStatementNoWith}.
 * @param ctx the parse tree
 */
fn enter_singleInsertQuery(&mut self, _ctx: &SingleInsertQueryContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code singleInsertQuery}
 * labeled alternative in {@link SqlBaseParser#dmlStatementNoWith}.
 * @param ctx the parse tree
 */
fn exit_singleInsertQuery(&mut self, _ctx: &SingleInsertQueryContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code multiInsertQuery}
 * labeled alternative in {@link SqlBaseParser#dmlStatementNoWith}.
 * @param ctx the parse tree
 */
fn enter_multiInsertQuery(&mut self, _ctx: &MultiInsertQueryContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code multiInsertQuery}
 * labeled alternative in {@link SqlBaseParser#dmlStatementNoWith}.
 * @param ctx the parse tree
 */
fn exit_multiInsertQuery(&mut self, _ctx: &MultiInsertQueryContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code deleteFromTable}
 * labeled alternative in {@link SqlBaseParser#dmlStatementNoWith}.
 * @param ctx the parse tree
 */
fn enter_deleteFromTable(&mut self, _ctx: &DeleteFromTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code deleteFromTable}
 * labeled alternative in {@link SqlBaseParser#dmlStatementNoWith}.
 * @param ctx the parse tree
 */
fn exit_deleteFromTable(&mut self, _ctx: &DeleteFromTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code updateTable}
 * labeled alternative in {@link SqlBaseParser#dmlStatementNoWith}.
 * @param ctx the parse tree
 */
fn enter_updateTable(&mut self, _ctx: &UpdateTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code updateTable}
 * labeled alternative in {@link SqlBaseParser#dmlStatementNoWith}.
 * @param ctx the parse tree
 */
fn exit_updateTable(&mut self, _ctx: &UpdateTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code mergeIntoTable}
 * labeled alternative in {@link SqlBaseParser#dmlStatementNoWith}.
 * @param ctx the parse tree
 */
fn enter_mergeIntoTable(&mut self, _ctx: &MergeIntoTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code mergeIntoTable}
 * labeled alternative in {@link SqlBaseParser#dmlStatementNoWith}.
 * @param ctx the parse tree
 */
fn exit_mergeIntoTable(&mut self, _ctx: &MergeIntoTableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#identifierReference}.
 * @param ctx the parse tree
 */
fn enter_identifierReference(&mut self, _ctx: &IdentifierReferenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#identifierReference}.
 * @param ctx the parse tree
 */
fn exit_identifierReference(&mut self, _ctx: &IdentifierReferenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#catalogIdentifierReference}.
 * @param ctx the parse tree
 */
fn enter_catalogIdentifierReference(&mut self, _ctx: &CatalogIdentifierReferenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#catalogIdentifierReference}.
 * @param ctx the parse tree
 */
fn exit_catalogIdentifierReference(&mut self, _ctx: &CatalogIdentifierReferenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#queryOrganization}.
 * @param ctx the parse tree
 */
fn enter_queryOrganization(&mut self, _ctx: &QueryOrganizationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#queryOrganization}.
 * @param ctx the parse tree
 */
fn exit_queryOrganization(&mut self, _ctx: &QueryOrganizationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#multiInsertQueryBody}.
 * @param ctx the parse tree
 */
fn enter_multiInsertQueryBody(&mut self, _ctx: &MultiInsertQueryBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#multiInsertQueryBody}.
 * @param ctx the parse tree
 */
fn exit_multiInsertQueryBody(&mut self, _ctx: &MultiInsertQueryBodyContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code operatorPipeStatement}
 * labeled alternative in {@link SqlBaseParser#queryTerm}.
 * @param ctx the parse tree
 */
fn enter_operatorPipeStatement(&mut self, _ctx: &OperatorPipeStatementContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code operatorPipeStatement}
 * labeled alternative in {@link SqlBaseParser#queryTerm}.
 * @param ctx the parse tree
 */
fn exit_operatorPipeStatement(&mut self, _ctx: &OperatorPipeStatementContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code queryTermDefault}
 * labeled alternative in {@link SqlBaseParser#queryTerm}.
 * @param ctx the parse tree
 */
fn enter_queryTermDefault(&mut self, _ctx: &QueryTermDefaultContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code queryTermDefault}
 * labeled alternative in {@link SqlBaseParser#queryTerm}.
 * @param ctx the parse tree
 */
fn exit_queryTermDefault(&mut self, _ctx: &QueryTermDefaultContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code setOperation}
 * labeled alternative in {@link SqlBaseParser#queryTerm}.
 * @param ctx the parse tree
 */
fn enter_setOperation(&mut self, _ctx: &SetOperationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code setOperation}
 * labeled alternative in {@link SqlBaseParser#queryTerm}.
 * @param ctx the parse tree
 */
fn exit_setOperation(&mut self, _ctx: &SetOperationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code queryPrimaryDefault}
 * labeled alternative in {@link SqlBaseParser#queryPrimary}.
 * @param ctx the parse tree
 */
fn enter_queryPrimaryDefault(&mut self, _ctx: &QueryPrimaryDefaultContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code queryPrimaryDefault}
 * labeled alternative in {@link SqlBaseParser#queryPrimary}.
 * @param ctx the parse tree
 */
fn exit_queryPrimaryDefault(&mut self, _ctx: &QueryPrimaryDefaultContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code fromStmt}
 * labeled alternative in {@link SqlBaseParser#queryPrimary}.
 * @param ctx the parse tree
 */
fn enter_fromStmt(&mut self, _ctx: &FromStmtContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code fromStmt}
 * labeled alternative in {@link SqlBaseParser#queryPrimary}.
 * @param ctx the parse tree
 */
fn exit_fromStmt(&mut self, _ctx: &FromStmtContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code table}
 * labeled alternative in {@link SqlBaseParser#queryPrimary}.
 * @param ctx the parse tree
 */
fn enter_table(&mut self, _ctx: &TableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code table}
 * labeled alternative in {@link SqlBaseParser#queryPrimary}.
 * @param ctx the parse tree
 */
fn exit_table(&mut self, _ctx: &TableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code inlineTableDefault1}
 * labeled alternative in {@link SqlBaseParser#queryPrimary}.
 * @param ctx the parse tree
 */
fn enter_inlineTableDefault1(&mut self, _ctx: &InlineTableDefault1Context<'input>) { }
/**
 * Exit a parse tree produced by the {@code inlineTableDefault1}
 * labeled alternative in {@link SqlBaseParser#queryPrimary}.
 * @param ctx the parse tree
 */
fn exit_inlineTableDefault1(&mut self, _ctx: &InlineTableDefault1Context<'input>) { }
/**
 * Enter a parse tree produced by the {@code subquery}
 * labeled alternative in {@link SqlBaseParser#queryPrimary}.
 * @param ctx the parse tree
 */
fn enter_subquery(&mut self, _ctx: &SubqueryContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code subquery}
 * labeled alternative in {@link SqlBaseParser#queryPrimary}.
 * @param ctx the parse tree
 */
fn exit_subquery(&mut self, _ctx: &SubqueryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#sortItem}.
 * @param ctx the parse tree
 */
fn enter_sortItem(&mut self, _ctx: &SortItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#sortItem}.
 * @param ctx the parse tree
 */
fn exit_sortItem(&mut self, _ctx: &SortItemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#fromStatement}.
 * @param ctx the parse tree
 */
fn enter_fromStatement(&mut self, _ctx: &FromStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#fromStatement}.
 * @param ctx the parse tree
 */
fn exit_fromStatement(&mut self, _ctx: &FromStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#fromStatementBody}.
 * @param ctx the parse tree
 */
fn enter_fromStatementBody(&mut self, _ctx: &FromStatementBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#fromStatementBody}.
 * @param ctx the parse tree
 */
fn exit_fromStatementBody(&mut self, _ctx: &FromStatementBodyContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code transformQuerySpecification}
 * labeled alternative in {@link SqlBaseParser#querySpecification}.
 * @param ctx the parse tree
 */
fn enter_transformQuerySpecification(&mut self, _ctx: &TransformQuerySpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code transformQuerySpecification}
 * labeled alternative in {@link SqlBaseParser#querySpecification}.
 * @param ctx the parse tree
 */
fn exit_transformQuerySpecification(&mut self, _ctx: &TransformQuerySpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code regularQuerySpecification}
 * labeled alternative in {@link SqlBaseParser#querySpecification}.
 * @param ctx the parse tree
 */
fn enter_regularQuerySpecification(&mut self, _ctx: &RegularQuerySpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code regularQuerySpecification}
 * labeled alternative in {@link SqlBaseParser#querySpecification}.
 * @param ctx the parse tree
 */
fn exit_regularQuerySpecification(&mut self, _ctx: &RegularQuerySpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#transformClause}.
 * @param ctx the parse tree
 */
fn enter_transformClause(&mut self, _ctx: &TransformClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#transformClause}.
 * @param ctx the parse tree
 */
fn exit_transformClause(&mut self, _ctx: &TransformClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#selectClause}.
 * @param ctx the parse tree
 */
fn enter_selectClause(&mut self, _ctx: &SelectClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#selectClause}.
 * @param ctx the parse tree
 */
fn exit_selectClause(&mut self, _ctx: &SelectClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#setClause}.
 * @param ctx the parse tree
 */
fn enter_setClause(&mut self, _ctx: &SetClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#setClause}.
 * @param ctx the parse tree
 */
fn exit_setClause(&mut self, _ctx: &SetClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#matchedClause}.
 * @param ctx the parse tree
 */
fn enter_matchedClause(&mut self, _ctx: &MatchedClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#matchedClause}.
 * @param ctx the parse tree
 */
fn exit_matchedClause(&mut self, _ctx: &MatchedClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#notMatchedClause}.
 * @param ctx the parse tree
 */
fn enter_notMatchedClause(&mut self, _ctx: &NotMatchedClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#notMatchedClause}.
 * @param ctx the parse tree
 */
fn exit_notMatchedClause(&mut self, _ctx: &NotMatchedClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#notMatchedBySourceClause}.
 * @param ctx the parse tree
 */
fn enter_notMatchedBySourceClause(&mut self, _ctx: &NotMatchedBySourceClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#notMatchedBySourceClause}.
 * @param ctx the parse tree
 */
fn exit_notMatchedBySourceClause(&mut self, _ctx: &NotMatchedBySourceClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#matchedAction}.
 * @param ctx the parse tree
 */
fn enter_matchedAction(&mut self, _ctx: &MatchedActionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#matchedAction}.
 * @param ctx the parse tree
 */
fn exit_matchedAction(&mut self, _ctx: &MatchedActionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#notMatchedAction}.
 * @param ctx the parse tree
 */
fn enter_notMatchedAction(&mut self, _ctx: &NotMatchedActionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#notMatchedAction}.
 * @param ctx the parse tree
 */
fn exit_notMatchedAction(&mut self, _ctx: &NotMatchedActionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#notMatchedBySourceAction}.
 * @param ctx the parse tree
 */
fn enter_notMatchedBySourceAction(&mut self, _ctx: &NotMatchedBySourceActionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#notMatchedBySourceAction}.
 * @param ctx the parse tree
 */
fn exit_notMatchedBySourceAction(&mut self, _ctx: &NotMatchedBySourceActionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#exceptClause}.
 * @param ctx the parse tree
 */
fn enter_exceptClause(&mut self, _ctx: &ExceptClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#exceptClause}.
 * @param ctx the parse tree
 */
fn exit_exceptClause(&mut self, _ctx: &ExceptClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#assignmentList}.
 * @param ctx the parse tree
 */
fn enter_assignmentList(&mut self, _ctx: &AssignmentListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#assignmentList}.
 * @param ctx the parse tree
 */
fn exit_assignmentList(&mut self, _ctx: &AssignmentListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#assignment}.
 * @param ctx the parse tree
 */
fn enter_assignment(&mut self, _ctx: &AssignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#assignment}.
 * @param ctx the parse tree
 */
fn exit_assignment(&mut self, _ctx: &AssignmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#whereClause}.
 * @param ctx the parse tree
 */
fn enter_whereClause(&mut self, _ctx: &WhereClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#whereClause}.
 * @param ctx the parse tree
 */
fn exit_whereClause(&mut self, _ctx: &WhereClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#havingClause}.
 * @param ctx the parse tree
 */
fn enter_havingClause(&mut self, _ctx: &HavingClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#havingClause}.
 * @param ctx the parse tree
 */
fn exit_havingClause(&mut self, _ctx: &HavingClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#hint}.
 * @param ctx the parse tree
 */
fn enter_hint(&mut self, _ctx: &HintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#hint}.
 * @param ctx the parse tree
 */
fn exit_hint(&mut self, _ctx: &HintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#hintStatement}.
 * @param ctx the parse tree
 */
fn enter_hintStatement(&mut self, _ctx: &HintStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#hintStatement}.
 * @param ctx the parse tree
 */
fn exit_hintStatement(&mut self, _ctx: &HintStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#fromClause}.
 * @param ctx the parse tree
 */
fn enter_fromClause(&mut self, _ctx: &FromClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#fromClause}.
 * @param ctx the parse tree
 */
fn exit_fromClause(&mut self, _ctx: &FromClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#temporalClause}.
 * @param ctx the parse tree
 */
fn enter_temporalClause(&mut self, _ctx: &TemporalClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#temporalClause}.
 * @param ctx the parse tree
 */
fn exit_temporalClause(&mut self, _ctx: &TemporalClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#aggregationClause}.
 * @param ctx the parse tree
 */
fn enter_aggregationClause(&mut self, _ctx: &AggregationClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#aggregationClause}.
 * @param ctx the parse tree
 */
fn exit_aggregationClause(&mut self, _ctx: &AggregationClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#groupByClause}.
 * @param ctx the parse tree
 */
fn enter_groupByClause(&mut self, _ctx: &GroupByClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#groupByClause}.
 * @param ctx the parse tree
 */
fn exit_groupByClause(&mut self, _ctx: &GroupByClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#groupingAnalytics}.
 * @param ctx the parse tree
 */
fn enter_groupingAnalytics(&mut self, _ctx: &GroupingAnalyticsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#groupingAnalytics}.
 * @param ctx the parse tree
 */
fn exit_groupingAnalytics(&mut self, _ctx: &GroupingAnalyticsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#groupingElement}.
 * @param ctx the parse tree
 */
fn enter_groupingElement(&mut self, _ctx: &GroupingElementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#groupingElement}.
 * @param ctx the parse tree
 */
fn exit_groupingElement(&mut self, _ctx: &GroupingElementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#groupingSet}.
 * @param ctx the parse tree
 */
fn enter_groupingSet(&mut self, _ctx: &GroupingSetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#groupingSet}.
 * @param ctx the parse tree
 */
fn exit_groupingSet(&mut self, _ctx: &GroupingSetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#pivotClause}.
 * @param ctx the parse tree
 */
fn enter_pivotClause(&mut self, _ctx: &PivotClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#pivotClause}.
 * @param ctx the parse tree
 */
fn exit_pivotClause(&mut self, _ctx: &PivotClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#pivotColumn}.
 * @param ctx the parse tree
 */
fn enter_pivotColumn(&mut self, _ctx: &PivotColumnContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#pivotColumn}.
 * @param ctx the parse tree
 */
fn exit_pivotColumn(&mut self, _ctx: &PivotColumnContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#pivotValue}.
 * @param ctx the parse tree
 */
fn enter_pivotValue(&mut self, _ctx: &PivotValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#pivotValue}.
 * @param ctx the parse tree
 */
fn exit_pivotValue(&mut self, _ctx: &PivotValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#unpivotClause}.
 * @param ctx the parse tree
 */
fn enter_unpivotClause(&mut self, _ctx: &UnpivotClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#unpivotClause}.
 * @param ctx the parse tree
 */
fn exit_unpivotClause(&mut self, _ctx: &UnpivotClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#unpivotNullClause}.
 * @param ctx the parse tree
 */
fn enter_unpivotNullClause(&mut self, _ctx: &UnpivotNullClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#unpivotNullClause}.
 * @param ctx the parse tree
 */
fn exit_unpivotNullClause(&mut self, _ctx: &UnpivotNullClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#unpivotOperator}.
 * @param ctx the parse tree
 */
fn enter_unpivotOperator(&mut self, _ctx: &UnpivotOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#unpivotOperator}.
 * @param ctx the parse tree
 */
fn exit_unpivotOperator(&mut self, _ctx: &UnpivotOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#unpivotSingleValueColumnClause}.
 * @param ctx the parse tree
 */
fn enter_unpivotSingleValueColumnClause(&mut self, _ctx: &UnpivotSingleValueColumnClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#unpivotSingleValueColumnClause}.
 * @param ctx the parse tree
 */
fn exit_unpivotSingleValueColumnClause(&mut self, _ctx: &UnpivotSingleValueColumnClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#unpivotMultiValueColumnClause}.
 * @param ctx the parse tree
 */
fn enter_unpivotMultiValueColumnClause(&mut self, _ctx: &UnpivotMultiValueColumnClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#unpivotMultiValueColumnClause}.
 * @param ctx the parse tree
 */
fn exit_unpivotMultiValueColumnClause(&mut self, _ctx: &UnpivotMultiValueColumnClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#unpivotColumnSet}.
 * @param ctx the parse tree
 */
fn enter_unpivotColumnSet(&mut self, _ctx: &UnpivotColumnSetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#unpivotColumnSet}.
 * @param ctx the parse tree
 */
fn exit_unpivotColumnSet(&mut self, _ctx: &UnpivotColumnSetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#unpivotValueColumn}.
 * @param ctx the parse tree
 */
fn enter_unpivotValueColumn(&mut self, _ctx: &UnpivotValueColumnContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#unpivotValueColumn}.
 * @param ctx the parse tree
 */
fn exit_unpivotValueColumn(&mut self, _ctx: &UnpivotValueColumnContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#unpivotNameColumn}.
 * @param ctx the parse tree
 */
fn enter_unpivotNameColumn(&mut self, _ctx: &UnpivotNameColumnContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#unpivotNameColumn}.
 * @param ctx the parse tree
 */
fn exit_unpivotNameColumn(&mut self, _ctx: &UnpivotNameColumnContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#unpivotColumnAndAlias}.
 * @param ctx the parse tree
 */
fn enter_unpivotColumnAndAlias(&mut self, _ctx: &UnpivotColumnAndAliasContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#unpivotColumnAndAlias}.
 * @param ctx the parse tree
 */
fn exit_unpivotColumnAndAlias(&mut self, _ctx: &UnpivotColumnAndAliasContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#unpivotColumn}.
 * @param ctx the parse tree
 */
fn enter_unpivotColumn(&mut self, _ctx: &UnpivotColumnContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#unpivotColumn}.
 * @param ctx the parse tree
 */
fn exit_unpivotColumn(&mut self, _ctx: &UnpivotColumnContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#unpivotAlias}.
 * @param ctx the parse tree
 */
fn enter_unpivotAlias(&mut self, _ctx: &UnpivotAliasContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#unpivotAlias}.
 * @param ctx the parse tree
 */
fn exit_unpivotAlias(&mut self, _ctx: &UnpivotAliasContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#lateralView}.
 * @param ctx the parse tree
 */
fn enter_lateralView(&mut self, _ctx: &LateralViewContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#lateralView}.
 * @param ctx the parse tree
 */
fn exit_lateralView(&mut self, _ctx: &LateralViewContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#watermarkClause}.
 * @param ctx the parse tree
 */
fn enter_watermarkClause(&mut self, _ctx: &WatermarkClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#watermarkClause}.
 * @param ctx the parse tree
 */
fn exit_watermarkClause(&mut self, _ctx: &WatermarkClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#setQuantifier}.
 * @param ctx the parse tree
 */
fn enter_setQuantifier(&mut self, _ctx: &SetQuantifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#setQuantifier}.
 * @param ctx the parse tree
 */
fn exit_setQuantifier(&mut self, _ctx: &SetQuantifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#relation}.
 * @param ctx the parse tree
 */
fn enter_relation(&mut self, _ctx: &RelationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#relation}.
 * @param ctx the parse tree
 */
fn exit_relation(&mut self, _ctx: &RelationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#relationExtension}.
 * @param ctx the parse tree
 */
fn enter_relationExtension(&mut self, _ctx: &RelationExtensionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#relationExtension}.
 * @param ctx the parse tree
 */
fn exit_relationExtension(&mut self, _ctx: &RelationExtensionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#joinRelation}.
 * @param ctx the parse tree
 */
fn enter_joinRelation(&mut self, _ctx: &JoinRelationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#joinRelation}.
 * @param ctx the parse tree
 */
fn exit_joinRelation(&mut self, _ctx: &JoinRelationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#joinType}.
 * @param ctx the parse tree
 */
fn enter_joinType(&mut self, _ctx: &JoinTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#joinType}.
 * @param ctx the parse tree
 */
fn exit_joinType(&mut self, _ctx: &JoinTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#joinCriteria}.
 * @param ctx the parse tree
 */
fn enter_joinCriteria(&mut self, _ctx: &JoinCriteriaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#joinCriteria}.
 * @param ctx the parse tree
 */
fn exit_joinCriteria(&mut self, _ctx: &JoinCriteriaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#sample}.
 * @param ctx the parse tree
 */
fn enter_sample(&mut self, _ctx: &SampleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#sample}.
 * @param ctx the parse tree
 */
fn exit_sample(&mut self, _ctx: &SampleContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code sampleByPercentile}
 * labeled alternative in {@link SqlBaseParser#sampleMethod}.
 * @param ctx the parse tree
 */
fn enter_sampleByPercentile(&mut self, _ctx: &SampleByPercentileContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code sampleByPercentile}
 * labeled alternative in {@link SqlBaseParser#sampleMethod}.
 * @param ctx the parse tree
 */
fn exit_sampleByPercentile(&mut self, _ctx: &SampleByPercentileContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code sampleByRows}
 * labeled alternative in {@link SqlBaseParser#sampleMethod}.
 * @param ctx the parse tree
 */
fn enter_sampleByRows(&mut self, _ctx: &SampleByRowsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code sampleByRows}
 * labeled alternative in {@link SqlBaseParser#sampleMethod}.
 * @param ctx the parse tree
 */
fn exit_sampleByRows(&mut self, _ctx: &SampleByRowsContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code sampleByBucket}
 * labeled alternative in {@link SqlBaseParser#sampleMethod}.
 * @param ctx the parse tree
 */
fn enter_sampleByBucket(&mut self, _ctx: &SampleByBucketContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code sampleByBucket}
 * labeled alternative in {@link SqlBaseParser#sampleMethod}.
 * @param ctx the parse tree
 */
fn exit_sampleByBucket(&mut self, _ctx: &SampleByBucketContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code sampleByBytes}
 * labeled alternative in {@link SqlBaseParser#sampleMethod}.
 * @param ctx the parse tree
 */
fn enter_sampleByBytes(&mut self, _ctx: &SampleByBytesContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code sampleByBytes}
 * labeled alternative in {@link SqlBaseParser#sampleMethod}.
 * @param ctx the parse tree
 */
fn exit_sampleByBytes(&mut self, _ctx: &SampleByBytesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#identifierList}.
 * @param ctx the parse tree
 */
fn enter_identifierList(&mut self, _ctx: &IdentifierListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#identifierList}.
 * @param ctx the parse tree
 */
fn exit_identifierList(&mut self, _ctx: &IdentifierListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#identifierSeq}.
 * @param ctx the parse tree
 */
fn enter_identifierSeq(&mut self, _ctx: &IdentifierSeqContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#identifierSeq}.
 * @param ctx the parse tree
 */
fn exit_identifierSeq(&mut self, _ctx: &IdentifierSeqContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#orderedIdentifierList}.
 * @param ctx the parse tree
 */
fn enter_orderedIdentifierList(&mut self, _ctx: &OrderedIdentifierListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#orderedIdentifierList}.
 * @param ctx the parse tree
 */
fn exit_orderedIdentifierList(&mut self, _ctx: &OrderedIdentifierListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#orderedIdentifier}.
 * @param ctx the parse tree
 */
fn enter_orderedIdentifier(&mut self, _ctx: &OrderedIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#orderedIdentifier}.
 * @param ctx the parse tree
 */
fn exit_orderedIdentifier(&mut self, _ctx: &OrderedIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#identifierCommentList}.
 * @param ctx the parse tree
 */
fn enter_identifierCommentList(&mut self, _ctx: &IdentifierCommentListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#identifierCommentList}.
 * @param ctx the parse tree
 */
fn exit_identifierCommentList(&mut self, _ctx: &IdentifierCommentListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#identifierComment}.
 * @param ctx the parse tree
 */
fn enter_identifierComment(&mut self, _ctx: &IdentifierCommentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#identifierComment}.
 * @param ctx the parse tree
 */
fn exit_identifierComment(&mut self, _ctx: &IdentifierCommentContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code streamRelation}
 * labeled alternative in {@link SqlBaseParser#relationPrimary}.
 * @param ctx the parse tree
 */
fn enter_streamRelation(&mut self, _ctx: &StreamRelationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code streamRelation}
 * labeled alternative in {@link SqlBaseParser#relationPrimary}.
 * @param ctx the parse tree
 */
fn exit_streamRelation(&mut self, _ctx: &StreamRelationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code tableName}
 * labeled alternative in {@link SqlBaseParser#relationPrimary}.
 * @param ctx the parse tree
 */
fn enter_tableName(&mut self, _ctx: &TableNameContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code tableName}
 * labeled alternative in {@link SqlBaseParser#relationPrimary}.
 * @param ctx the parse tree
 */
fn exit_tableName(&mut self, _ctx: &TableNameContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code aliasedQuery}
 * labeled alternative in {@link SqlBaseParser#relationPrimary}.
 * @param ctx the parse tree
 */
fn enter_aliasedQuery(&mut self, _ctx: &AliasedQueryContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code aliasedQuery}
 * labeled alternative in {@link SqlBaseParser#relationPrimary}.
 * @param ctx the parse tree
 */
fn exit_aliasedQuery(&mut self, _ctx: &AliasedQueryContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code aliasedRelation}
 * labeled alternative in {@link SqlBaseParser#relationPrimary}.
 * @param ctx the parse tree
 */
fn enter_aliasedRelation(&mut self, _ctx: &AliasedRelationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code aliasedRelation}
 * labeled alternative in {@link SqlBaseParser#relationPrimary}.
 * @param ctx the parse tree
 */
fn exit_aliasedRelation(&mut self, _ctx: &AliasedRelationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code inlineTableDefault2}
 * labeled alternative in {@link SqlBaseParser#relationPrimary}.
 * @param ctx the parse tree
 */
fn enter_inlineTableDefault2(&mut self, _ctx: &InlineTableDefault2Context<'input>) { }
/**
 * Exit a parse tree produced by the {@code inlineTableDefault2}
 * labeled alternative in {@link SqlBaseParser#relationPrimary}.
 * @param ctx the parse tree
 */
fn exit_inlineTableDefault2(&mut self, _ctx: &InlineTableDefault2Context<'input>) { }
/**
 * Enter a parse tree produced by the {@code tableValuedFunction}
 * labeled alternative in {@link SqlBaseParser#relationPrimary}.
 * @param ctx the parse tree
 */
fn enter_tableValuedFunction(&mut self, _ctx: &TableValuedFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code tableValuedFunction}
 * labeled alternative in {@link SqlBaseParser#relationPrimary}.
 * @param ctx the parse tree
 */
fn exit_tableValuedFunction(&mut self, _ctx: &TableValuedFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#optionsClause}.
 * @param ctx the parse tree
 */
fn enter_optionsClause(&mut self, _ctx: &OptionsClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#optionsClause}.
 * @param ctx the parse tree
 */
fn exit_optionsClause(&mut self, _ctx: &OptionsClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#inlineTable}.
 * @param ctx the parse tree
 */
fn enter_inlineTable(&mut self, _ctx: &InlineTableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#inlineTable}.
 * @param ctx the parse tree
 */
fn exit_inlineTable(&mut self, _ctx: &InlineTableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#functionTableSubqueryArgument}.
 * @param ctx the parse tree
 */
fn enter_functionTableSubqueryArgument(&mut self, _ctx: &FunctionTableSubqueryArgumentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#functionTableSubqueryArgument}.
 * @param ctx the parse tree
 */
fn exit_functionTableSubqueryArgument(&mut self, _ctx: &FunctionTableSubqueryArgumentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#tableArgumentPartitioning}.
 * @param ctx the parse tree
 */
fn enter_tableArgumentPartitioning(&mut self, _ctx: &TableArgumentPartitioningContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#tableArgumentPartitioning}.
 * @param ctx the parse tree
 */
fn exit_tableArgumentPartitioning(&mut self, _ctx: &TableArgumentPartitioningContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#functionTableNamedArgumentExpression}.
 * @param ctx the parse tree
 */
fn enter_functionTableNamedArgumentExpression(&mut self, _ctx: &FunctionTableNamedArgumentExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#functionTableNamedArgumentExpression}.
 * @param ctx the parse tree
 */
fn exit_functionTableNamedArgumentExpression(&mut self, _ctx: &FunctionTableNamedArgumentExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#functionTableReferenceArgument}.
 * @param ctx the parse tree
 */
fn enter_functionTableReferenceArgument(&mut self, _ctx: &FunctionTableReferenceArgumentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#functionTableReferenceArgument}.
 * @param ctx the parse tree
 */
fn exit_functionTableReferenceArgument(&mut self, _ctx: &FunctionTableReferenceArgumentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#functionTableArgument}.
 * @param ctx the parse tree
 */
fn enter_functionTableArgument(&mut self, _ctx: &FunctionTableArgumentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#functionTableArgument}.
 * @param ctx the parse tree
 */
fn exit_functionTableArgument(&mut self, _ctx: &FunctionTableArgumentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#functionTable}.
 * @param ctx the parse tree
 */
fn enter_functionTable(&mut self, _ctx: &FunctionTableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#functionTable}.
 * @param ctx the parse tree
 */
fn exit_functionTable(&mut self, _ctx: &FunctionTableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#tableAlias}.
 * @param ctx the parse tree
 */
fn enter_tableAlias(&mut self, _ctx: &TableAliasContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#tableAlias}.
 * @param ctx the parse tree
 */
fn exit_tableAlias(&mut self, _ctx: &TableAliasContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code rowFormatSerde}
 * labeled alternative in {@link SqlBaseParser#rowFormat}.
 * @param ctx the parse tree
 */
fn enter_rowFormatSerde(&mut self, _ctx: &RowFormatSerdeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code rowFormatSerde}
 * labeled alternative in {@link SqlBaseParser#rowFormat}.
 * @param ctx the parse tree
 */
fn exit_rowFormatSerde(&mut self, _ctx: &RowFormatSerdeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code rowFormatDelimited}
 * labeled alternative in {@link SqlBaseParser#rowFormat}.
 * @param ctx the parse tree
 */
fn enter_rowFormatDelimited(&mut self, _ctx: &RowFormatDelimitedContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code rowFormatDelimited}
 * labeled alternative in {@link SqlBaseParser#rowFormat}.
 * @param ctx the parse tree
 */
fn exit_rowFormatDelimited(&mut self, _ctx: &RowFormatDelimitedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#multipartIdentifierList}.
 * @param ctx the parse tree
 */
fn enter_multipartIdentifierList(&mut self, _ctx: &MultipartIdentifierListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#multipartIdentifierList}.
 * @param ctx the parse tree
 */
fn exit_multipartIdentifierList(&mut self, _ctx: &MultipartIdentifierListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#multipartIdentifier}.
 * @param ctx the parse tree
 */
fn enter_multipartIdentifier(&mut self, _ctx: &MultipartIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#multipartIdentifier}.
 * @param ctx the parse tree
 */
fn exit_multipartIdentifier(&mut self, _ctx: &MultipartIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#multipartIdentifierPropertyList}.
 * @param ctx the parse tree
 */
fn enter_multipartIdentifierPropertyList(&mut self, _ctx: &MultipartIdentifierPropertyListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#multipartIdentifierPropertyList}.
 * @param ctx the parse tree
 */
fn exit_multipartIdentifierPropertyList(&mut self, _ctx: &MultipartIdentifierPropertyListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#multipartIdentifierProperty}.
 * @param ctx the parse tree
 */
fn enter_multipartIdentifierProperty(&mut self, _ctx: &MultipartIdentifierPropertyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#multipartIdentifierProperty}.
 * @param ctx the parse tree
 */
fn exit_multipartIdentifierProperty(&mut self, _ctx: &MultipartIdentifierPropertyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#tableIdentifier}.
 * @param ctx the parse tree
 */
fn enter_tableIdentifier(&mut self, _ctx: &TableIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#tableIdentifier}.
 * @param ctx the parse tree
 */
fn exit_tableIdentifier(&mut self, _ctx: &TableIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#functionIdentifier}.
 * @param ctx the parse tree
 */
fn enter_functionIdentifier(&mut self, _ctx: &FunctionIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#functionIdentifier}.
 * @param ctx the parse tree
 */
fn exit_functionIdentifier(&mut self, _ctx: &FunctionIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#namedExpression}.
 * @param ctx the parse tree
 */
fn enter_namedExpression(&mut self, _ctx: &NamedExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#namedExpression}.
 * @param ctx the parse tree
 */
fn exit_namedExpression(&mut self, _ctx: &NamedExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#namedExpressionSeq}.
 * @param ctx the parse tree
 */
fn enter_namedExpressionSeq(&mut self, _ctx: &NamedExpressionSeqContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#namedExpressionSeq}.
 * @param ctx the parse tree
 */
fn exit_namedExpressionSeq(&mut self, _ctx: &NamedExpressionSeqContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#partitionFieldList}.
 * @param ctx the parse tree
 */
fn enter_partitionFieldList(&mut self, _ctx: &PartitionFieldListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#partitionFieldList}.
 * @param ctx the parse tree
 */
fn exit_partitionFieldList(&mut self, _ctx: &PartitionFieldListContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code partitionTransform}
 * labeled alternative in {@link SqlBaseParser#partitionField}.
 * @param ctx the parse tree
 */
fn enter_partitionTransform(&mut self, _ctx: &PartitionTransformContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code partitionTransform}
 * labeled alternative in {@link SqlBaseParser#partitionField}.
 * @param ctx the parse tree
 */
fn exit_partitionTransform(&mut self, _ctx: &PartitionTransformContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code partitionColumn}
 * labeled alternative in {@link SqlBaseParser#partitionField}.
 * @param ctx the parse tree
 */
fn enter_partitionColumn(&mut self, _ctx: &PartitionColumnContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code partitionColumn}
 * labeled alternative in {@link SqlBaseParser#partitionField}.
 * @param ctx the parse tree
 */
fn exit_partitionColumn(&mut self, _ctx: &PartitionColumnContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code identityTransform}
 * labeled alternative in {@link SqlBaseParser#transform}.
 * @param ctx the parse tree
 */
fn enter_identityTransform(&mut self, _ctx: &IdentityTransformContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code identityTransform}
 * labeled alternative in {@link SqlBaseParser#transform}.
 * @param ctx the parse tree
 */
fn exit_identityTransform(&mut self, _ctx: &IdentityTransformContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code applyTransform}
 * labeled alternative in {@link SqlBaseParser#transform}.
 * @param ctx the parse tree
 */
fn enter_applyTransform(&mut self, _ctx: &ApplyTransformContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code applyTransform}
 * labeled alternative in {@link SqlBaseParser#transform}.
 * @param ctx the parse tree
 */
fn exit_applyTransform(&mut self, _ctx: &ApplyTransformContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#transformArgument}.
 * @param ctx the parse tree
 */
fn enter_transformArgument(&mut self, _ctx: &TransformArgumentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#transformArgument}.
 * @param ctx the parse tree
 */
fn exit_transformArgument(&mut self, _ctx: &TransformArgumentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#namedArgumentExpression}.
 * @param ctx the parse tree
 */
fn enter_namedArgumentExpression(&mut self, _ctx: &NamedArgumentExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#namedArgumentExpression}.
 * @param ctx the parse tree
 */
fn exit_namedArgumentExpression(&mut self, _ctx: &NamedArgumentExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#functionArgument}.
 * @param ctx the parse tree
 */
fn enter_functionArgument(&mut self, _ctx: &FunctionArgumentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#functionArgument}.
 * @param ctx the parse tree
 */
fn exit_functionArgument(&mut self, _ctx: &FunctionArgumentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#expressionSeq}.
 * @param ctx the parse tree
 */
fn enter_expressionSeq(&mut self, _ctx: &ExpressionSeqContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#expressionSeq}.
 * @param ctx the parse tree
 */
fn exit_expressionSeq(&mut self, _ctx: &ExpressionSeqContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code logicalNot}
 * labeled alternative in {@link SqlBaseParser#booleanExpression}.
 * @param ctx the parse tree
 */
fn enter_logicalNot(&mut self, _ctx: &LogicalNotContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code logicalNot}
 * labeled alternative in {@link SqlBaseParser#booleanExpression}.
 * @param ctx the parse tree
 */
fn exit_logicalNot(&mut self, _ctx: &LogicalNotContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code predicated}
 * labeled alternative in {@link SqlBaseParser#booleanExpression}.
 * @param ctx the parse tree
 */
fn enter_predicated(&mut self, _ctx: &PredicatedContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code predicated}
 * labeled alternative in {@link SqlBaseParser#booleanExpression}.
 * @param ctx the parse tree
 */
fn exit_predicated(&mut self, _ctx: &PredicatedContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code exists}
 * labeled alternative in {@link SqlBaseParser#booleanExpression}.
 * @param ctx the parse tree
 */
fn enter_exists(&mut self, _ctx: &ExistsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code exists}
 * labeled alternative in {@link SqlBaseParser#booleanExpression}.
 * @param ctx the parse tree
 */
fn exit_exists(&mut self, _ctx: &ExistsContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code logicalBinary}
 * labeled alternative in {@link SqlBaseParser#booleanExpression}.
 * @param ctx the parse tree
 */
fn enter_logicalBinary(&mut self, _ctx: &LogicalBinaryContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code logicalBinary}
 * labeled alternative in {@link SqlBaseParser#booleanExpression}.
 * @param ctx the parse tree
 */
fn exit_logicalBinary(&mut self, _ctx: &LogicalBinaryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#predicate}.
 * @param ctx the parse tree
 */
fn enter_predicate(&mut self, _ctx: &PredicateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#predicate}.
 * @param ctx the parse tree
 */
fn exit_predicate(&mut self, _ctx: &PredicateContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#errorCapturingNot}.
 * @param ctx the parse tree
 */
fn enter_errorCapturingNot(&mut self, _ctx: &ErrorCapturingNotContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#errorCapturingNot}.
 * @param ctx the parse tree
 */
fn exit_errorCapturingNot(&mut self, _ctx: &ErrorCapturingNotContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code valueExpressionDefault}
 * labeled alternative in {@link SqlBaseParser#valueExpression}.
 * @param ctx the parse tree
 */
fn enter_valueExpressionDefault(&mut self, _ctx: &ValueExpressionDefaultContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code valueExpressionDefault}
 * labeled alternative in {@link SqlBaseParser#valueExpression}.
 * @param ctx the parse tree
 */
fn exit_valueExpressionDefault(&mut self, _ctx: &ValueExpressionDefaultContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code comparison}
 * labeled alternative in {@link SqlBaseParser#valueExpression}.
 * @param ctx the parse tree
 */
fn enter_comparison(&mut self, _ctx: &ComparisonContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code comparison}
 * labeled alternative in {@link SqlBaseParser#valueExpression}.
 * @param ctx the parse tree
 */
fn exit_comparison(&mut self, _ctx: &ComparisonContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code shiftExpression}
 * labeled alternative in {@link SqlBaseParser#valueExpression}.
 * @param ctx the parse tree
 */
fn enter_shiftExpression(&mut self, _ctx: &ShiftExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code shiftExpression}
 * labeled alternative in {@link SqlBaseParser#valueExpression}.
 * @param ctx the parse tree
 */
fn exit_shiftExpression(&mut self, _ctx: &ShiftExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code arithmeticBinary}
 * labeled alternative in {@link SqlBaseParser#valueExpression}.
 * @param ctx the parse tree
 */
fn enter_arithmeticBinary(&mut self, _ctx: &ArithmeticBinaryContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code arithmeticBinary}
 * labeled alternative in {@link SqlBaseParser#valueExpression}.
 * @param ctx the parse tree
 */
fn exit_arithmeticBinary(&mut self, _ctx: &ArithmeticBinaryContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code arithmeticUnary}
 * labeled alternative in {@link SqlBaseParser#valueExpression}.
 * @param ctx the parse tree
 */
fn enter_arithmeticUnary(&mut self, _ctx: &ArithmeticUnaryContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code arithmeticUnary}
 * labeled alternative in {@link SqlBaseParser#valueExpression}.
 * @param ctx the parse tree
 */
fn exit_arithmeticUnary(&mut self, _ctx: &ArithmeticUnaryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#shiftOperator}.
 * @param ctx the parse tree
 */
fn enter_shiftOperator(&mut self, _ctx: &ShiftOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#shiftOperator}.
 * @param ctx the parse tree
 */
fn exit_shiftOperator(&mut self, _ctx: &ShiftOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#datetimeUnit}.
 * @param ctx the parse tree
 */
fn enter_datetimeUnit(&mut self, _ctx: &DatetimeUnitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#datetimeUnit}.
 * @param ctx the parse tree
 */
fn exit_datetimeUnit(&mut self, _ctx: &DatetimeUnitContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code struct}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_struct(&mut self, _ctx: &StructContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code struct}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_struct(&mut self, _ctx: &StructContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code dereference}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_dereference(&mut self, _ctx: &DereferenceContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code dereference}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_dereference(&mut self, _ctx: &DereferenceContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code castByColon}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_castByColon(&mut self, _ctx: &CastByColonContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code castByColon}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_castByColon(&mut self, _ctx: &CastByColonContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code timestampadd}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_timestampadd(&mut self, _ctx: &TimestampaddContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code timestampadd}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_timestampadd(&mut self, _ctx: &TimestampaddContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code substring}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_substring(&mut self, _ctx: &SubstringContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code substring}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_substring(&mut self, _ctx: &SubstringContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code cast}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_cast(&mut self, _ctx: &CastContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code cast}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_cast(&mut self, _ctx: &CastContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code lambda}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_lambda(&mut self, _ctx: &LambdaContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code lambda}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_lambda(&mut self, _ctx: &LambdaContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code parenthesizedExpression}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_parenthesizedExpression(&mut self, _ctx: &ParenthesizedExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code parenthesizedExpression}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_parenthesizedExpression(&mut self, _ctx: &ParenthesizedExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code any_value}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_any_value(&mut self, _ctx: &Any_valueContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code any_value}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_any_value(&mut self, _ctx: &Any_valueContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code trim}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_trim(&mut self, _ctx: &TrimContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code trim}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_trim(&mut self, _ctx: &TrimContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code semiStructuredExtract}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_semiStructuredExtract(&mut self, _ctx: &SemiStructuredExtractContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code semiStructuredExtract}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_semiStructuredExtract(&mut self, _ctx: &SemiStructuredExtractContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code simpleCase}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_simpleCase(&mut self, _ctx: &SimpleCaseContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code simpleCase}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_simpleCase(&mut self, _ctx: &SimpleCaseContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code currentLike}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_currentLike(&mut self, _ctx: &CurrentLikeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code currentLike}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_currentLike(&mut self, _ctx: &CurrentLikeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code columnReference}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_columnReference(&mut self, _ctx: &ColumnReferenceContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code columnReference}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_columnReference(&mut self, _ctx: &ColumnReferenceContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code rowConstructor}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_rowConstructor(&mut self, _ctx: &RowConstructorContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code rowConstructor}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_rowConstructor(&mut self, _ctx: &RowConstructorContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code last}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_last(&mut self, _ctx: &LastContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code last}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_last(&mut self, _ctx: &LastContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code star}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_star(&mut self, _ctx: &StarContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code star}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_star(&mut self, _ctx: &StarContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code overlay}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_overlay(&mut self, _ctx: &OverlayContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code overlay}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_overlay(&mut self, _ctx: &OverlayContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code subscript}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_subscript(&mut self, _ctx: &SubscriptContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code subscript}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_subscript(&mut self, _ctx: &SubscriptContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code timestampdiff}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_timestampdiff(&mut self, _ctx: &TimestampdiffContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code timestampdiff}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_timestampdiff(&mut self, _ctx: &TimestampdiffContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code subqueryExpression}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_subqueryExpression(&mut self, _ctx: &SubqueryExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code subqueryExpression}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_subqueryExpression(&mut self, _ctx: &SubqueryExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code collate}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_collate(&mut self, _ctx: &CollateContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code collate}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_collate(&mut self, _ctx: &CollateContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code constantDefault}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_constantDefault(&mut self, _ctx: &ConstantDefaultContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code constantDefault}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_constantDefault(&mut self, _ctx: &ConstantDefaultContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code extract}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_extract(&mut self, _ctx: &ExtractContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code extract}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_extract(&mut self, _ctx: &ExtractContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code functionCall}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_functionCall(&mut self, _ctx: &FunctionCallContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code functionCall}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_functionCall(&mut self, _ctx: &FunctionCallContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code searchedCase}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_searchedCase(&mut self, _ctx: &SearchedCaseContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code searchedCase}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_searchedCase(&mut self, _ctx: &SearchedCaseContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code position}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_position(&mut self, _ctx: &PositionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code position}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_position(&mut self, _ctx: &PositionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code first}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_first(&mut self, _ctx: &FirstContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code first}
 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_first(&mut self, _ctx: &FirstContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#semiStructuredExtractionPath}.
 * @param ctx the parse tree
 */
fn enter_semiStructuredExtractionPath(&mut self, _ctx: &SemiStructuredExtractionPathContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#semiStructuredExtractionPath}.
 * @param ctx the parse tree
 */
fn exit_semiStructuredExtractionPath(&mut self, _ctx: &SemiStructuredExtractionPathContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#jsonPathIdentifier}.
 * @param ctx the parse tree
 */
fn enter_jsonPathIdentifier(&mut self, _ctx: &JsonPathIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#jsonPathIdentifier}.
 * @param ctx the parse tree
 */
fn exit_jsonPathIdentifier(&mut self, _ctx: &JsonPathIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#jsonPathBracketedIdentifier}.
 * @param ctx the parse tree
 */
fn enter_jsonPathBracketedIdentifier(&mut self, _ctx: &JsonPathBracketedIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#jsonPathBracketedIdentifier}.
 * @param ctx the parse tree
 */
fn exit_jsonPathBracketedIdentifier(&mut self, _ctx: &JsonPathBracketedIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#jsonPathFirstPart}.
 * @param ctx the parse tree
 */
fn enter_jsonPathFirstPart(&mut self, _ctx: &JsonPathFirstPartContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#jsonPathFirstPart}.
 * @param ctx the parse tree
 */
fn exit_jsonPathFirstPart(&mut self, _ctx: &JsonPathFirstPartContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#jsonPathParts}.
 * @param ctx the parse tree
 */
fn enter_jsonPathParts(&mut self, _ctx: &JsonPathPartsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#jsonPathParts}.
 * @param ctx the parse tree
 */
fn exit_jsonPathParts(&mut self, _ctx: &JsonPathPartsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#literalType}.
 * @param ctx the parse tree
 */
fn enter_literalType(&mut self, _ctx: &LiteralTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#literalType}.
 * @param ctx the parse tree
 */
fn exit_literalType(&mut self, _ctx: &LiteralTypeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code nullLiteral}
 * labeled alternative in {@link SqlBaseParser#constant}.
 * @param ctx the parse tree
 */
fn enter_nullLiteral(&mut self, _ctx: &NullLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code nullLiteral}
 * labeled alternative in {@link SqlBaseParser#constant}.
 * @param ctx the parse tree
 */
fn exit_nullLiteral(&mut self, _ctx: &NullLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code posParameterLiteral}
 * labeled alternative in {@link SqlBaseParser#constant}.
 * @param ctx the parse tree
 */
fn enter_posParameterLiteral(&mut self, _ctx: &PosParameterLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code posParameterLiteral}
 * labeled alternative in {@link SqlBaseParser#constant}.
 * @param ctx the parse tree
 */
fn exit_posParameterLiteral(&mut self, _ctx: &PosParameterLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code namedParameterLiteral}
 * labeled alternative in {@link SqlBaseParser#constant}.
 * @param ctx the parse tree
 */
fn enter_namedParameterLiteral(&mut self, _ctx: &NamedParameterLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code namedParameterLiteral}
 * labeled alternative in {@link SqlBaseParser#constant}.
 * @param ctx the parse tree
 */
fn exit_namedParameterLiteral(&mut self, _ctx: &NamedParameterLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code intervalLiteral}
 * labeled alternative in {@link SqlBaseParser#constant}.
 * @param ctx the parse tree
 */
fn enter_intervalLiteral(&mut self, _ctx: &IntervalLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code intervalLiteral}
 * labeled alternative in {@link SqlBaseParser#constant}.
 * @param ctx the parse tree
 */
fn exit_intervalLiteral(&mut self, _ctx: &IntervalLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code typeConstructor}
 * labeled alternative in {@link SqlBaseParser#constant}.
 * @param ctx the parse tree
 */
fn enter_typeConstructor(&mut self, _ctx: &TypeConstructorContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code typeConstructor}
 * labeled alternative in {@link SqlBaseParser#constant}.
 * @param ctx the parse tree
 */
fn exit_typeConstructor(&mut self, _ctx: &TypeConstructorContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code numericLiteral}
 * labeled alternative in {@link SqlBaseParser#constant}.
 * @param ctx the parse tree
 */
fn enter_numericLiteral(&mut self, _ctx: &NumericLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code numericLiteral}
 * labeled alternative in {@link SqlBaseParser#constant}.
 * @param ctx the parse tree
 */
fn exit_numericLiteral(&mut self, _ctx: &NumericLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code booleanLiteral}
 * labeled alternative in {@link SqlBaseParser#constant}.
 * @param ctx the parse tree
 */
fn enter_booleanLiteral(&mut self, _ctx: &BooleanLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code booleanLiteral}
 * labeled alternative in {@link SqlBaseParser#constant}.
 * @param ctx the parse tree
 */
fn exit_booleanLiteral(&mut self, _ctx: &BooleanLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code stringLiteral}
 * labeled alternative in {@link SqlBaseParser#constant}.
 * @param ctx the parse tree
 */
fn enter_stringLiteral(&mut self, _ctx: &StringLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code stringLiteral}
 * labeled alternative in {@link SqlBaseParser#constant}.
 * @param ctx the parse tree
 */
fn exit_stringLiteral(&mut self, _ctx: &StringLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#namedParameterMarker}.
 * @param ctx the parse tree
 */
fn enter_namedParameterMarker(&mut self, _ctx: &NamedParameterMarkerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#namedParameterMarker}.
 * @param ctx the parse tree
 */
fn exit_namedParameterMarker(&mut self, _ctx: &NamedParameterMarkerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#comparisonOperator}.
 * @param ctx the parse tree
 */
fn enter_comparisonOperator(&mut self, _ctx: &ComparisonOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#comparisonOperator}.
 * @param ctx the parse tree
 */
fn exit_comparisonOperator(&mut self, _ctx: &ComparisonOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#arithmeticOperator}.
 * @param ctx the parse tree
 */
fn enter_arithmeticOperator(&mut self, _ctx: &ArithmeticOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#arithmeticOperator}.
 * @param ctx the parse tree
 */
fn exit_arithmeticOperator(&mut self, _ctx: &ArithmeticOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#predicateOperator}.
 * @param ctx the parse tree
 */
fn enter_predicateOperator(&mut self, _ctx: &PredicateOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#predicateOperator}.
 * @param ctx the parse tree
 */
fn exit_predicateOperator(&mut self, _ctx: &PredicateOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#booleanValue}.
 * @param ctx the parse tree
 */
fn enter_booleanValue(&mut self, _ctx: &BooleanValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#booleanValue}.
 * @param ctx the parse tree
 */
fn exit_booleanValue(&mut self, _ctx: &BooleanValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#interval}.
 * @param ctx the parse tree
 */
fn enter_interval(&mut self, _ctx: &IntervalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#interval}.
 * @param ctx the parse tree
 */
fn exit_interval(&mut self, _ctx: &IntervalContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#errorCapturingMultiUnitsInterval}.
 * @param ctx the parse tree
 */
fn enter_errorCapturingMultiUnitsInterval(&mut self, _ctx: &ErrorCapturingMultiUnitsIntervalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#errorCapturingMultiUnitsInterval}.
 * @param ctx the parse tree
 */
fn exit_errorCapturingMultiUnitsInterval(&mut self, _ctx: &ErrorCapturingMultiUnitsIntervalContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#multiUnitsInterval}.
 * @param ctx the parse tree
 */
fn enter_multiUnitsInterval(&mut self, _ctx: &MultiUnitsIntervalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#multiUnitsInterval}.
 * @param ctx the parse tree
 */
fn exit_multiUnitsInterval(&mut self, _ctx: &MultiUnitsIntervalContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#errorCapturingUnitToUnitInterval}.
 * @param ctx the parse tree
 */
fn enter_errorCapturingUnitToUnitInterval(&mut self, _ctx: &ErrorCapturingUnitToUnitIntervalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#errorCapturingUnitToUnitInterval}.
 * @param ctx the parse tree
 */
fn exit_errorCapturingUnitToUnitInterval(&mut self, _ctx: &ErrorCapturingUnitToUnitIntervalContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#unitToUnitInterval}.
 * @param ctx the parse tree
 */
fn enter_unitToUnitInterval(&mut self, _ctx: &UnitToUnitIntervalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#unitToUnitInterval}.
 * @param ctx the parse tree
 */
fn exit_unitToUnitInterval(&mut self, _ctx: &UnitToUnitIntervalContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#intervalValue}.
 * @param ctx the parse tree
 */
fn enter_intervalValue(&mut self, _ctx: &IntervalValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#intervalValue}.
 * @param ctx the parse tree
 */
fn exit_intervalValue(&mut self, _ctx: &IntervalValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#unitInMultiUnits}.
 * @param ctx the parse tree
 */
fn enter_unitInMultiUnits(&mut self, _ctx: &UnitInMultiUnitsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#unitInMultiUnits}.
 * @param ctx the parse tree
 */
fn exit_unitInMultiUnits(&mut self, _ctx: &UnitInMultiUnitsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#unitInUnitToUnit}.
 * @param ctx the parse tree
 */
fn enter_unitInUnitToUnit(&mut self, _ctx: &UnitInUnitToUnitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#unitInUnitToUnit}.
 * @param ctx the parse tree
 */
fn exit_unitInUnitToUnit(&mut self, _ctx: &UnitInUnitToUnitContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#colPosition}.
 * @param ctx the parse tree
 */
fn enter_colPosition(&mut self, _ctx: &ColPositionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#colPosition}.
 * @param ctx the parse tree
 */
fn exit_colPosition(&mut self, _ctx: &ColPositionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#collationSpec}.
 * @param ctx the parse tree
 */
fn enter_collationSpec(&mut self, _ctx: &CollationSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#collationSpec}.
 * @param ctx the parse tree
 */
fn exit_collationSpec(&mut self, _ctx: &CollationSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#collateClause}.
 * @param ctx the parse tree
 */
fn enter_collateClause(&mut self, _ctx: &CollateClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#collateClause}.
 * @param ctx the parse tree
 */
fn exit_collateClause(&mut self, _ctx: &CollateClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#nonTrivialPrimitiveType}.
 * @param ctx the parse tree
 */
fn enter_nonTrivialPrimitiveType(&mut self, _ctx: &NonTrivialPrimitiveTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#nonTrivialPrimitiveType}.
 * @param ctx the parse tree
 */
fn exit_nonTrivialPrimitiveType(&mut self, _ctx: &NonTrivialPrimitiveTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#trivialPrimitiveType}.
 * @param ctx the parse tree
 */
fn enter_trivialPrimitiveType(&mut self, _ctx: &TrivialPrimitiveTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#trivialPrimitiveType}.
 * @param ctx the parse tree
 */
fn exit_trivialPrimitiveType(&mut self, _ctx: &TrivialPrimitiveTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#primitiveType}.
 * @param ctx the parse tree
 */
fn enter_primitiveType(&mut self, _ctx: &PrimitiveTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#primitiveType}.
 * @param ctx the parse tree
 */
fn exit_primitiveType(&mut self, _ctx: &PrimitiveTypeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code complexDataType}
 * labeled alternative in {@link SqlBaseParser#dataType}.
 * @param ctx the parse tree
 */
fn enter_complexDataType(&mut self, _ctx: &ComplexDataTypeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code complexDataType}
 * labeled alternative in {@link SqlBaseParser#dataType}.
 * @param ctx the parse tree
 */
fn exit_complexDataType(&mut self, _ctx: &ComplexDataTypeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code primitiveDataType}
 * labeled alternative in {@link SqlBaseParser#dataType}.
 * @param ctx the parse tree
 */
fn enter_primitiveDataType(&mut self, _ctx: &PrimitiveDataTypeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code primitiveDataType}
 * labeled alternative in {@link SqlBaseParser#dataType}.
 * @param ctx the parse tree
 */
fn exit_primitiveDataType(&mut self, _ctx: &PrimitiveDataTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#qualifiedColTypeWithPositionList}.
 * @param ctx the parse tree
 */
fn enter_qualifiedColTypeWithPositionList(&mut self, _ctx: &QualifiedColTypeWithPositionListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#qualifiedColTypeWithPositionList}.
 * @param ctx the parse tree
 */
fn exit_qualifiedColTypeWithPositionList(&mut self, _ctx: &QualifiedColTypeWithPositionListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#qualifiedColTypeWithPosition}.
 * @param ctx the parse tree
 */
fn enter_qualifiedColTypeWithPosition(&mut self, _ctx: &QualifiedColTypeWithPositionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#qualifiedColTypeWithPosition}.
 * @param ctx the parse tree
 */
fn exit_qualifiedColTypeWithPosition(&mut self, _ctx: &QualifiedColTypeWithPositionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#colDefinitionDescriptorWithPosition}.
 * @param ctx the parse tree
 */
fn enter_colDefinitionDescriptorWithPosition(&mut self, _ctx: &ColDefinitionDescriptorWithPositionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#colDefinitionDescriptorWithPosition}.
 * @param ctx the parse tree
 */
fn exit_colDefinitionDescriptorWithPosition(&mut self, _ctx: &ColDefinitionDescriptorWithPositionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#defaultExpression}.
 * @param ctx the parse tree
 */
fn enter_defaultExpression(&mut self, _ctx: &DefaultExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#defaultExpression}.
 * @param ctx the parse tree
 */
fn exit_defaultExpression(&mut self, _ctx: &DefaultExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#variableDefaultExpression}.
 * @param ctx the parse tree
 */
fn enter_variableDefaultExpression(&mut self, _ctx: &VariableDefaultExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#variableDefaultExpression}.
 * @param ctx the parse tree
 */
fn exit_variableDefaultExpression(&mut self, _ctx: &VariableDefaultExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#colTypeList}.
 * @param ctx the parse tree
 */
fn enter_colTypeList(&mut self, _ctx: &ColTypeListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#colTypeList}.
 * @param ctx the parse tree
 */
fn exit_colTypeList(&mut self, _ctx: &ColTypeListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#colType}.
 * @param ctx the parse tree
 */
fn enter_colType(&mut self, _ctx: &ColTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#colType}.
 * @param ctx the parse tree
 */
fn exit_colType(&mut self, _ctx: &ColTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#tableElementList}.
 * @param ctx the parse tree
 */
fn enter_tableElementList(&mut self, _ctx: &TableElementListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#tableElementList}.
 * @param ctx the parse tree
 */
fn exit_tableElementList(&mut self, _ctx: &TableElementListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#tableElement}.
 * @param ctx the parse tree
 */
fn enter_tableElement(&mut self, _ctx: &TableElementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#tableElement}.
 * @param ctx the parse tree
 */
fn exit_tableElement(&mut self, _ctx: &TableElementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#colDefinitionList}.
 * @param ctx the parse tree
 */
fn enter_colDefinitionList(&mut self, _ctx: &ColDefinitionListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#colDefinitionList}.
 * @param ctx the parse tree
 */
fn exit_colDefinitionList(&mut self, _ctx: &ColDefinitionListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#colDefinition}.
 * @param ctx the parse tree
 */
fn enter_colDefinition(&mut self, _ctx: &ColDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#colDefinition}.
 * @param ctx the parse tree
 */
fn exit_colDefinition(&mut self, _ctx: &ColDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#colDefinitionOption}.
 * @param ctx the parse tree
 */
fn enter_colDefinitionOption(&mut self, _ctx: &ColDefinitionOptionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#colDefinitionOption}.
 * @param ctx the parse tree
 */
fn exit_colDefinitionOption(&mut self, _ctx: &ColDefinitionOptionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code generatedColumn}
 * labeled alternative in {@link SqlBaseParser#generationExpression}.
 * @param ctx the parse tree
 */
fn enter_generatedColumn(&mut self, _ctx: &GeneratedColumnContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code generatedColumn}
 * labeled alternative in {@link SqlBaseParser#generationExpression}.
 * @param ctx the parse tree
 */
fn exit_generatedColumn(&mut self, _ctx: &GeneratedColumnContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code identityColumn}
 * labeled alternative in {@link SqlBaseParser#generationExpression}.
 * @param ctx the parse tree
 */
fn enter_identityColumn(&mut self, _ctx: &IdentityColumnContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code identityColumn}
 * labeled alternative in {@link SqlBaseParser#generationExpression}.
 * @param ctx the parse tree
 */
fn exit_identityColumn(&mut self, _ctx: &IdentityColumnContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#identityColSpec}.
 * @param ctx the parse tree
 */
fn enter_identityColSpec(&mut self, _ctx: &IdentityColSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#identityColSpec}.
 * @param ctx the parse tree
 */
fn exit_identityColSpec(&mut self, _ctx: &IdentityColSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#sequenceGeneratorOption}.
 * @param ctx the parse tree
 */
fn enter_sequenceGeneratorOption(&mut self, _ctx: &SequenceGeneratorOptionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#sequenceGeneratorOption}.
 * @param ctx the parse tree
 */
fn exit_sequenceGeneratorOption(&mut self, _ctx: &SequenceGeneratorOptionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#sequenceGeneratorStartOrStep}.
 * @param ctx the parse tree
 */
fn enter_sequenceGeneratorStartOrStep(&mut self, _ctx: &SequenceGeneratorStartOrStepContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#sequenceGeneratorStartOrStep}.
 * @param ctx the parse tree
 */
fn exit_sequenceGeneratorStartOrStep(&mut self, _ctx: &SequenceGeneratorStartOrStepContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#complexColTypeList}.
 * @param ctx the parse tree
 */
fn enter_complexColTypeList(&mut self, _ctx: &ComplexColTypeListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#complexColTypeList}.
 * @param ctx the parse tree
 */
fn exit_complexColTypeList(&mut self, _ctx: &ComplexColTypeListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#complexColType}.
 * @param ctx the parse tree
 */
fn enter_complexColType(&mut self, _ctx: &ComplexColTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#complexColType}.
 * @param ctx the parse tree
 */
fn exit_complexColType(&mut self, _ctx: &ComplexColTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#codeLiteral}.
 * @param ctx the parse tree
 */
fn enter_codeLiteral(&mut self, _ctx: &CodeLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#codeLiteral}.
 * @param ctx the parse tree
 */
fn exit_codeLiteral(&mut self, _ctx: &CodeLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#routineCharacteristics}.
 * @param ctx the parse tree
 */
fn enter_routineCharacteristics(&mut self, _ctx: &RoutineCharacteristicsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#routineCharacteristics}.
 * @param ctx the parse tree
 */
fn exit_routineCharacteristics(&mut self, _ctx: &RoutineCharacteristicsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#routineLanguage}.
 * @param ctx the parse tree
 */
fn enter_routineLanguage(&mut self, _ctx: &RoutineLanguageContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#routineLanguage}.
 * @param ctx the parse tree
 */
fn exit_routineLanguage(&mut self, _ctx: &RoutineLanguageContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#specificName}.
 * @param ctx the parse tree
 */
fn enter_specificName(&mut self, _ctx: &SpecificNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#specificName}.
 * @param ctx the parse tree
 */
fn exit_specificName(&mut self, _ctx: &SpecificNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#deterministic}.
 * @param ctx the parse tree
 */
fn enter_deterministic(&mut self, _ctx: &DeterministicContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#deterministic}.
 * @param ctx the parse tree
 */
fn exit_deterministic(&mut self, _ctx: &DeterministicContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#sqlDataAccess}.
 * @param ctx the parse tree
 */
fn enter_sqlDataAccess(&mut self, _ctx: &SqlDataAccessContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#sqlDataAccess}.
 * @param ctx the parse tree
 */
fn exit_sqlDataAccess(&mut self, _ctx: &SqlDataAccessContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#nullCall}.
 * @param ctx the parse tree
 */
fn enter_nullCall(&mut self, _ctx: &NullCallContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#nullCall}.
 * @param ctx the parse tree
 */
fn exit_nullCall(&mut self, _ctx: &NullCallContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#rightsClause}.
 * @param ctx the parse tree
 */
fn enter_rightsClause(&mut self, _ctx: &RightsClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#rightsClause}.
 * @param ctx the parse tree
 */
fn exit_rightsClause(&mut self, _ctx: &RightsClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#whenClause}.
 * @param ctx the parse tree
 */
fn enter_whenClause(&mut self, _ctx: &WhenClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#whenClause}.
 * @param ctx the parse tree
 */
fn exit_whenClause(&mut self, _ctx: &WhenClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#windowClause}.
 * @param ctx the parse tree
 */
fn enter_windowClause(&mut self, _ctx: &WindowClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#windowClause}.
 * @param ctx the parse tree
 */
fn exit_windowClause(&mut self, _ctx: &WindowClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#namedWindow}.
 * @param ctx the parse tree
 */
fn enter_namedWindow(&mut self, _ctx: &NamedWindowContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#namedWindow}.
 * @param ctx the parse tree
 */
fn exit_namedWindow(&mut self, _ctx: &NamedWindowContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code windowRef}
 * labeled alternative in {@link SqlBaseParser#windowSpec}.
 * @param ctx the parse tree
 */
fn enter_windowRef(&mut self, _ctx: &WindowRefContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code windowRef}
 * labeled alternative in {@link SqlBaseParser#windowSpec}.
 * @param ctx the parse tree
 */
fn exit_windowRef(&mut self, _ctx: &WindowRefContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code windowDef}
 * labeled alternative in {@link SqlBaseParser#windowSpec}.
 * @param ctx the parse tree
 */
fn enter_windowDef(&mut self, _ctx: &WindowDefContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code windowDef}
 * labeled alternative in {@link SqlBaseParser#windowSpec}.
 * @param ctx the parse tree
 */
fn exit_windowDef(&mut self, _ctx: &WindowDefContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#windowFrame}.
 * @param ctx the parse tree
 */
fn enter_windowFrame(&mut self, _ctx: &WindowFrameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#windowFrame}.
 * @param ctx the parse tree
 */
fn exit_windowFrame(&mut self, _ctx: &WindowFrameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#frameBound}.
 * @param ctx the parse tree
 */
fn enter_frameBound(&mut self, _ctx: &FrameBoundContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#frameBound}.
 * @param ctx the parse tree
 */
fn exit_frameBound(&mut self, _ctx: &FrameBoundContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#qualifiedNameList}.
 * @param ctx the parse tree
 */
fn enter_qualifiedNameList(&mut self, _ctx: &QualifiedNameListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#qualifiedNameList}.
 * @param ctx the parse tree
 */
fn exit_qualifiedNameList(&mut self, _ctx: &QualifiedNameListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#functionName}.
 * @param ctx the parse tree
 */
fn enter_functionName(&mut self, _ctx: &FunctionNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#functionName}.
 * @param ctx the parse tree
 */
fn exit_functionName(&mut self, _ctx: &FunctionNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#qualifiedName}.
 * @param ctx the parse tree
 */
fn enter_qualifiedName(&mut self, _ctx: &QualifiedNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#qualifiedName}.
 * @param ctx the parse tree
 */
fn exit_qualifiedName(&mut self, _ctx: &QualifiedNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#errorCapturingIdentifier}.
 * @param ctx the parse tree
 */
fn enter_errorCapturingIdentifier(&mut self, _ctx: &ErrorCapturingIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#errorCapturingIdentifier}.
 * @param ctx the parse tree
 */
fn exit_errorCapturingIdentifier(&mut self, _ctx: &ErrorCapturingIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code errorIdent}
 * labeled alternative in {@link SqlBaseParser#errorCapturingIdentifierExtra}.
 * @param ctx the parse tree
 */
fn enter_errorIdent(&mut self, _ctx: &ErrorIdentContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code errorIdent}
 * labeled alternative in {@link SqlBaseParser#errorCapturingIdentifierExtra}.
 * @param ctx the parse tree
 */
fn exit_errorIdent(&mut self, _ctx: &ErrorIdentContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code realIdent}
 * labeled alternative in {@link SqlBaseParser#errorCapturingIdentifierExtra}.
 * @param ctx the parse tree
 */
fn enter_realIdent(&mut self, _ctx: &RealIdentContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code realIdent}
 * labeled alternative in {@link SqlBaseParser#errorCapturingIdentifierExtra}.
 * @param ctx the parse tree
 */
fn exit_realIdent(&mut self, _ctx: &RealIdentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#identifier}.
 * @param ctx the parse tree
 */
fn enter_identifier(&mut self, _ctx: &IdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#identifier}.
 * @param ctx the parse tree
 */
fn exit_identifier(&mut self, _ctx: &IdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#simpleIdentifier}.
 * @param ctx the parse tree
 */
fn enter_simpleIdentifier(&mut self, _ctx: &SimpleIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#simpleIdentifier}.
 * @param ctx the parse tree
 */
fn exit_simpleIdentifier(&mut self, _ctx: &SimpleIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code unquotedIdentifier}
 * labeled alternative in {@link SqlBaseParser#strictIdentifier}.
 * @param ctx the parse tree
 */
fn enter_unquotedIdentifier(&mut self, _ctx: &UnquotedIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code unquotedIdentifier}
 * labeled alternative in {@link SqlBaseParser#strictIdentifier}.
 * @param ctx the parse tree
 */
fn exit_unquotedIdentifier(&mut self, _ctx: &UnquotedIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code quotedIdentifierAlternative}
 * labeled alternative in {@link SqlBaseParser#strictIdentifier}.
 * @param ctx the parse tree
 */
fn enter_quotedIdentifierAlternative(&mut self, _ctx: &QuotedIdentifierAlternativeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code quotedIdentifierAlternative}
 * labeled alternative in {@link SqlBaseParser#strictIdentifier}.
 * @param ctx the parse tree
 */
fn exit_quotedIdentifierAlternative(&mut self, _ctx: &QuotedIdentifierAlternativeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code identifierLiteral}
 * labeled alternative in {@link SqlBaseParser#strictIdentifier}.
 * @param ctx the parse tree
 */
fn enter_identifierLiteral(&mut self, _ctx: &IdentifierLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code identifierLiteral}
 * labeled alternative in {@link SqlBaseParser#strictIdentifier}.
 * @param ctx the parse tree
 */
fn exit_identifierLiteral(&mut self, _ctx: &IdentifierLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code simpleUnquotedIdentifier}
 * labeled alternative in {@link SqlBaseParser#simpleStrictIdentifier}.
 * @param ctx the parse tree
 */
fn enter_simpleUnquotedIdentifier(&mut self, _ctx: &SimpleUnquotedIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code simpleUnquotedIdentifier}
 * labeled alternative in {@link SqlBaseParser#simpleStrictIdentifier}.
 * @param ctx the parse tree
 */
fn exit_simpleUnquotedIdentifier(&mut self, _ctx: &SimpleUnquotedIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code simpleQuotedIdentifierAlternative}
 * labeled alternative in {@link SqlBaseParser#simpleStrictIdentifier}.
 * @param ctx the parse tree
 */
fn enter_simpleQuotedIdentifierAlternative(&mut self, _ctx: &SimpleQuotedIdentifierAlternativeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code simpleQuotedIdentifierAlternative}
 * labeled alternative in {@link SqlBaseParser#simpleStrictIdentifier}.
 * @param ctx the parse tree
 */
fn exit_simpleQuotedIdentifierAlternative(&mut self, _ctx: &SimpleQuotedIdentifierAlternativeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#quotedIdentifier}.
 * @param ctx the parse tree
 */
fn enter_quotedIdentifier(&mut self, _ctx: &QuotedIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#quotedIdentifier}.
 * @param ctx the parse tree
 */
fn exit_quotedIdentifier(&mut self, _ctx: &QuotedIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#backQuotedIdentifier}.
 * @param ctx the parse tree
 */
fn enter_backQuotedIdentifier(&mut self, _ctx: &BackQuotedIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#backQuotedIdentifier}.
 * @param ctx the parse tree
 */
fn exit_backQuotedIdentifier(&mut self, _ctx: &BackQuotedIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code exponentLiteral}
 * labeled alternative in {@link SqlBaseParser#number}.
 * @param ctx the parse tree
 */
fn enter_exponentLiteral(&mut self, _ctx: &ExponentLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code exponentLiteral}
 * labeled alternative in {@link SqlBaseParser#number}.
 * @param ctx the parse tree
 */
fn exit_exponentLiteral(&mut self, _ctx: &ExponentLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code decimalLiteral}
 * labeled alternative in {@link SqlBaseParser#number}.
 * @param ctx the parse tree
 */
fn enter_decimalLiteral(&mut self, _ctx: &DecimalLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code decimalLiteral}
 * labeled alternative in {@link SqlBaseParser#number}.
 * @param ctx the parse tree
 */
fn exit_decimalLiteral(&mut self, _ctx: &DecimalLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code legacyDecimalLiteral}
 * labeled alternative in {@link SqlBaseParser#number}.
 * @param ctx the parse tree
 */
fn enter_legacyDecimalLiteral(&mut self, _ctx: &LegacyDecimalLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code legacyDecimalLiteral}
 * labeled alternative in {@link SqlBaseParser#number}.
 * @param ctx the parse tree
 */
fn exit_legacyDecimalLiteral(&mut self, _ctx: &LegacyDecimalLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code integerLiteral}
 * labeled alternative in {@link SqlBaseParser#number}.
 * @param ctx the parse tree
 */
fn enter_integerLiteral(&mut self, _ctx: &IntegerLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code integerLiteral}
 * labeled alternative in {@link SqlBaseParser#number}.
 * @param ctx the parse tree
 */
fn exit_integerLiteral(&mut self, _ctx: &IntegerLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code bigIntLiteral}
 * labeled alternative in {@link SqlBaseParser#number}.
 * @param ctx the parse tree
 */
fn enter_bigIntLiteral(&mut self, _ctx: &BigIntLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code bigIntLiteral}
 * labeled alternative in {@link SqlBaseParser#number}.
 * @param ctx the parse tree
 */
fn exit_bigIntLiteral(&mut self, _ctx: &BigIntLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code smallIntLiteral}
 * labeled alternative in {@link SqlBaseParser#number}.
 * @param ctx the parse tree
 */
fn enter_smallIntLiteral(&mut self, _ctx: &SmallIntLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code smallIntLiteral}
 * labeled alternative in {@link SqlBaseParser#number}.
 * @param ctx the parse tree
 */
fn exit_smallIntLiteral(&mut self, _ctx: &SmallIntLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code tinyIntLiteral}
 * labeled alternative in {@link SqlBaseParser#number}.
 * @param ctx the parse tree
 */
fn enter_tinyIntLiteral(&mut self, _ctx: &TinyIntLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code tinyIntLiteral}
 * labeled alternative in {@link SqlBaseParser#number}.
 * @param ctx the parse tree
 */
fn exit_tinyIntLiteral(&mut self, _ctx: &TinyIntLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code doubleLiteral}
 * labeled alternative in {@link SqlBaseParser#number}.
 * @param ctx the parse tree
 */
fn enter_doubleLiteral(&mut self, _ctx: &DoubleLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code doubleLiteral}
 * labeled alternative in {@link SqlBaseParser#number}.
 * @param ctx the parse tree
 */
fn exit_doubleLiteral(&mut self, _ctx: &DoubleLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code floatLiteral}
 * labeled alternative in {@link SqlBaseParser#number}.
 * @param ctx the parse tree
 */
fn enter_floatLiteral(&mut self, _ctx: &FloatLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code floatLiteral}
 * labeled alternative in {@link SqlBaseParser#number}.
 * @param ctx the parse tree
 */
fn exit_floatLiteral(&mut self, _ctx: &FloatLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code bigDecimalLiteral}
 * labeled alternative in {@link SqlBaseParser#number}.
 * @param ctx the parse tree
 */
fn enter_bigDecimalLiteral(&mut self, _ctx: &BigDecimalLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code bigDecimalLiteral}
 * labeled alternative in {@link SqlBaseParser#number}.
 * @param ctx the parse tree
 */
fn exit_bigDecimalLiteral(&mut self, _ctx: &BigDecimalLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code integerVal}
 * labeled alternative in {@link SqlBaseParser#integerValue}.
 * @param ctx the parse tree
 */
fn enter_integerVal(&mut self, _ctx: &IntegerValContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code integerVal}
 * labeled alternative in {@link SqlBaseParser#integerValue}.
 * @param ctx the parse tree
 */
fn exit_integerVal(&mut self, _ctx: &IntegerValContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code parameterIntegerValue}
 * labeled alternative in {@link SqlBaseParser#integerValue}.
 * @param ctx the parse tree
 */
fn enter_parameterIntegerValue(&mut self, _ctx: &ParameterIntegerValueContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code parameterIntegerValue}
 * labeled alternative in {@link SqlBaseParser#integerValue}.
 * @param ctx the parse tree
 */
fn exit_parameterIntegerValue(&mut self, _ctx: &ParameterIntegerValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#columnConstraintDefinition}.
 * @param ctx the parse tree
 */
fn enter_columnConstraintDefinition(&mut self, _ctx: &ColumnConstraintDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#columnConstraintDefinition}.
 * @param ctx the parse tree
 */
fn exit_columnConstraintDefinition(&mut self, _ctx: &ColumnConstraintDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#columnConstraint}.
 * @param ctx the parse tree
 */
fn enter_columnConstraint(&mut self, _ctx: &ColumnConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#columnConstraint}.
 * @param ctx the parse tree
 */
fn exit_columnConstraint(&mut self, _ctx: &ColumnConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#tableConstraintDefinition}.
 * @param ctx the parse tree
 */
fn enter_tableConstraintDefinition(&mut self, _ctx: &TableConstraintDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#tableConstraintDefinition}.
 * @param ctx the parse tree
 */
fn exit_tableConstraintDefinition(&mut self, _ctx: &TableConstraintDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#tableConstraint}.
 * @param ctx the parse tree
 */
fn enter_tableConstraint(&mut self, _ctx: &TableConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#tableConstraint}.
 * @param ctx the parse tree
 */
fn exit_tableConstraint(&mut self, _ctx: &TableConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#checkConstraint}.
 * @param ctx the parse tree
 */
fn enter_checkConstraint(&mut self, _ctx: &CheckConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#checkConstraint}.
 * @param ctx the parse tree
 */
fn exit_checkConstraint(&mut self, _ctx: &CheckConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#uniqueSpec}.
 * @param ctx the parse tree
 */
fn enter_uniqueSpec(&mut self, _ctx: &UniqueSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#uniqueSpec}.
 * @param ctx the parse tree
 */
fn exit_uniqueSpec(&mut self, _ctx: &UniqueSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#uniqueConstraint}.
 * @param ctx the parse tree
 */
fn enter_uniqueConstraint(&mut self, _ctx: &UniqueConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#uniqueConstraint}.
 * @param ctx the parse tree
 */
fn exit_uniqueConstraint(&mut self, _ctx: &UniqueConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#referenceSpec}.
 * @param ctx the parse tree
 */
fn enter_referenceSpec(&mut self, _ctx: &ReferenceSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#referenceSpec}.
 * @param ctx the parse tree
 */
fn exit_referenceSpec(&mut self, _ctx: &ReferenceSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#foreignKeyConstraint}.
 * @param ctx the parse tree
 */
fn enter_foreignKeyConstraint(&mut self, _ctx: &ForeignKeyConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#foreignKeyConstraint}.
 * @param ctx the parse tree
 */
fn exit_foreignKeyConstraint(&mut self, _ctx: &ForeignKeyConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#constraintCharacteristic}.
 * @param ctx the parse tree
 */
fn enter_constraintCharacteristic(&mut self, _ctx: &ConstraintCharacteristicContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#constraintCharacteristic}.
 * @param ctx the parse tree
 */
fn exit_constraintCharacteristic(&mut self, _ctx: &ConstraintCharacteristicContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#enforcedCharacteristic}.
 * @param ctx the parse tree
 */
fn enter_enforcedCharacteristic(&mut self, _ctx: &EnforcedCharacteristicContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#enforcedCharacteristic}.
 * @param ctx the parse tree
 */
fn exit_enforcedCharacteristic(&mut self, _ctx: &EnforcedCharacteristicContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#relyCharacteristic}.
 * @param ctx the parse tree
 */
fn enter_relyCharacteristic(&mut self, _ctx: &RelyCharacteristicContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#relyCharacteristic}.
 * @param ctx the parse tree
 */
fn exit_relyCharacteristic(&mut self, _ctx: &RelyCharacteristicContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#alterColumnSpecList}.
 * @param ctx the parse tree
 */
fn enter_alterColumnSpecList(&mut self, _ctx: &AlterColumnSpecListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#alterColumnSpecList}.
 * @param ctx the parse tree
 */
fn exit_alterColumnSpecList(&mut self, _ctx: &AlterColumnSpecListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#alterColumnSpec}.
 * @param ctx the parse tree
 */
fn enter_alterColumnSpec(&mut self, _ctx: &AlterColumnSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#alterColumnSpec}.
 * @param ctx the parse tree
 */
fn exit_alterColumnSpec(&mut self, _ctx: &AlterColumnSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#alterColumnAction}.
 * @param ctx the parse tree
 */
fn enter_alterColumnAction(&mut self, _ctx: &AlterColumnActionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#alterColumnAction}.
 * @param ctx the parse tree
 */
fn exit_alterColumnAction(&mut self, _ctx: &AlterColumnActionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code singleStringLiteralValue}
 * labeled alternative in {@link SqlBaseParser#singleStringLitWithoutMarker}.
 * @param ctx the parse tree
 */
fn enter_singleStringLiteralValue(&mut self, _ctx: &SingleStringLiteralValueContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code singleStringLiteralValue}
 * labeled alternative in {@link SqlBaseParser#singleStringLitWithoutMarker}.
 * @param ctx the parse tree
 */
fn exit_singleStringLiteralValue(&mut self, _ctx: &SingleStringLiteralValueContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code singleDoubleQuotedStringLiteralValue}
 * labeled alternative in {@link SqlBaseParser#singleStringLitWithoutMarker}.
 * @param ctx the parse tree
 */
fn enter_singleDoubleQuotedStringLiteralValue(&mut self, _ctx: &SingleDoubleQuotedStringLiteralValueContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code singleDoubleQuotedStringLiteralValue}
 * labeled alternative in {@link SqlBaseParser#singleStringLitWithoutMarker}.
 * @param ctx the parse tree
 */
fn exit_singleDoubleQuotedStringLiteralValue(&mut self, _ctx: &SingleDoubleQuotedStringLiteralValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#singleStringLit}.
 * @param ctx the parse tree
 */
fn enter_singleStringLit(&mut self, _ctx: &SingleStringLitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#singleStringLit}.
 * @param ctx the parse tree
 */
fn exit_singleStringLit(&mut self, _ctx: &SingleStringLitContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code namedParameterMarkerRule}
 * labeled alternative in {@link SqlBaseParser#parameterMarker}.
 * @param ctx the parse tree
 */
fn enter_namedParameterMarkerRule(&mut self, _ctx: &NamedParameterMarkerRuleContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code namedParameterMarkerRule}
 * labeled alternative in {@link SqlBaseParser#parameterMarker}.
 * @param ctx the parse tree
 */
fn exit_namedParameterMarkerRule(&mut self, _ctx: &NamedParameterMarkerRuleContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code positionalParameterMarkerRule}
 * labeled alternative in {@link SqlBaseParser#parameterMarker}.
 * @param ctx the parse tree
 */
fn enter_positionalParameterMarkerRule(&mut self, _ctx: &PositionalParameterMarkerRuleContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code positionalParameterMarkerRule}
 * labeled alternative in {@link SqlBaseParser#parameterMarker}.
 * @param ctx the parse tree
 */
fn exit_positionalParameterMarkerRule(&mut self, _ctx: &PositionalParameterMarkerRuleContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#stringLit}.
 * @param ctx the parse tree
 */
fn enter_stringLit(&mut self, _ctx: &StringLitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#stringLit}.
 * @param ctx the parse tree
 */
fn exit_stringLit(&mut self, _ctx: &StringLitContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#comment}.
 * @param ctx the parse tree
 */
fn enter_comment(&mut self, _ctx: &CommentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#comment}.
 * @param ctx the parse tree
 */
fn exit_comment(&mut self, _ctx: &CommentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#version}.
 * @param ctx the parse tree
 */
fn enter_version(&mut self, _ctx: &VersionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#version}.
 * @param ctx the parse tree
 */
fn exit_version(&mut self, _ctx: &VersionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#operatorPipeRightSide}.
 * @param ctx the parse tree
 */
fn enter_operatorPipeRightSide(&mut self, _ctx: &OperatorPipeRightSideContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#operatorPipeRightSide}.
 * @param ctx the parse tree
 */
fn exit_operatorPipeRightSide(&mut self, _ctx: &OperatorPipeRightSideContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#operatorPipeSetAssignmentSeq}.
 * @param ctx the parse tree
 */
fn enter_operatorPipeSetAssignmentSeq(&mut self, _ctx: &OperatorPipeSetAssignmentSeqContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#operatorPipeSetAssignmentSeq}.
 * @param ctx the parse tree
 */
fn exit_operatorPipeSetAssignmentSeq(&mut self, _ctx: &OperatorPipeSetAssignmentSeqContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#ansiNonReserved}.
 * @param ctx the parse tree
 */
fn enter_ansiNonReserved(&mut self, _ctx: &AnsiNonReservedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#ansiNonReserved}.
 * @param ctx the parse tree
 */
fn exit_ansiNonReserved(&mut self, _ctx: &AnsiNonReservedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#strictNonReserved}.
 * @param ctx the parse tree
 */
fn enter_strictNonReserved(&mut self, _ctx: &StrictNonReservedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#strictNonReserved}.
 * @param ctx the parse tree
 */
fn exit_strictNonReserved(&mut self, _ctx: &StrictNonReservedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SqlBaseParser#nonReserved}.
 * @param ctx the parse tree
 */
fn enter_nonReserved(&mut self, _ctx: &NonReservedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SqlBaseParser#nonReserved}.
 * @param ctx the parse tree
 */
fn exit_nonReserved(&mut self, _ctx: &NonReservedContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : SqlBaseParserListener<'input> }


