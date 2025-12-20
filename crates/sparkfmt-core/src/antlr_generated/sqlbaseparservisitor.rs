#![allow(nonstandard_style)]
// Generated from /home/runner/work/ms-spark-formatter/ms-spark-formatter/grammar/SqlBaseParser.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::sqlbaseparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link SqlBaseParser}.
 */
pub trait SqlBaseParserVisitor<'input>: ParseTreeVisitor<'input,SqlBaseParserContextType>{
	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#compoundOrSingleStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_compoundOrSingleStatement(&mut self, ctx: &CompoundOrSingleStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#singleCompoundStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_singleCompoundStatement(&mut self, ctx: &SingleCompoundStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#beginEndCompoundBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_beginEndCompoundBlock(&mut self, ctx: &BeginEndCompoundBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#compoundBody}.
	 * @param ctx the parse tree
	 */
	fn visit_compoundBody(&mut self, ctx: &CompoundBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#compoundStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_compoundStatement(&mut self, ctx: &CompoundStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code setVariableInsideSqlScript}
	 * labeled alternative in {@link SqlBaseParser#setStatementInsideSqlScript}.
	 * @param ctx the parse tree
	 */
	fn visit_setVariableInsideSqlScript(&mut self, ctx: &SetVariableInsideSqlScriptContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#sqlStateValue}.
	 * @param ctx the parse tree
	 */
	fn visit_sqlStateValue(&mut self, ctx: &SqlStateValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#declareConditionStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_declareConditionStatement(&mut self, ctx: &DeclareConditionStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#conditionValue}.
	 * @param ctx the parse tree
	 */
	fn visit_conditionValue(&mut self, ctx: &ConditionValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#conditionValues}.
	 * @param ctx the parse tree
	 */
	fn visit_conditionValues(&mut self, ctx: &ConditionValuesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#declareHandlerStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_declareHandlerStatement(&mut self, ctx: &DeclareHandlerStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#whileStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_whileStatement(&mut self, ctx: &WhileStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#ifElseStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_ifElseStatement(&mut self, ctx: &IfElseStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#repeatStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_repeatStatement(&mut self, ctx: &RepeatStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#leaveStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_leaveStatement(&mut self, ctx: &LeaveStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#iterateStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_iterateStatement(&mut self, ctx: &IterateStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code searchedCaseStatement}
	 * labeled alternative in {@link SqlBaseParser#caseStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_searchedCaseStatement(&mut self, ctx: &SearchedCaseStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code simpleCaseStatement}
	 * labeled alternative in {@link SqlBaseParser#caseStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_simpleCaseStatement(&mut self, ctx: &SimpleCaseStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#loopStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_loopStatement(&mut self, ctx: &LoopStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#forStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_forStatement(&mut self, ctx: &ForStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#singleStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_singleStatement(&mut self, ctx: &SingleStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#beginLabel}.
	 * @param ctx the parse tree
	 */
	fn visit_beginLabel(&mut self, ctx: &BeginLabelContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#endLabel}.
	 * @param ctx the parse tree
	 */
	fn visit_endLabel(&mut self, ctx: &EndLabelContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#singleExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_singleExpression(&mut self, ctx: &SingleExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#singleTableIdentifier}.
	 * @param ctx the parse tree
	 */
	fn visit_singleTableIdentifier(&mut self, ctx: &SingleTableIdentifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#singleMultipartIdentifier}.
	 * @param ctx the parse tree
	 */
	fn visit_singleMultipartIdentifier(&mut self, ctx: &SingleMultipartIdentifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#singleFunctionIdentifier}.
	 * @param ctx the parse tree
	 */
	fn visit_singleFunctionIdentifier(&mut self, ctx: &SingleFunctionIdentifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#singleDataType}.
	 * @param ctx the parse tree
	 */
	fn visit_singleDataType(&mut self, ctx: &SingleDataTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#singleTableSchema}.
	 * @param ctx the parse tree
	 */
	fn visit_singleTableSchema(&mut self, ctx: &SingleTableSchemaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#singleRoutineParamList}.
	 * @param ctx the parse tree
	 */
	fn visit_singleRoutineParamList(&mut self, ctx: &SingleRoutineParamListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code statementDefault}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_statementDefault(&mut self, ctx: &StatementDefaultContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code visitExecuteImmediate}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_visitExecuteImmediate(&mut self, ctx: &VisitExecuteImmediateContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code dmlStatement}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_dmlStatement(&mut self, ctx: &DmlStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code use}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_use(&mut self, ctx: &UseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code useNamespace}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_useNamespace(&mut self, ctx: &UseNamespaceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code setCatalog}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_setCatalog(&mut self, ctx: &SetCatalogContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code createNamespace}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_createNamespace(&mut self, ctx: &CreateNamespaceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code setNamespaceProperties}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_setNamespaceProperties(&mut self, ctx: &SetNamespacePropertiesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code unsetNamespaceProperties}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_unsetNamespaceProperties(&mut self, ctx: &UnsetNamespacePropertiesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code setNamespaceCollation}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_setNamespaceCollation(&mut self, ctx: &SetNamespaceCollationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code setNamespaceLocation}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_setNamespaceLocation(&mut self, ctx: &SetNamespaceLocationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code dropNamespace}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_dropNamespace(&mut self, ctx: &DropNamespaceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code showNamespaces}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_showNamespaces(&mut self, ctx: &ShowNamespacesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code createTable}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_createTable(&mut self, ctx: &CreateTableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code createTableLike}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_createTableLike(&mut self, ctx: &CreateTableLikeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code replaceTable}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_replaceTable(&mut self, ctx: &ReplaceTableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code analyze}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_analyze(&mut self, ctx: &AnalyzeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code analyzeTables}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_analyzeTables(&mut self, ctx: &AnalyzeTablesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code addTableColumns}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_addTableColumns(&mut self, ctx: &AddTableColumnsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code renameTableColumn}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_renameTableColumn(&mut self, ctx: &RenameTableColumnContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code dropTableColumns}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_dropTableColumns(&mut self, ctx: &DropTableColumnsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code renameTable}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_renameTable(&mut self, ctx: &RenameTableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code setTableProperties}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_setTableProperties(&mut self, ctx: &SetTablePropertiesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code unsetTableProperties}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_unsetTableProperties(&mut self, ctx: &UnsetTablePropertiesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code alterTableAlterColumn}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_alterTableAlterColumn(&mut self, ctx: &AlterTableAlterColumnContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code hiveChangeColumn}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_hiveChangeColumn(&mut self, ctx: &HiveChangeColumnContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code hiveReplaceColumns}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_hiveReplaceColumns(&mut self, ctx: &HiveReplaceColumnsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code setTableSerDe}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_setTableSerDe(&mut self, ctx: &SetTableSerDeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code addTablePartition}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_addTablePartition(&mut self, ctx: &AddTablePartitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code renameTablePartition}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_renameTablePartition(&mut self, ctx: &RenameTablePartitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code dropTablePartitions}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_dropTablePartitions(&mut self, ctx: &DropTablePartitionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code setTableLocation}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_setTableLocation(&mut self, ctx: &SetTableLocationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code recoverPartitions}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_recoverPartitions(&mut self, ctx: &RecoverPartitionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code alterClusterBy}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_alterClusterBy(&mut self, ctx: &AlterClusterByContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code alterTableCollation}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_alterTableCollation(&mut self, ctx: &AlterTableCollationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code addTableConstraint}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_addTableConstraint(&mut self, ctx: &AddTableConstraintContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code dropTableConstraint}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_dropTableConstraint(&mut self, ctx: &DropTableConstraintContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code dropTable}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_dropTable(&mut self, ctx: &DropTableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code dropView}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_dropView(&mut self, ctx: &DropViewContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code createView}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_createView(&mut self, ctx: &CreateViewContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code createMetricView}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_createMetricView(&mut self, ctx: &CreateMetricViewContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code createTempViewUsing}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_createTempViewUsing(&mut self, ctx: &CreateTempViewUsingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code alterViewQuery}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_alterViewQuery(&mut self, ctx: &AlterViewQueryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code alterViewSchemaBinding}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_alterViewSchemaBinding(&mut self, ctx: &AlterViewSchemaBindingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code createFunction}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_createFunction(&mut self, ctx: &CreateFunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code createUserDefinedFunction}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_createUserDefinedFunction(&mut self, ctx: &CreateUserDefinedFunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code dropFunction}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_dropFunction(&mut self, ctx: &DropFunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code createVariable}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_createVariable(&mut self, ctx: &CreateVariableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code dropVariable}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_dropVariable(&mut self, ctx: &DropVariableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code explain}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_explain(&mut self, ctx: &ExplainContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code showTables}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_showTables(&mut self, ctx: &ShowTablesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code showTableExtended}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_showTableExtended(&mut self, ctx: &ShowTableExtendedContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code showTblProperties}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_showTblProperties(&mut self, ctx: &ShowTblPropertiesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code showColumns}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_showColumns(&mut self, ctx: &ShowColumnsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code showViews}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_showViews(&mut self, ctx: &ShowViewsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code showPartitions}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_showPartitions(&mut self, ctx: &ShowPartitionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code showFunctions}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_showFunctions(&mut self, ctx: &ShowFunctionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code showProcedures}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_showProcedures(&mut self, ctx: &ShowProceduresContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code showCreateTable}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_showCreateTable(&mut self, ctx: &ShowCreateTableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code showCurrentNamespace}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_showCurrentNamespace(&mut self, ctx: &ShowCurrentNamespaceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code showCatalogs}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_showCatalogs(&mut self, ctx: &ShowCatalogsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code describeFunction}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_describeFunction(&mut self, ctx: &DescribeFunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code describeProcedure}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_describeProcedure(&mut self, ctx: &DescribeProcedureContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code describeNamespace}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_describeNamespace(&mut self, ctx: &DescribeNamespaceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code describeRelation}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_describeRelation(&mut self, ctx: &DescribeRelationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code describeQuery}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_describeQuery(&mut self, ctx: &DescribeQueryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code commentNamespace}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_commentNamespace(&mut self, ctx: &CommentNamespaceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code commentTable}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_commentTable(&mut self, ctx: &CommentTableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code refreshTable}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_refreshTable(&mut self, ctx: &RefreshTableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code refreshFunction}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_refreshFunction(&mut self, ctx: &RefreshFunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code refreshResource}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_refreshResource(&mut self, ctx: &RefreshResourceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code cacheTable}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_cacheTable(&mut self, ctx: &CacheTableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code uncacheTable}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_uncacheTable(&mut self, ctx: &UncacheTableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code clearCache}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_clearCache(&mut self, ctx: &ClearCacheContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code loadData}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_loadData(&mut self, ctx: &LoadDataContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code truncateTable}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_truncateTable(&mut self, ctx: &TruncateTableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code repairTable}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_repairTable(&mut self, ctx: &RepairTableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code manageResource}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_manageResource(&mut self, ctx: &ManageResourceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code createIndex}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_createIndex(&mut self, ctx: &CreateIndexContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code dropIndex}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_dropIndex(&mut self, ctx: &DropIndexContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code call}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_call(&mut self, ctx: &CallContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code failNativeCommand}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_failNativeCommand(&mut self, ctx: &FailNativeCommandContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code createPipelineDataset}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_createPipelineDataset(&mut self, ctx: &CreatePipelineDatasetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code createPipelineInsertIntoFlow}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_createPipelineInsertIntoFlow(&mut self, ctx: &CreatePipelineInsertIntoFlowContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#materializedView}.
	 * @param ctx the parse tree
	 */
	fn visit_materializedView(&mut self, ctx: &MaterializedViewContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#streamingTable}.
	 * @param ctx the parse tree
	 */
	fn visit_streamingTable(&mut self, ctx: &StreamingTableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#createPipelineDatasetHeader}.
	 * @param ctx the parse tree
	 */
	fn visit_createPipelineDatasetHeader(&mut self, ctx: &CreatePipelineDatasetHeaderContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code streamTableName}
	 * labeled alternative in {@link SqlBaseParser#streamRelationPrimary}.
	 * @param ctx the parse tree
	 */
	fn visit_streamTableName(&mut self, ctx: &StreamTableNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code failSetRole}
	 * labeled alternative in {@link SqlBaseParser#setResetStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_failSetRole(&mut self, ctx: &FailSetRoleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code setTimeZone}
	 * labeled alternative in {@link SqlBaseParser#setResetStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_setTimeZone(&mut self, ctx: &SetTimeZoneContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code setVariable}
	 * labeled alternative in {@link SqlBaseParser#setResetStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_setVariable(&mut self, ctx: &SetVariableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code setQuotedConfiguration}
	 * labeled alternative in {@link SqlBaseParser#setResetStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_setQuotedConfiguration(&mut self, ctx: &SetQuotedConfigurationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code setConfiguration}
	 * labeled alternative in {@link SqlBaseParser#setResetStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_setConfiguration(&mut self, ctx: &SetConfigurationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code resetQuotedConfiguration}
	 * labeled alternative in {@link SqlBaseParser#setResetStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_resetQuotedConfiguration(&mut self, ctx: &ResetQuotedConfigurationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code resetConfiguration}
	 * labeled alternative in {@link SqlBaseParser#setResetStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_resetConfiguration(&mut self, ctx: &ResetConfigurationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#executeImmediate}.
	 * @param ctx the parse tree
	 */
	fn visit_executeImmediate(&mut self, ctx: &ExecuteImmediateContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#executeImmediateUsing}.
	 * @param ctx the parse tree
	 */
	fn visit_executeImmediateUsing(&mut self, ctx: &ExecuteImmediateUsingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#timezone}.
	 * @param ctx the parse tree
	 */
	fn visit_timezone(&mut self, ctx: &TimezoneContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#configKey}.
	 * @param ctx the parse tree
	 */
	fn visit_configKey(&mut self, ctx: &ConfigKeyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#configValue}.
	 * @param ctx the parse tree
	 */
	fn visit_configValue(&mut self, ctx: &ConfigValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#unsupportedHiveNativeCommands}.
	 * @param ctx the parse tree
	 */
	fn visit_unsupportedHiveNativeCommands(&mut self, ctx: &UnsupportedHiveNativeCommandsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#createTableHeader}.
	 * @param ctx the parse tree
	 */
	fn visit_createTableHeader(&mut self, ctx: &CreateTableHeaderContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#replaceTableHeader}.
	 * @param ctx the parse tree
	 */
	fn visit_replaceTableHeader(&mut self, ctx: &ReplaceTableHeaderContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#clusterBySpec}.
	 * @param ctx the parse tree
	 */
	fn visit_clusterBySpec(&mut self, ctx: &ClusterBySpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#bucketSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_bucketSpec(&mut self, ctx: &BucketSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#skewSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_skewSpec(&mut self, ctx: &SkewSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#locationSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_locationSpec(&mut self, ctx: &LocationSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#schemaBinding}.
	 * @param ctx the parse tree
	 */
	fn visit_schemaBinding(&mut self, ctx: &SchemaBindingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#commentSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_commentSpec(&mut self, ctx: &CommentSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#singleQuery}.
	 * @param ctx the parse tree
	 */
	fn visit_singleQuery(&mut self, ctx: &SingleQueryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#query}.
	 * @param ctx the parse tree
	 */
	fn visit_query(&mut self, ctx: &QueryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code insertOverwriteTable}
	 * labeled alternative in {@link SqlBaseParser#insertInto}.
	 * @param ctx the parse tree
	 */
	fn visit_insertOverwriteTable(&mut self, ctx: &InsertOverwriteTableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code insertIntoTable}
	 * labeled alternative in {@link SqlBaseParser#insertInto}.
	 * @param ctx the parse tree
	 */
	fn visit_insertIntoTable(&mut self, ctx: &InsertIntoTableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code insertIntoReplaceWhere}
	 * labeled alternative in {@link SqlBaseParser#insertInto}.
	 * @param ctx the parse tree
	 */
	fn visit_insertIntoReplaceWhere(&mut self, ctx: &InsertIntoReplaceWhereContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code insertOverwriteHiveDir}
	 * labeled alternative in {@link SqlBaseParser#insertInto}.
	 * @param ctx the parse tree
	 */
	fn visit_insertOverwriteHiveDir(&mut self, ctx: &InsertOverwriteHiveDirContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code insertOverwriteDir}
	 * labeled alternative in {@link SqlBaseParser#insertInto}.
	 * @param ctx the parse tree
	 */
	fn visit_insertOverwriteDir(&mut self, ctx: &InsertOverwriteDirContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#partitionSpecLocation}.
	 * @param ctx the parse tree
	 */
	fn visit_partitionSpecLocation(&mut self, ctx: &PartitionSpecLocationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#partitionSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_partitionSpec(&mut self, ctx: &PartitionSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#partitionVal}.
	 * @param ctx the parse tree
	 */
	fn visit_partitionVal(&mut self, ctx: &PartitionValContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#createPipelineFlowHeader}.
	 * @param ctx the parse tree
	 */
	fn visit_createPipelineFlowHeader(&mut self, ctx: &CreatePipelineFlowHeaderContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#namespace}.
	 * @param ctx the parse tree
	 */
	fn visit_namespace(&mut self, ctx: &NamespaceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#namespaces}.
	 * @param ctx the parse tree
	 */
	fn visit_namespaces(&mut self, ctx: &NamespacesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#variable}.
	 * @param ctx the parse tree
	 */
	fn visit_variable(&mut self, ctx: &VariableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#describeFuncName}.
	 * @param ctx the parse tree
	 */
	fn visit_describeFuncName(&mut self, ctx: &DescribeFuncNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#describeColName}.
	 * @param ctx the parse tree
	 */
	fn visit_describeColName(&mut self, ctx: &DescribeColNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#ctes}.
	 * @param ctx the parse tree
	 */
	fn visit_ctes(&mut self, ctx: &CtesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#namedQuery}.
	 * @param ctx the parse tree
	 */
	fn visit_namedQuery(&mut self, ctx: &NamedQueryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#tableProvider}.
	 * @param ctx the parse tree
	 */
	fn visit_tableProvider(&mut self, ctx: &TableProviderContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#createTableClauses}.
	 * @param ctx the parse tree
	 */
	fn visit_createTableClauses(&mut self, ctx: &CreateTableClausesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#propertyList}.
	 * @param ctx the parse tree
	 */
	fn visit_propertyList(&mut self, ctx: &PropertyListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code propertyWithKeyAndEquals}
	 * labeled alternative in {@link SqlBaseParser#property}.
	 * @param ctx the parse tree
	 */
	fn visit_propertyWithKeyAndEquals(&mut self, ctx: &PropertyWithKeyAndEqualsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code propertyWithKeyNoEquals}
	 * labeled alternative in {@link SqlBaseParser#property}.
	 * @param ctx the parse tree
	 */
	fn visit_propertyWithKeyNoEquals(&mut self, ctx: &PropertyWithKeyNoEqualsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#propertyKey}.
	 * @param ctx the parse tree
	 */
	fn visit_propertyKey(&mut self, ctx: &PropertyKeyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#propertyKeyOrStringLit}.
	 * @param ctx the parse tree
	 */
	fn visit_propertyKeyOrStringLit(&mut self, ctx: &PropertyKeyOrStringLitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#propertyKeyOrStringLitNoCoalesce}.
	 * @param ctx the parse tree
	 */
	fn visit_propertyKeyOrStringLitNoCoalesce(&mut self, ctx: &PropertyKeyOrStringLitNoCoalesceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#propertyValue}.
	 * @param ctx the parse tree
	 */
	fn visit_propertyValue(&mut self, ctx: &PropertyValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#expressionPropertyList}.
	 * @param ctx the parse tree
	 */
	fn visit_expressionPropertyList(&mut self, ctx: &ExpressionPropertyListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code expressionPropertyWithKeyAndEquals}
	 * labeled alternative in {@link SqlBaseParser#expressionProperty}.
	 * @param ctx the parse tree
	 */
	fn visit_expressionPropertyWithKeyAndEquals(&mut self, ctx: &ExpressionPropertyWithKeyAndEqualsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code expressionPropertyWithKeyNoEquals}
	 * labeled alternative in {@link SqlBaseParser#expressionProperty}.
	 * @param ctx the parse tree
	 */
	fn visit_expressionPropertyWithKeyNoEquals(&mut self, ctx: &ExpressionPropertyWithKeyNoEqualsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#constantList}.
	 * @param ctx the parse tree
	 */
	fn visit_constantList(&mut self, ctx: &ConstantListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#nestedConstantList}.
	 * @param ctx the parse tree
	 */
	fn visit_nestedConstantList(&mut self, ctx: &NestedConstantListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#createFileFormat}.
	 * @param ctx the parse tree
	 */
	fn visit_createFileFormat(&mut self, ctx: &CreateFileFormatContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code tableFileFormat}
	 * labeled alternative in {@link SqlBaseParser#fileFormat}.
	 * @param ctx the parse tree
	 */
	fn visit_tableFileFormat(&mut self, ctx: &TableFileFormatContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code genericFileFormat}
	 * labeled alternative in {@link SqlBaseParser#fileFormat}.
	 * @param ctx the parse tree
	 */
	fn visit_genericFileFormat(&mut self, ctx: &GenericFileFormatContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#storageHandler}.
	 * @param ctx the parse tree
	 */
	fn visit_storageHandler(&mut self, ctx: &StorageHandlerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#resource}.
	 * @param ctx the parse tree
	 */
	fn visit_resource(&mut self, ctx: &ResourceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code singleInsertQuery}
	 * labeled alternative in {@link SqlBaseParser#dmlStatementNoWith}.
	 * @param ctx the parse tree
	 */
	fn visit_singleInsertQuery(&mut self, ctx: &SingleInsertQueryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code multiInsertQuery}
	 * labeled alternative in {@link SqlBaseParser#dmlStatementNoWith}.
	 * @param ctx the parse tree
	 */
	fn visit_multiInsertQuery(&mut self, ctx: &MultiInsertQueryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code deleteFromTable}
	 * labeled alternative in {@link SqlBaseParser#dmlStatementNoWith}.
	 * @param ctx the parse tree
	 */
	fn visit_deleteFromTable(&mut self, ctx: &DeleteFromTableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code updateTable}
	 * labeled alternative in {@link SqlBaseParser#dmlStatementNoWith}.
	 * @param ctx the parse tree
	 */
	fn visit_updateTable(&mut self, ctx: &UpdateTableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code mergeIntoTable}
	 * labeled alternative in {@link SqlBaseParser#dmlStatementNoWith}.
	 * @param ctx the parse tree
	 */
	fn visit_mergeIntoTable(&mut self, ctx: &MergeIntoTableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#identifierReference}.
	 * @param ctx the parse tree
	 */
	fn visit_identifierReference(&mut self, ctx: &IdentifierReferenceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#catalogIdentifierReference}.
	 * @param ctx the parse tree
	 */
	fn visit_catalogIdentifierReference(&mut self, ctx: &CatalogIdentifierReferenceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#queryOrganization}.
	 * @param ctx the parse tree
	 */
	fn visit_queryOrganization(&mut self, ctx: &QueryOrganizationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#multiInsertQueryBody}.
	 * @param ctx the parse tree
	 */
	fn visit_multiInsertQueryBody(&mut self, ctx: &MultiInsertQueryBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code operatorPipeStatement}
	 * labeled alternative in {@link SqlBaseParser#queryTerm}.
	 * @param ctx the parse tree
	 */
	fn visit_operatorPipeStatement(&mut self, ctx: &OperatorPipeStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code queryTermDefault}
	 * labeled alternative in {@link SqlBaseParser#queryTerm}.
	 * @param ctx the parse tree
	 */
	fn visit_queryTermDefault(&mut self, ctx: &QueryTermDefaultContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code setOperation}
	 * labeled alternative in {@link SqlBaseParser#queryTerm}.
	 * @param ctx the parse tree
	 */
	fn visit_setOperation(&mut self, ctx: &SetOperationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code queryPrimaryDefault}
	 * labeled alternative in {@link SqlBaseParser#queryPrimary}.
	 * @param ctx the parse tree
	 */
	fn visit_queryPrimaryDefault(&mut self, ctx: &QueryPrimaryDefaultContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code fromStmt}
	 * labeled alternative in {@link SqlBaseParser#queryPrimary}.
	 * @param ctx the parse tree
	 */
	fn visit_fromStmt(&mut self, ctx: &FromStmtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code table}
	 * labeled alternative in {@link SqlBaseParser#queryPrimary}.
	 * @param ctx the parse tree
	 */
	fn visit_table(&mut self, ctx: &TableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code inlineTableDefault1}
	 * labeled alternative in {@link SqlBaseParser#queryPrimary}.
	 * @param ctx the parse tree
	 */
	fn visit_inlineTableDefault1(&mut self, ctx: &InlineTableDefault1Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code subquery}
	 * labeled alternative in {@link SqlBaseParser#queryPrimary}.
	 * @param ctx the parse tree
	 */
	fn visit_subquery(&mut self, ctx: &SubqueryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#sortItem}.
	 * @param ctx the parse tree
	 */
	fn visit_sortItem(&mut self, ctx: &SortItemContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#fromStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_fromStatement(&mut self, ctx: &FromStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#fromStatementBody}.
	 * @param ctx the parse tree
	 */
	fn visit_fromStatementBody(&mut self, ctx: &FromStatementBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code transformQuerySpecification}
	 * labeled alternative in {@link SqlBaseParser#querySpecification}.
	 * @param ctx the parse tree
	 */
	fn visit_transformQuerySpecification(&mut self, ctx: &TransformQuerySpecificationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code regularQuerySpecification}
	 * labeled alternative in {@link SqlBaseParser#querySpecification}.
	 * @param ctx the parse tree
	 */
	fn visit_regularQuerySpecification(&mut self, ctx: &RegularQuerySpecificationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#transformClause}.
	 * @param ctx the parse tree
	 */
	fn visit_transformClause(&mut self, ctx: &TransformClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#selectClause}.
	 * @param ctx the parse tree
	 */
	fn visit_selectClause(&mut self, ctx: &SelectClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#setClause}.
	 * @param ctx the parse tree
	 */
	fn visit_setClause(&mut self, ctx: &SetClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#matchedClause}.
	 * @param ctx the parse tree
	 */
	fn visit_matchedClause(&mut self, ctx: &MatchedClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#notMatchedClause}.
	 * @param ctx the parse tree
	 */
	fn visit_notMatchedClause(&mut self, ctx: &NotMatchedClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#notMatchedBySourceClause}.
	 * @param ctx the parse tree
	 */
	fn visit_notMatchedBySourceClause(&mut self, ctx: &NotMatchedBySourceClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#matchedAction}.
	 * @param ctx the parse tree
	 */
	fn visit_matchedAction(&mut self, ctx: &MatchedActionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#notMatchedAction}.
	 * @param ctx the parse tree
	 */
	fn visit_notMatchedAction(&mut self, ctx: &NotMatchedActionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#notMatchedBySourceAction}.
	 * @param ctx the parse tree
	 */
	fn visit_notMatchedBySourceAction(&mut self, ctx: &NotMatchedBySourceActionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#exceptClause}.
	 * @param ctx the parse tree
	 */
	fn visit_exceptClause(&mut self, ctx: &ExceptClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#assignmentList}.
	 * @param ctx the parse tree
	 */
	fn visit_assignmentList(&mut self, ctx: &AssignmentListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#assignment}.
	 * @param ctx the parse tree
	 */
	fn visit_assignment(&mut self, ctx: &AssignmentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#whereClause}.
	 * @param ctx the parse tree
	 */
	fn visit_whereClause(&mut self, ctx: &WhereClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#havingClause}.
	 * @param ctx the parse tree
	 */
	fn visit_havingClause(&mut self, ctx: &HavingClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#hint}.
	 * @param ctx the parse tree
	 */
	fn visit_hint(&mut self, ctx: &HintContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#hintStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_hintStatement(&mut self, ctx: &HintStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#fromClause}.
	 * @param ctx the parse tree
	 */
	fn visit_fromClause(&mut self, ctx: &FromClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#temporalClause}.
	 * @param ctx the parse tree
	 */
	fn visit_temporalClause(&mut self, ctx: &TemporalClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#aggregationClause}.
	 * @param ctx the parse tree
	 */
	fn visit_aggregationClause(&mut self, ctx: &AggregationClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#groupByClause}.
	 * @param ctx the parse tree
	 */
	fn visit_groupByClause(&mut self, ctx: &GroupByClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#groupingAnalytics}.
	 * @param ctx the parse tree
	 */
	fn visit_groupingAnalytics(&mut self, ctx: &GroupingAnalyticsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#groupingElement}.
	 * @param ctx the parse tree
	 */
	fn visit_groupingElement(&mut self, ctx: &GroupingElementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#groupingSet}.
	 * @param ctx the parse tree
	 */
	fn visit_groupingSet(&mut self, ctx: &GroupingSetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#pivotClause}.
	 * @param ctx the parse tree
	 */
	fn visit_pivotClause(&mut self, ctx: &PivotClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#pivotColumn}.
	 * @param ctx the parse tree
	 */
	fn visit_pivotColumn(&mut self, ctx: &PivotColumnContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#pivotValue}.
	 * @param ctx the parse tree
	 */
	fn visit_pivotValue(&mut self, ctx: &PivotValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#unpivotClause}.
	 * @param ctx the parse tree
	 */
	fn visit_unpivotClause(&mut self, ctx: &UnpivotClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#unpivotNullClause}.
	 * @param ctx the parse tree
	 */
	fn visit_unpivotNullClause(&mut self, ctx: &UnpivotNullClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#unpivotOperator}.
	 * @param ctx the parse tree
	 */
	fn visit_unpivotOperator(&mut self, ctx: &UnpivotOperatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#unpivotSingleValueColumnClause}.
	 * @param ctx the parse tree
	 */
	fn visit_unpivotSingleValueColumnClause(&mut self, ctx: &UnpivotSingleValueColumnClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#unpivotMultiValueColumnClause}.
	 * @param ctx the parse tree
	 */
	fn visit_unpivotMultiValueColumnClause(&mut self, ctx: &UnpivotMultiValueColumnClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#unpivotColumnSet}.
	 * @param ctx the parse tree
	 */
	fn visit_unpivotColumnSet(&mut self, ctx: &UnpivotColumnSetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#unpivotValueColumn}.
	 * @param ctx the parse tree
	 */
	fn visit_unpivotValueColumn(&mut self, ctx: &UnpivotValueColumnContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#unpivotNameColumn}.
	 * @param ctx the parse tree
	 */
	fn visit_unpivotNameColumn(&mut self, ctx: &UnpivotNameColumnContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#unpivotColumnAndAlias}.
	 * @param ctx the parse tree
	 */
	fn visit_unpivotColumnAndAlias(&mut self, ctx: &UnpivotColumnAndAliasContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#unpivotColumn}.
	 * @param ctx the parse tree
	 */
	fn visit_unpivotColumn(&mut self, ctx: &UnpivotColumnContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#unpivotAlias}.
	 * @param ctx the parse tree
	 */
	fn visit_unpivotAlias(&mut self, ctx: &UnpivotAliasContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#lateralView}.
	 * @param ctx the parse tree
	 */
	fn visit_lateralView(&mut self, ctx: &LateralViewContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#watermarkClause}.
	 * @param ctx the parse tree
	 */
	fn visit_watermarkClause(&mut self, ctx: &WatermarkClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#setQuantifier}.
	 * @param ctx the parse tree
	 */
	fn visit_setQuantifier(&mut self, ctx: &SetQuantifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#relation}.
	 * @param ctx the parse tree
	 */
	fn visit_relation(&mut self, ctx: &RelationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#relationExtension}.
	 * @param ctx the parse tree
	 */
	fn visit_relationExtension(&mut self, ctx: &RelationExtensionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#joinRelation}.
	 * @param ctx the parse tree
	 */
	fn visit_joinRelation(&mut self, ctx: &JoinRelationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#joinType}.
	 * @param ctx the parse tree
	 */
	fn visit_joinType(&mut self, ctx: &JoinTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#joinCriteria}.
	 * @param ctx the parse tree
	 */
	fn visit_joinCriteria(&mut self, ctx: &JoinCriteriaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#sample}.
	 * @param ctx the parse tree
	 */
	fn visit_sample(&mut self, ctx: &SampleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code sampleByPercentile}
	 * labeled alternative in {@link SqlBaseParser#sampleMethod}.
	 * @param ctx the parse tree
	 */
	fn visit_sampleByPercentile(&mut self, ctx: &SampleByPercentileContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code sampleByRows}
	 * labeled alternative in {@link SqlBaseParser#sampleMethod}.
	 * @param ctx the parse tree
	 */
	fn visit_sampleByRows(&mut self, ctx: &SampleByRowsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code sampleByBucket}
	 * labeled alternative in {@link SqlBaseParser#sampleMethod}.
	 * @param ctx the parse tree
	 */
	fn visit_sampleByBucket(&mut self, ctx: &SampleByBucketContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code sampleByBytes}
	 * labeled alternative in {@link SqlBaseParser#sampleMethod}.
	 * @param ctx the parse tree
	 */
	fn visit_sampleByBytes(&mut self, ctx: &SampleByBytesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#identifierList}.
	 * @param ctx the parse tree
	 */
	fn visit_identifierList(&mut self, ctx: &IdentifierListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#identifierSeq}.
	 * @param ctx the parse tree
	 */
	fn visit_identifierSeq(&mut self, ctx: &IdentifierSeqContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#orderedIdentifierList}.
	 * @param ctx the parse tree
	 */
	fn visit_orderedIdentifierList(&mut self, ctx: &OrderedIdentifierListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#orderedIdentifier}.
	 * @param ctx the parse tree
	 */
	fn visit_orderedIdentifier(&mut self, ctx: &OrderedIdentifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#identifierCommentList}.
	 * @param ctx the parse tree
	 */
	fn visit_identifierCommentList(&mut self, ctx: &IdentifierCommentListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#identifierComment}.
	 * @param ctx the parse tree
	 */
	fn visit_identifierComment(&mut self, ctx: &IdentifierCommentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code streamRelation}
	 * labeled alternative in {@link SqlBaseParser#relationPrimary}.
	 * @param ctx the parse tree
	 */
	fn visit_streamRelation(&mut self, ctx: &StreamRelationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code tableName}
	 * labeled alternative in {@link SqlBaseParser#relationPrimary}.
	 * @param ctx the parse tree
	 */
	fn visit_tableName(&mut self, ctx: &TableNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code aliasedQuery}
	 * labeled alternative in {@link SqlBaseParser#relationPrimary}.
	 * @param ctx the parse tree
	 */
	fn visit_aliasedQuery(&mut self, ctx: &AliasedQueryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code aliasedRelation}
	 * labeled alternative in {@link SqlBaseParser#relationPrimary}.
	 * @param ctx the parse tree
	 */
	fn visit_aliasedRelation(&mut self, ctx: &AliasedRelationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code inlineTableDefault2}
	 * labeled alternative in {@link SqlBaseParser#relationPrimary}.
	 * @param ctx the parse tree
	 */
	fn visit_inlineTableDefault2(&mut self, ctx: &InlineTableDefault2Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code tableValuedFunction}
	 * labeled alternative in {@link SqlBaseParser#relationPrimary}.
	 * @param ctx the parse tree
	 */
	fn visit_tableValuedFunction(&mut self, ctx: &TableValuedFunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#optionsClause}.
	 * @param ctx the parse tree
	 */
	fn visit_optionsClause(&mut self, ctx: &OptionsClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#inlineTable}.
	 * @param ctx the parse tree
	 */
	fn visit_inlineTable(&mut self, ctx: &InlineTableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#functionTableSubqueryArgument}.
	 * @param ctx the parse tree
	 */
	fn visit_functionTableSubqueryArgument(&mut self, ctx: &FunctionTableSubqueryArgumentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#tableArgumentPartitioning}.
	 * @param ctx the parse tree
	 */
	fn visit_tableArgumentPartitioning(&mut self, ctx: &TableArgumentPartitioningContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#functionTableNamedArgumentExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_functionTableNamedArgumentExpression(&mut self, ctx: &FunctionTableNamedArgumentExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#functionTableReferenceArgument}.
	 * @param ctx the parse tree
	 */
	fn visit_functionTableReferenceArgument(&mut self, ctx: &FunctionTableReferenceArgumentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#functionTableArgument}.
	 * @param ctx the parse tree
	 */
	fn visit_functionTableArgument(&mut self, ctx: &FunctionTableArgumentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#functionTable}.
	 * @param ctx the parse tree
	 */
	fn visit_functionTable(&mut self, ctx: &FunctionTableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#tableAlias}.
	 * @param ctx the parse tree
	 */
	fn visit_tableAlias(&mut self, ctx: &TableAliasContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code rowFormatSerde}
	 * labeled alternative in {@link SqlBaseParser#rowFormat}.
	 * @param ctx the parse tree
	 */
	fn visit_rowFormatSerde(&mut self, ctx: &RowFormatSerdeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code rowFormatDelimited}
	 * labeled alternative in {@link SqlBaseParser#rowFormat}.
	 * @param ctx the parse tree
	 */
	fn visit_rowFormatDelimited(&mut self, ctx: &RowFormatDelimitedContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#multipartIdentifierList}.
	 * @param ctx the parse tree
	 */
	fn visit_multipartIdentifierList(&mut self, ctx: &MultipartIdentifierListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#multipartIdentifier}.
	 * @param ctx the parse tree
	 */
	fn visit_multipartIdentifier(&mut self, ctx: &MultipartIdentifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#multipartIdentifierPropertyList}.
	 * @param ctx the parse tree
	 */
	fn visit_multipartIdentifierPropertyList(&mut self, ctx: &MultipartIdentifierPropertyListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#multipartIdentifierProperty}.
	 * @param ctx the parse tree
	 */
	fn visit_multipartIdentifierProperty(&mut self, ctx: &MultipartIdentifierPropertyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#tableIdentifier}.
	 * @param ctx the parse tree
	 */
	fn visit_tableIdentifier(&mut self, ctx: &TableIdentifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#functionIdentifier}.
	 * @param ctx the parse tree
	 */
	fn visit_functionIdentifier(&mut self, ctx: &FunctionIdentifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#namedExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_namedExpression(&mut self, ctx: &NamedExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#namedExpressionSeq}.
	 * @param ctx the parse tree
	 */
	fn visit_namedExpressionSeq(&mut self, ctx: &NamedExpressionSeqContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#partitionFieldList}.
	 * @param ctx the parse tree
	 */
	fn visit_partitionFieldList(&mut self, ctx: &PartitionFieldListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code partitionTransform}
	 * labeled alternative in {@link SqlBaseParser#partitionField}.
	 * @param ctx the parse tree
	 */
	fn visit_partitionTransform(&mut self, ctx: &PartitionTransformContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code partitionColumn}
	 * labeled alternative in {@link SqlBaseParser#partitionField}.
	 * @param ctx the parse tree
	 */
	fn visit_partitionColumn(&mut self, ctx: &PartitionColumnContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code identityTransform}
	 * labeled alternative in {@link SqlBaseParser#transform}.
	 * @param ctx the parse tree
	 */
	fn visit_identityTransform(&mut self, ctx: &IdentityTransformContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code applyTransform}
	 * labeled alternative in {@link SqlBaseParser#transform}.
	 * @param ctx the parse tree
	 */
	fn visit_applyTransform(&mut self, ctx: &ApplyTransformContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#transformArgument}.
	 * @param ctx the parse tree
	 */
	fn visit_transformArgument(&mut self, ctx: &TransformArgumentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#namedArgumentExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_namedArgumentExpression(&mut self, ctx: &NamedArgumentExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#functionArgument}.
	 * @param ctx the parse tree
	 */
	fn visit_functionArgument(&mut self, ctx: &FunctionArgumentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#expressionSeq}.
	 * @param ctx the parse tree
	 */
	fn visit_expressionSeq(&mut self, ctx: &ExpressionSeqContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code logicalNot}
	 * labeled alternative in {@link SqlBaseParser#booleanExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_logicalNot(&mut self, ctx: &LogicalNotContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code predicated}
	 * labeled alternative in {@link SqlBaseParser#booleanExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_predicated(&mut self, ctx: &PredicatedContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code exists}
	 * labeled alternative in {@link SqlBaseParser#booleanExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_exists(&mut self, ctx: &ExistsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code logicalBinary}
	 * labeled alternative in {@link SqlBaseParser#booleanExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_logicalBinary(&mut self, ctx: &LogicalBinaryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#predicate}.
	 * @param ctx the parse tree
	 */
	fn visit_predicate(&mut self, ctx: &PredicateContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#errorCapturingNot}.
	 * @param ctx the parse tree
	 */
	fn visit_errorCapturingNot(&mut self, ctx: &ErrorCapturingNotContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code valueExpressionDefault}
	 * labeled alternative in {@link SqlBaseParser#valueExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_valueExpressionDefault(&mut self, ctx: &ValueExpressionDefaultContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code comparison}
	 * labeled alternative in {@link SqlBaseParser#valueExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_comparison(&mut self, ctx: &ComparisonContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code shiftExpression}
	 * labeled alternative in {@link SqlBaseParser#valueExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_shiftExpression(&mut self, ctx: &ShiftExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code arithmeticBinary}
	 * labeled alternative in {@link SqlBaseParser#valueExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_arithmeticBinary(&mut self, ctx: &ArithmeticBinaryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code arithmeticUnary}
	 * labeled alternative in {@link SqlBaseParser#valueExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_arithmeticUnary(&mut self, ctx: &ArithmeticUnaryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#shiftOperator}.
	 * @param ctx the parse tree
	 */
	fn visit_shiftOperator(&mut self, ctx: &ShiftOperatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#datetimeUnit}.
	 * @param ctx the parse tree
	 */
	fn visit_datetimeUnit(&mut self, ctx: &DatetimeUnitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code struct}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_struct(&mut self, ctx: &StructContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code dereference}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_dereference(&mut self, ctx: &DereferenceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code castByColon}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_castByColon(&mut self, ctx: &CastByColonContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code timestampadd}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_timestampadd(&mut self, ctx: &TimestampaddContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code substring}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_substring(&mut self, ctx: &SubstringContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code cast}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_cast(&mut self, ctx: &CastContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code lambda}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_lambda(&mut self, ctx: &LambdaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code parenthesizedExpression}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_parenthesizedExpression(&mut self, ctx: &ParenthesizedExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code any_value}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_any_value(&mut self, ctx: &Any_valueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code trim}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_trim(&mut self, ctx: &TrimContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code semiStructuredExtract}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_semiStructuredExtract(&mut self, ctx: &SemiStructuredExtractContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code simpleCase}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_simpleCase(&mut self, ctx: &SimpleCaseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code currentLike}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_currentLike(&mut self, ctx: &CurrentLikeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code columnReference}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_columnReference(&mut self, ctx: &ColumnReferenceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code rowConstructor}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_rowConstructor(&mut self, ctx: &RowConstructorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code last}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_last(&mut self, ctx: &LastContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code star}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_star(&mut self, ctx: &StarContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code overlay}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_overlay(&mut self, ctx: &OverlayContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code subscript}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_subscript(&mut self, ctx: &SubscriptContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code timestampdiff}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_timestampdiff(&mut self, ctx: &TimestampdiffContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code subqueryExpression}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_subqueryExpression(&mut self, ctx: &SubqueryExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code collate}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_collate(&mut self, ctx: &CollateContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code constantDefault}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_constantDefault(&mut self, ctx: &ConstantDefaultContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code extract}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_extract(&mut self, ctx: &ExtractContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code functionCall}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_functionCall(&mut self, ctx: &FunctionCallContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code searchedCase}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_searchedCase(&mut self, ctx: &SearchedCaseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code position}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_position(&mut self, ctx: &PositionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code first}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_first(&mut self, ctx: &FirstContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#semiStructuredExtractionPath}.
	 * @param ctx the parse tree
	 */
	fn visit_semiStructuredExtractionPath(&mut self, ctx: &SemiStructuredExtractionPathContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#jsonPathIdentifier}.
	 * @param ctx the parse tree
	 */
	fn visit_jsonPathIdentifier(&mut self, ctx: &JsonPathIdentifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#jsonPathBracketedIdentifier}.
	 * @param ctx the parse tree
	 */
	fn visit_jsonPathBracketedIdentifier(&mut self, ctx: &JsonPathBracketedIdentifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#jsonPathFirstPart}.
	 * @param ctx the parse tree
	 */
	fn visit_jsonPathFirstPart(&mut self, ctx: &JsonPathFirstPartContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#jsonPathParts}.
	 * @param ctx the parse tree
	 */
	fn visit_jsonPathParts(&mut self, ctx: &JsonPathPartsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#literalType}.
	 * @param ctx the parse tree
	 */
	fn visit_literalType(&mut self, ctx: &LiteralTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code nullLiteral}
	 * labeled alternative in {@link SqlBaseParser#constant}.
	 * @param ctx the parse tree
	 */
	fn visit_nullLiteral(&mut self, ctx: &NullLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code posParameterLiteral}
	 * labeled alternative in {@link SqlBaseParser#constant}.
	 * @param ctx the parse tree
	 */
	fn visit_posParameterLiteral(&mut self, ctx: &PosParameterLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code namedParameterLiteral}
	 * labeled alternative in {@link SqlBaseParser#constant}.
	 * @param ctx the parse tree
	 */
	fn visit_namedParameterLiteral(&mut self, ctx: &NamedParameterLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code intervalLiteral}
	 * labeled alternative in {@link SqlBaseParser#constant}.
	 * @param ctx the parse tree
	 */
	fn visit_intervalLiteral(&mut self, ctx: &IntervalLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code typeConstructor}
	 * labeled alternative in {@link SqlBaseParser#constant}.
	 * @param ctx the parse tree
	 */
	fn visit_typeConstructor(&mut self, ctx: &TypeConstructorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code numericLiteral}
	 * labeled alternative in {@link SqlBaseParser#constant}.
	 * @param ctx the parse tree
	 */
	fn visit_numericLiteral(&mut self, ctx: &NumericLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code booleanLiteral}
	 * labeled alternative in {@link SqlBaseParser#constant}.
	 * @param ctx the parse tree
	 */
	fn visit_booleanLiteral(&mut self, ctx: &BooleanLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code stringLiteral}
	 * labeled alternative in {@link SqlBaseParser#constant}.
	 * @param ctx the parse tree
	 */
	fn visit_stringLiteral(&mut self, ctx: &StringLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#namedParameterMarker}.
	 * @param ctx the parse tree
	 */
	fn visit_namedParameterMarker(&mut self, ctx: &NamedParameterMarkerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#comparisonOperator}.
	 * @param ctx the parse tree
	 */
	fn visit_comparisonOperator(&mut self, ctx: &ComparisonOperatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#arithmeticOperator}.
	 * @param ctx the parse tree
	 */
	fn visit_arithmeticOperator(&mut self, ctx: &ArithmeticOperatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#predicateOperator}.
	 * @param ctx the parse tree
	 */
	fn visit_predicateOperator(&mut self, ctx: &PredicateOperatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#booleanValue}.
	 * @param ctx the parse tree
	 */
	fn visit_booleanValue(&mut self, ctx: &BooleanValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#interval}.
	 * @param ctx the parse tree
	 */
	fn visit_interval(&mut self, ctx: &IntervalContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#errorCapturingMultiUnitsInterval}.
	 * @param ctx the parse tree
	 */
	fn visit_errorCapturingMultiUnitsInterval(&mut self, ctx: &ErrorCapturingMultiUnitsIntervalContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#multiUnitsInterval}.
	 * @param ctx the parse tree
	 */
	fn visit_multiUnitsInterval(&mut self, ctx: &MultiUnitsIntervalContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#errorCapturingUnitToUnitInterval}.
	 * @param ctx the parse tree
	 */
	fn visit_errorCapturingUnitToUnitInterval(&mut self, ctx: &ErrorCapturingUnitToUnitIntervalContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#unitToUnitInterval}.
	 * @param ctx the parse tree
	 */
	fn visit_unitToUnitInterval(&mut self, ctx: &UnitToUnitIntervalContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#intervalValue}.
	 * @param ctx the parse tree
	 */
	fn visit_intervalValue(&mut self, ctx: &IntervalValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#unitInMultiUnits}.
	 * @param ctx the parse tree
	 */
	fn visit_unitInMultiUnits(&mut self, ctx: &UnitInMultiUnitsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#unitInUnitToUnit}.
	 * @param ctx the parse tree
	 */
	fn visit_unitInUnitToUnit(&mut self, ctx: &UnitInUnitToUnitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#colPosition}.
	 * @param ctx the parse tree
	 */
	fn visit_colPosition(&mut self, ctx: &ColPositionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#collationSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_collationSpec(&mut self, ctx: &CollationSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#collateClause}.
	 * @param ctx the parse tree
	 */
	fn visit_collateClause(&mut self, ctx: &CollateClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#nonTrivialPrimitiveType}.
	 * @param ctx the parse tree
	 */
	fn visit_nonTrivialPrimitiveType(&mut self, ctx: &NonTrivialPrimitiveTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#trivialPrimitiveType}.
	 * @param ctx the parse tree
	 */
	fn visit_trivialPrimitiveType(&mut self, ctx: &TrivialPrimitiveTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#primitiveType}.
	 * @param ctx the parse tree
	 */
	fn visit_primitiveType(&mut self, ctx: &PrimitiveTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code complexDataType}
	 * labeled alternative in {@link SqlBaseParser#dataType}.
	 * @param ctx the parse tree
	 */
	fn visit_complexDataType(&mut self, ctx: &ComplexDataTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code primitiveDataType}
	 * labeled alternative in {@link SqlBaseParser#dataType}.
	 * @param ctx the parse tree
	 */
	fn visit_primitiveDataType(&mut self, ctx: &PrimitiveDataTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#qualifiedColTypeWithPositionList}.
	 * @param ctx the parse tree
	 */
	fn visit_qualifiedColTypeWithPositionList(&mut self, ctx: &QualifiedColTypeWithPositionListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#qualifiedColTypeWithPosition}.
	 * @param ctx the parse tree
	 */
	fn visit_qualifiedColTypeWithPosition(&mut self, ctx: &QualifiedColTypeWithPositionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#colDefinitionDescriptorWithPosition}.
	 * @param ctx the parse tree
	 */
	fn visit_colDefinitionDescriptorWithPosition(&mut self, ctx: &ColDefinitionDescriptorWithPositionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#defaultExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_defaultExpression(&mut self, ctx: &DefaultExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#variableDefaultExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_variableDefaultExpression(&mut self, ctx: &VariableDefaultExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#colTypeList}.
	 * @param ctx the parse tree
	 */
	fn visit_colTypeList(&mut self, ctx: &ColTypeListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#colType}.
	 * @param ctx the parse tree
	 */
	fn visit_colType(&mut self, ctx: &ColTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#tableElementList}.
	 * @param ctx the parse tree
	 */
	fn visit_tableElementList(&mut self, ctx: &TableElementListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#tableElement}.
	 * @param ctx the parse tree
	 */
	fn visit_tableElement(&mut self, ctx: &TableElementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#colDefinitionList}.
	 * @param ctx the parse tree
	 */
	fn visit_colDefinitionList(&mut self, ctx: &ColDefinitionListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#colDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_colDefinition(&mut self, ctx: &ColDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#colDefinitionOption}.
	 * @param ctx the parse tree
	 */
	fn visit_colDefinitionOption(&mut self, ctx: &ColDefinitionOptionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code generatedColumn}
	 * labeled alternative in {@link SqlBaseParser#generationExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_generatedColumn(&mut self, ctx: &GeneratedColumnContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code identityColumn}
	 * labeled alternative in {@link SqlBaseParser#generationExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_identityColumn(&mut self, ctx: &IdentityColumnContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#identityColSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_identityColSpec(&mut self, ctx: &IdentityColSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#sequenceGeneratorOption}.
	 * @param ctx the parse tree
	 */
	fn visit_sequenceGeneratorOption(&mut self, ctx: &SequenceGeneratorOptionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#sequenceGeneratorStartOrStep}.
	 * @param ctx the parse tree
	 */
	fn visit_sequenceGeneratorStartOrStep(&mut self, ctx: &SequenceGeneratorStartOrStepContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#complexColTypeList}.
	 * @param ctx the parse tree
	 */
	fn visit_complexColTypeList(&mut self, ctx: &ComplexColTypeListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#complexColType}.
	 * @param ctx the parse tree
	 */
	fn visit_complexColType(&mut self, ctx: &ComplexColTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#codeLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_codeLiteral(&mut self, ctx: &CodeLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#routineCharacteristics}.
	 * @param ctx the parse tree
	 */
	fn visit_routineCharacteristics(&mut self, ctx: &RoutineCharacteristicsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#routineLanguage}.
	 * @param ctx the parse tree
	 */
	fn visit_routineLanguage(&mut self, ctx: &RoutineLanguageContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#specificName}.
	 * @param ctx the parse tree
	 */
	fn visit_specificName(&mut self, ctx: &SpecificNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#deterministic}.
	 * @param ctx the parse tree
	 */
	fn visit_deterministic(&mut self, ctx: &DeterministicContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#sqlDataAccess}.
	 * @param ctx the parse tree
	 */
	fn visit_sqlDataAccess(&mut self, ctx: &SqlDataAccessContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#nullCall}.
	 * @param ctx the parse tree
	 */
	fn visit_nullCall(&mut self, ctx: &NullCallContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#rightsClause}.
	 * @param ctx the parse tree
	 */
	fn visit_rightsClause(&mut self, ctx: &RightsClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#whenClause}.
	 * @param ctx the parse tree
	 */
	fn visit_whenClause(&mut self, ctx: &WhenClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#windowClause}.
	 * @param ctx the parse tree
	 */
	fn visit_windowClause(&mut self, ctx: &WindowClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#namedWindow}.
	 * @param ctx the parse tree
	 */
	fn visit_namedWindow(&mut self, ctx: &NamedWindowContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code windowRef}
	 * labeled alternative in {@link SqlBaseParser#windowSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_windowRef(&mut self, ctx: &WindowRefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code windowDef}
	 * labeled alternative in {@link SqlBaseParser#windowSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_windowDef(&mut self, ctx: &WindowDefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#windowFrame}.
	 * @param ctx the parse tree
	 */
	fn visit_windowFrame(&mut self, ctx: &WindowFrameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#frameBound}.
	 * @param ctx the parse tree
	 */
	fn visit_frameBound(&mut self, ctx: &FrameBoundContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#qualifiedNameList}.
	 * @param ctx the parse tree
	 */
	fn visit_qualifiedNameList(&mut self, ctx: &QualifiedNameListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#functionName}.
	 * @param ctx the parse tree
	 */
	fn visit_functionName(&mut self, ctx: &FunctionNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#qualifiedName}.
	 * @param ctx the parse tree
	 */
	fn visit_qualifiedName(&mut self, ctx: &QualifiedNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#errorCapturingIdentifier}.
	 * @param ctx the parse tree
	 */
	fn visit_errorCapturingIdentifier(&mut self, ctx: &ErrorCapturingIdentifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code errorIdent}
	 * labeled alternative in {@link SqlBaseParser#errorCapturingIdentifierExtra}.
	 * @param ctx the parse tree
	 */
	fn visit_errorIdent(&mut self, ctx: &ErrorIdentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code realIdent}
	 * labeled alternative in {@link SqlBaseParser#errorCapturingIdentifierExtra}.
	 * @param ctx the parse tree
	 */
	fn visit_realIdent(&mut self, ctx: &RealIdentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#identifier}.
	 * @param ctx the parse tree
	 */
	fn visit_identifier(&mut self, ctx: &IdentifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#simpleIdentifier}.
	 * @param ctx the parse tree
	 */
	fn visit_simpleIdentifier(&mut self, ctx: &SimpleIdentifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code unquotedIdentifier}
	 * labeled alternative in {@link SqlBaseParser#strictIdentifier}.
	 * @param ctx the parse tree
	 */
	fn visit_unquotedIdentifier(&mut self, ctx: &UnquotedIdentifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code quotedIdentifierAlternative}
	 * labeled alternative in {@link SqlBaseParser#strictIdentifier}.
	 * @param ctx the parse tree
	 */
	fn visit_quotedIdentifierAlternative(&mut self, ctx: &QuotedIdentifierAlternativeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code identifierLiteral}
	 * labeled alternative in {@link SqlBaseParser#strictIdentifier}.
	 * @param ctx the parse tree
	 */
	fn visit_identifierLiteral(&mut self, ctx: &IdentifierLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code simpleUnquotedIdentifier}
	 * labeled alternative in {@link SqlBaseParser#simpleStrictIdentifier}.
	 * @param ctx the parse tree
	 */
	fn visit_simpleUnquotedIdentifier(&mut self, ctx: &SimpleUnquotedIdentifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code simpleQuotedIdentifierAlternative}
	 * labeled alternative in {@link SqlBaseParser#simpleStrictIdentifier}.
	 * @param ctx the parse tree
	 */
	fn visit_simpleQuotedIdentifierAlternative(&mut self, ctx: &SimpleQuotedIdentifierAlternativeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#quotedIdentifier}.
	 * @param ctx the parse tree
	 */
	fn visit_quotedIdentifier(&mut self, ctx: &QuotedIdentifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#backQuotedIdentifier}.
	 * @param ctx the parse tree
	 */
	fn visit_backQuotedIdentifier(&mut self, ctx: &BackQuotedIdentifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code exponentLiteral}
	 * labeled alternative in {@link SqlBaseParser#number}.
	 * @param ctx the parse tree
	 */
	fn visit_exponentLiteral(&mut self, ctx: &ExponentLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code decimalLiteral}
	 * labeled alternative in {@link SqlBaseParser#number}.
	 * @param ctx the parse tree
	 */
	fn visit_decimalLiteral(&mut self, ctx: &DecimalLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code legacyDecimalLiteral}
	 * labeled alternative in {@link SqlBaseParser#number}.
	 * @param ctx the parse tree
	 */
	fn visit_legacyDecimalLiteral(&mut self, ctx: &LegacyDecimalLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code integerLiteral}
	 * labeled alternative in {@link SqlBaseParser#number}.
	 * @param ctx the parse tree
	 */
	fn visit_integerLiteral(&mut self, ctx: &IntegerLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code bigIntLiteral}
	 * labeled alternative in {@link SqlBaseParser#number}.
	 * @param ctx the parse tree
	 */
	fn visit_bigIntLiteral(&mut self, ctx: &BigIntLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code smallIntLiteral}
	 * labeled alternative in {@link SqlBaseParser#number}.
	 * @param ctx the parse tree
	 */
	fn visit_smallIntLiteral(&mut self, ctx: &SmallIntLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code tinyIntLiteral}
	 * labeled alternative in {@link SqlBaseParser#number}.
	 * @param ctx the parse tree
	 */
	fn visit_tinyIntLiteral(&mut self, ctx: &TinyIntLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code doubleLiteral}
	 * labeled alternative in {@link SqlBaseParser#number}.
	 * @param ctx the parse tree
	 */
	fn visit_doubleLiteral(&mut self, ctx: &DoubleLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code floatLiteral}
	 * labeled alternative in {@link SqlBaseParser#number}.
	 * @param ctx the parse tree
	 */
	fn visit_floatLiteral(&mut self, ctx: &FloatLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code bigDecimalLiteral}
	 * labeled alternative in {@link SqlBaseParser#number}.
	 * @param ctx the parse tree
	 */
	fn visit_bigDecimalLiteral(&mut self, ctx: &BigDecimalLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code integerVal}
	 * labeled alternative in {@link SqlBaseParser#integerValue}.
	 * @param ctx the parse tree
	 */
	fn visit_integerVal(&mut self, ctx: &IntegerValContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code parameterIntegerValue}
	 * labeled alternative in {@link SqlBaseParser#integerValue}.
	 * @param ctx the parse tree
	 */
	fn visit_parameterIntegerValue(&mut self, ctx: &ParameterIntegerValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#columnConstraintDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_columnConstraintDefinition(&mut self, ctx: &ColumnConstraintDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#columnConstraint}.
	 * @param ctx the parse tree
	 */
	fn visit_columnConstraint(&mut self, ctx: &ColumnConstraintContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#tableConstraintDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_tableConstraintDefinition(&mut self, ctx: &TableConstraintDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#tableConstraint}.
	 * @param ctx the parse tree
	 */
	fn visit_tableConstraint(&mut self, ctx: &TableConstraintContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#checkConstraint}.
	 * @param ctx the parse tree
	 */
	fn visit_checkConstraint(&mut self, ctx: &CheckConstraintContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#uniqueSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_uniqueSpec(&mut self, ctx: &UniqueSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#uniqueConstraint}.
	 * @param ctx the parse tree
	 */
	fn visit_uniqueConstraint(&mut self, ctx: &UniqueConstraintContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#referenceSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_referenceSpec(&mut self, ctx: &ReferenceSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#foreignKeyConstraint}.
	 * @param ctx the parse tree
	 */
	fn visit_foreignKeyConstraint(&mut self, ctx: &ForeignKeyConstraintContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#constraintCharacteristic}.
	 * @param ctx the parse tree
	 */
	fn visit_constraintCharacteristic(&mut self, ctx: &ConstraintCharacteristicContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#enforcedCharacteristic}.
	 * @param ctx the parse tree
	 */
	fn visit_enforcedCharacteristic(&mut self, ctx: &EnforcedCharacteristicContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#relyCharacteristic}.
	 * @param ctx the parse tree
	 */
	fn visit_relyCharacteristic(&mut self, ctx: &RelyCharacteristicContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#alterColumnSpecList}.
	 * @param ctx the parse tree
	 */
	fn visit_alterColumnSpecList(&mut self, ctx: &AlterColumnSpecListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#alterColumnSpec}.
	 * @param ctx the parse tree
	 */
	fn visit_alterColumnSpec(&mut self, ctx: &AlterColumnSpecContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#alterColumnAction}.
	 * @param ctx the parse tree
	 */
	fn visit_alterColumnAction(&mut self, ctx: &AlterColumnActionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code singleStringLiteralValue}
	 * labeled alternative in {@link SqlBaseParser#singleStringLitWithoutMarker}.
	 * @param ctx the parse tree
	 */
	fn visit_singleStringLiteralValue(&mut self, ctx: &SingleStringLiteralValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code singleDoubleQuotedStringLiteralValue}
	 * labeled alternative in {@link SqlBaseParser#singleStringLitWithoutMarker}.
	 * @param ctx the parse tree
	 */
	fn visit_singleDoubleQuotedStringLiteralValue(&mut self, ctx: &SingleDoubleQuotedStringLiteralValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#singleStringLit}.
	 * @param ctx the parse tree
	 */
	fn visit_singleStringLit(&mut self, ctx: &SingleStringLitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code namedParameterMarkerRule}
	 * labeled alternative in {@link SqlBaseParser#parameterMarker}.
	 * @param ctx the parse tree
	 */
	fn visit_namedParameterMarkerRule(&mut self, ctx: &NamedParameterMarkerRuleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code positionalParameterMarkerRule}
	 * labeled alternative in {@link SqlBaseParser#parameterMarker}.
	 * @param ctx the parse tree
	 */
	fn visit_positionalParameterMarkerRule(&mut self, ctx: &PositionalParameterMarkerRuleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#stringLit}.
	 * @param ctx the parse tree
	 */
	fn visit_stringLit(&mut self, ctx: &StringLitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#comment}.
	 * @param ctx the parse tree
	 */
	fn visit_comment(&mut self, ctx: &CommentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#version}.
	 * @param ctx the parse tree
	 */
	fn visit_version(&mut self, ctx: &VersionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#operatorPipeRightSide}.
	 * @param ctx the parse tree
	 */
	fn visit_operatorPipeRightSide(&mut self, ctx: &OperatorPipeRightSideContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#operatorPipeSetAssignmentSeq}.
	 * @param ctx the parse tree
	 */
	fn visit_operatorPipeSetAssignmentSeq(&mut self, ctx: &OperatorPipeSetAssignmentSeqContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#ansiNonReserved}.
	 * @param ctx the parse tree
	 */
	fn visit_ansiNonReserved(&mut self, ctx: &AnsiNonReservedContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#strictNonReserved}.
	 * @param ctx the parse tree
	 */
	fn visit_strictNonReserved(&mut self, ctx: &StrictNonReservedContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#nonReserved}.
	 * @param ctx the parse tree
	 */
	fn visit_nonReserved(&mut self, ctx: &NonReservedContext<'input>) { self.visit_children(ctx) }

}

pub trait SqlBaseParserVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= SqlBaseParserContextType>{
	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#compoundOrSingleStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_compoundOrSingleStatement(&mut self, ctx: &CompoundOrSingleStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#singleCompoundStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_singleCompoundStatement(&mut self, ctx: &SingleCompoundStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#beginEndCompoundBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_beginEndCompoundBlock(&mut self, ctx: &BeginEndCompoundBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#compoundBody}.
	 * @param ctx the parse tree
	 */
		fn visit_compoundBody(&mut self, ctx: &CompoundBodyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#compoundStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_compoundStatement(&mut self, ctx: &CompoundStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code setVariableInsideSqlScript}
	 * labeled alternative in {@link SqlBaseParser#setStatementInsideSqlScript}.
	 * @param ctx the parse tree
	 */
		fn visit_setVariableInsideSqlScript(&mut self, ctx: &SetVariableInsideSqlScriptContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#sqlStateValue}.
	 * @param ctx the parse tree
	 */
		fn visit_sqlStateValue(&mut self, ctx: &SqlStateValueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#declareConditionStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_declareConditionStatement(&mut self, ctx: &DeclareConditionStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#conditionValue}.
	 * @param ctx the parse tree
	 */
		fn visit_conditionValue(&mut self, ctx: &ConditionValueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#conditionValues}.
	 * @param ctx the parse tree
	 */
		fn visit_conditionValues(&mut self, ctx: &ConditionValuesContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#declareHandlerStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_declareHandlerStatement(&mut self, ctx: &DeclareHandlerStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#whileStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_whileStatement(&mut self, ctx: &WhileStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#ifElseStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_ifElseStatement(&mut self, ctx: &IfElseStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#repeatStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_repeatStatement(&mut self, ctx: &RepeatStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#leaveStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_leaveStatement(&mut self, ctx: &LeaveStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#iterateStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_iterateStatement(&mut self, ctx: &IterateStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code searchedCaseStatement}
	 * labeled alternative in {@link SqlBaseParser#caseStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_searchedCaseStatement(&mut self, ctx: &SearchedCaseStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code simpleCaseStatement}
	 * labeled alternative in {@link SqlBaseParser#caseStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_simpleCaseStatement(&mut self, ctx: &SimpleCaseStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#loopStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_loopStatement(&mut self, ctx: &LoopStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#forStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_forStatement(&mut self, ctx: &ForStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#singleStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_singleStatement(&mut self, ctx: &SingleStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#beginLabel}.
	 * @param ctx the parse tree
	 */
		fn visit_beginLabel(&mut self, ctx: &BeginLabelContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#endLabel}.
	 * @param ctx the parse tree
	 */
		fn visit_endLabel(&mut self, ctx: &EndLabelContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#singleExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_singleExpression(&mut self, ctx: &SingleExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#singleTableIdentifier}.
	 * @param ctx the parse tree
	 */
		fn visit_singleTableIdentifier(&mut self, ctx: &SingleTableIdentifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#singleMultipartIdentifier}.
	 * @param ctx the parse tree
	 */
		fn visit_singleMultipartIdentifier(&mut self, ctx: &SingleMultipartIdentifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#singleFunctionIdentifier}.
	 * @param ctx the parse tree
	 */
		fn visit_singleFunctionIdentifier(&mut self, ctx: &SingleFunctionIdentifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#singleDataType}.
	 * @param ctx the parse tree
	 */
		fn visit_singleDataType(&mut self, ctx: &SingleDataTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#singleTableSchema}.
	 * @param ctx the parse tree
	 */
		fn visit_singleTableSchema(&mut self, ctx: &SingleTableSchemaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#singleRoutineParamList}.
	 * @param ctx the parse tree
	 */
		fn visit_singleRoutineParamList(&mut self, ctx: &SingleRoutineParamListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code statementDefault}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_statementDefault(&mut self, ctx: &StatementDefaultContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code visitExecuteImmediate}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_visitExecuteImmediate(&mut self, ctx: &VisitExecuteImmediateContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code dmlStatement}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_dmlStatement(&mut self, ctx: &DmlStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code use}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_use(&mut self, ctx: &UseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code useNamespace}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_useNamespace(&mut self, ctx: &UseNamespaceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code setCatalog}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_setCatalog(&mut self, ctx: &SetCatalogContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code createNamespace}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_createNamespace(&mut self, ctx: &CreateNamespaceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code setNamespaceProperties}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_setNamespaceProperties(&mut self, ctx: &SetNamespacePropertiesContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code unsetNamespaceProperties}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_unsetNamespaceProperties(&mut self, ctx: &UnsetNamespacePropertiesContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code setNamespaceCollation}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_setNamespaceCollation(&mut self, ctx: &SetNamespaceCollationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code setNamespaceLocation}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_setNamespaceLocation(&mut self, ctx: &SetNamespaceLocationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code dropNamespace}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_dropNamespace(&mut self, ctx: &DropNamespaceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code showNamespaces}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_showNamespaces(&mut self, ctx: &ShowNamespacesContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code createTable}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_createTable(&mut self, ctx: &CreateTableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code createTableLike}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_createTableLike(&mut self, ctx: &CreateTableLikeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code replaceTable}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_replaceTable(&mut self, ctx: &ReplaceTableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code analyze}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_analyze(&mut self, ctx: &AnalyzeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code analyzeTables}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_analyzeTables(&mut self, ctx: &AnalyzeTablesContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code addTableColumns}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_addTableColumns(&mut self, ctx: &AddTableColumnsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code renameTableColumn}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_renameTableColumn(&mut self, ctx: &RenameTableColumnContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code dropTableColumns}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_dropTableColumns(&mut self, ctx: &DropTableColumnsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code renameTable}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_renameTable(&mut self, ctx: &RenameTableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code setTableProperties}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_setTableProperties(&mut self, ctx: &SetTablePropertiesContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code unsetTableProperties}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_unsetTableProperties(&mut self, ctx: &UnsetTablePropertiesContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code alterTableAlterColumn}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_alterTableAlterColumn(&mut self, ctx: &AlterTableAlterColumnContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code hiveChangeColumn}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_hiveChangeColumn(&mut self, ctx: &HiveChangeColumnContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code hiveReplaceColumns}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_hiveReplaceColumns(&mut self, ctx: &HiveReplaceColumnsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code setTableSerDe}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_setTableSerDe(&mut self, ctx: &SetTableSerDeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code addTablePartition}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_addTablePartition(&mut self, ctx: &AddTablePartitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code renameTablePartition}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_renameTablePartition(&mut self, ctx: &RenameTablePartitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code dropTablePartitions}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_dropTablePartitions(&mut self, ctx: &DropTablePartitionsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code setTableLocation}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_setTableLocation(&mut self, ctx: &SetTableLocationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code recoverPartitions}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_recoverPartitions(&mut self, ctx: &RecoverPartitionsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code alterClusterBy}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_alterClusterBy(&mut self, ctx: &AlterClusterByContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code alterTableCollation}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_alterTableCollation(&mut self, ctx: &AlterTableCollationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code addTableConstraint}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_addTableConstraint(&mut self, ctx: &AddTableConstraintContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code dropTableConstraint}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_dropTableConstraint(&mut self, ctx: &DropTableConstraintContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code dropTable}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_dropTable(&mut self, ctx: &DropTableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code dropView}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_dropView(&mut self, ctx: &DropViewContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code createView}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_createView(&mut self, ctx: &CreateViewContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code createMetricView}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_createMetricView(&mut self, ctx: &CreateMetricViewContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code createTempViewUsing}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_createTempViewUsing(&mut self, ctx: &CreateTempViewUsingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code alterViewQuery}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_alterViewQuery(&mut self, ctx: &AlterViewQueryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code alterViewSchemaBinding}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_alterViewSchemaBinding(&mut self, ctx: &AlterViewSchemaBindingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code createFunction}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_createFunction(&mut self, ctx: &CreateFunctionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code createUserDefinedFunction}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_createUserDefinedFunction(&mut self, ctx: &CreateUserDefinedFunctionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code dropFunction}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_dropFunction(&mut self, ctx: &DropFunctionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code createVariable}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_createVariable(&mut self, ctx: &CreateVariableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code dropVariable}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_dropVariable(&mut self, ctx: &DropVariableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code explain}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_explain(&mut self, ctx: &ExplainContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code showTables}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_showTables(&mut self, ctx: &ShowTablesContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code showTableExtended}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_showTableExtended(&mut self, ctx: &ShowTableExtendedContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code showTblProperties}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_showTblProperties(&mut self, ctx: &ShowTblPropertiesContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code showColumns}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_showColumns(&mut self, ctx: &ShowColumnsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code showViews}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_showViews(&mut self, ctx: &ShowViewsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code showPartitions}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_showPartitions(&mut self, ctx: &ShowPartitionsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code showFunctions}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_showFunctions(&mut self, ctx: &ShowFunctionsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code showProcedures}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_showProcedures(&mut self, ctx: &ShowProceduresContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code showCreateTable}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_showCreateTable(&mut self, ctx: &ShowCreateTableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code showCurrentNamespace}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_showCurrentNamespace(&mut self, ctx: &ShowCurrentNamespaceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code showCatalogs}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_showCatalogs(&mut self, ctx: &ShowCatalogsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code describeFunction}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_describeFunction(&mut self, ctx: &DescribeFunctionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code describeProcedure}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_describeProcedure(&mut self, ctx: &DescribeProcedureContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code describeNamespace}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_describeNamespace(&mut self, ctx: &DescribeNamespaceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code describeRelation}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_describeRelation(&mut self, ctx: &DescribeRelationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code describeQuery}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_describeQuery(&mut self, ctx: &DescribeQueryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code commentNamespace}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_commentNamespace(&mut self, ctx: &CommentNamespaceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code commentTable}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_commentTable(&mut self, ctx: &CommentTableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code refreshTable}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_refreshTable(&mut self, ctx: &RefreshTableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code refreshFunction}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_refreshFunction(&mut self, ctx: &RefreshFunctionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code refreshResource}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_refreshResource(&mut self, ctx: &RefreshResourceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code cacheTable}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_cacheTable(&mut self, ctx: &CacheTableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code uncacheTable}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_uncacheTable(&mut self, ctx: &UncacheTableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code clearCache}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_clearCache(&mut self, ctx: &ClearCacheContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code loadData}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_loadData(&mut self, ctx: &LoadDataContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code truncateTable}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_truncateTable(&mut self, ctx: &TruncateTableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code repairTable}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_repairTable(&mut self, ctx: &RepairTableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code manageResource}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_manageResource(&mut self, ctx: &ManageResourceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code createIndex}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_createIndex(&mut self, ctx: &CreateIndexContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code dropIndex}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_dropIndex(&mut self, ctx: &DropIndexContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code call}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_call(&mut self, ctx: &CallContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code failNativeCommand}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_failNativeCommand(&mut self, ctx: &FailNativeCommandContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code createPipelineDataset}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_createPipelineDataset(&mut self, ctx: &CreatePipelineDatasetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code createPipelineInsertIntoFlow}
	 * labeled alternative in {@link SqlBaseParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_createPipelineInsertIntoFlow(&mut self, ctx: &CreatePipelineInsertIntoFlowContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#materializedView}.
	 * @param ctx the parse tree
	 */
		fn visit_materializedView(&mut self, ctx: &MaterializedViewContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#streamingTable}.
	 * @param ctx the parse tree
	 */
		fn visit_streamingTable(&mut self, ctx: &StreamingTableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#createPipelineDatasetHeader}.
	 * @param ctx the parse tree
	 */
		fn visit_createPipelineDatasetHeader(&mut self, ctx: &CreatePipelineDatasetHeaderContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code streamTableName}
	 * labeled alternative in {@link SqlBaseParser#streamRelationPrimary}.
	 * @param ctx the parse tree
	 */
		fn visit_streamTableName(&mut self, ctx: &StreamTableNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code failSetRole}
	 * labeled alternative in {@link SqlBaseParser#setResetStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_failSetRole(&mut self, ctx: &FailSetRoleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code setTimeZone}
	 * labeled alternative in {@link SqlBaseParser#setResetStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_setTimeZone(&mut self, ctx: &SetTimeZoneContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code setVariable}
	 * labeled alternative in {@link SqlBaseParser#setResetStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_setVariable(&mut self, ctx: &SetVariableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code setQuotedConfiguration}
	 * labeled alternative in {@link SqlBaseParser#setResetStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_setQuotedConfiguration(&mut self, ctx: &SetQuotedConfigurationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code setConfiguration}
	 * labeled alternative in {@link SqlBaseParser#setResetStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_setConfiguration(&mut self, ctx: &SetConfigurationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code resetQuotedConfiguration}
	 * labeled alternative in {@link SqlBaseParser#setResetStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_resetQuotedConfiguration(&mut self, ctx: &ResetQuotedConfigurationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code resetConfiguration}
	 * labeled alternative in {@link SqlBaseParser#setResetStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_resetConfiguration(&mut self, ctx: &ResetConfigurationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#executeImmediate}.
	 * @param ctx the parse tree
	 */
		fn visit_executeImmediate(&mut self, ctx: &ExecuteImmediateContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#executeImmediateUsing}.
	 * @param ctx the parse tree
	 */
		fn visit_executeImmediateUsing(&mut self, ctx: &ExecuteImmediateUsingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#timezone}.
	 * @param ctx the parse tree
	 */
		fn visit_timezone(&mut self, ctx: &TimezoneContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#configKey}.
	 * @param ctx the parse tree
	 */
		fn visit_configKey(&mut self, ctx: &ConfigKeyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#configValue}.
	 * @param ctx the parse tree
	 */
		fn visit_configValue(&mut self, ctx: &ConfigValueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#unsupportedHiveNativeCommands}.
	 * @param ctx the parse tree
	 */
		fn visit_unsupportedHiveNativeCommands(&mut self, ctx: &UnsupportedHiveNativeCommandsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#createTableHeader}.
	 * @param ctx the parse tree
	 */
		fn visit_createTableHeader(&mut self, ctx: &CreateTableHeaderContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#replaceTableHeader}.
	 * @param ctx the parse tree
	 */
		fn visit_replaceTableHeader(&mut self, ctx: &ReplaceTableHeaderContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#clusterBySpec}.
	 * @param ctx the parse tree
	 */
		fn visit_clusterBySpec(&mut self, ctx: &ClusterBySpecContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#bucketSpec}.
	 * @param ctx the parse tree
	 */
		fn visit_bucketSpec(&mut self, ctx: &BucketSpecContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#skewSpec}.
	 * @param ctx the parse tree
	 */
		fn visit_skewSpec(&mut self, ctx: &SkewSpecContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#locationSpec}.
	 * @param ctx the parse tree
	 */
		fn visit_locationSpec(&mut self, ctx: &LocationSpecContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#schemaBinding}.
	 * @param ctx the parse tree
	 */
		fn visit_schemaBinding(&mut self, ctx: &SchemaBindingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#commentSpec}.
	 * @param ctx the parse tree
	 */
		fn visit_commentSpec(&mut self, ctx: &CommentSpecContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#singleQuery}.
	 * @param ctx the parse tree
	 */
		fn visit_singleQuery(&mut self, ctx: &SingleQueryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#query}.
	 * @param ctx the parse tree
	 */
		fn visit_query(&mut self, ctx: &QueryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code insertOverwriteTable}
	 * labeled alternative in {@link SqlBaseParser#insertInto}.
	 * @param ctx the parse tree
	 */
		fn visit_insertOverwriteTable(&mut self, ctx: &InsertOverwriteTableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code insertIntoTable}
	 * labeled alternative in {@link SqlBaseParser#insertInto}.
	 * @param ctx the parse tree
	 */
		fn visit_insertIntoTable(&mut self, ctx: &InsertIntoTableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code insertIntoReplaceWhere}
	 * labeled alternative in {@link SqlBaseParser#insertInto}.
	 * @param ctx the parse tree
	 */
		fn visit_insertIntoReplaceWhere(&mut self, ctx: &InsertIntoReplaceWhereContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code insertOverwriteHiveDir}
	 * labeled alternative in {@link SqlBaseParser#insertInto}.
	 * @param ctx the parse tree
	 */
		fn visit_insertOverwriteHiveDir(&mut self, ctx: &InsertOverwriteHiveDirContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code insertOverwriteDir}
	 * labeled alternative in {@link SqlBaseParser#insertInto}.
	 * @param ctx the parse tree
	 */
		fn visit_insertOverwriteDir(&mut self, ctx: &InsertOverwriteDirContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#partitionSpecLocation}.
	 * @param ctx the parse tree
	 */
		fn visit_partitionSpecLocation(&mut self, ctx: &PartitionSpecLocationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#partitionSpec}.
	 * @param ctx the parse tree
	 */
		fn visit_partitionSpec(&mut self, ctx: &PartitionSpecContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#partitionVal}.
	 * @param ctx the parse tree
	 */
		fn visit_partitionVal(&mut self, ctx: &PartitionValContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#createPipelineFlowHeader}.
	 * @param ctx the parse tree
	 */
		fn visit_createPipelineFlowHeader(&mut self, ctx: &CreatePipelineFlowHeaderContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#namespace}.
	 * @param ctx the parse tree
	 */
		fn visit_namespace(&mut self, ctx: &NamespaceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#namespaces}.
	 * @param ctx the parse tree
	 */
		fn visit_namespaces(&mut self, ctx: &NamespacesContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#variable}.
	 * @param ctx the parse tree
	 */
		fn visit_variable(&mut self, ctx: &VariableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#describeFuncName}.
	 * @param ctx the parse tree
	 */
		fn visit_describeFuncName(&mut self, ctx: &DescribeFuncNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#describeColName}.
	 * @param ctx the parse tree
	 */
		fn visit_describeColName(&mut self, ctx: &DescribeColNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#ctes}.
	 * @param ctx the parse tree
	 */
		fn visit_ctes(&mut self, ctx: &CtesContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#namedQuery}.
	 * @param ctx the parse tree
	 */
		fn visit_namedQuery(&mut self, ctx: &NamedQueryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#tableProvider}.
	 * @param ctx the parse tree
	 */
		fn visit_tableProvider(&mut self, ctx: &TableProviderContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#createTableClauses}.
	 * @param ctx the parse tree
	 */
		fn visit_createTableClauses(&mut self, ctx: &CreateTableClausesContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#propertyList}.
	 * @param ctx the parse tree
	 */
		fn visit_propertyList(&mut self, ctx: &PropertyListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code propertyWithKeyAndEquals}
	 * labeled alternative in {@link SqlBaseParser#property}.
	 * @param ctx the parse tree
	 */
		fn visit_propertyWithKeyAndEquals(&mut self, ctx: &PropertyWithKeyAndEqualsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code propertyWithKeyNoEquals}
	 * labeled alternative in {@link SqlBaseParser#property}.
	 * @param ctx the parse tree
	 */
		fn visit_propertyWithKeyNoEquals(&mut self, ctx: &PropertyWithKeyNoEqualsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#propertyKey}.
	 * @param ctx the parse tree
	 */
		fn visit_propertyKey(&mut self, ctx: &PropertyKeyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#propertyKeyOrStringLit}.
	 * @param ctx the parse tree
	 */
		fn visit_propertyKeyOrStringLit(&mut self, ctx: &PropertyKeyOrStringLitContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#propertyKeyOrStringLitNoCoalesce}.
	 * @param ctx the parse tree
	 */
		fn visit_propertyKeyOrStringLitNoCoalesce(&mut self, ctx: &PropertyKeyOrStringLitNoCoalesceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#propertyValue}.
	 * @param ctx the parse tree
	 */
		fn visit_propertyValue(&mut self, ctx: &PropertyValueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#expressionPropertyList}.
	 * @param ctx the parse tree
	 */
		fn visit_expressionPropertyList(&mut self, ctx: &ExpressionPropertyListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code expressionPropertyWithKeyAndEquals}
	 * labeled alternative in {@link SqlBaseParser#expressionProperty}.
	 * @param ctx the parse tree
	 */
		fn visit_expressionPropertyWithKeyAndEquals(&mut self, ctx: &ExpressionPropertyWithKeyAndEqualsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code expressionPropertyWithKeyNoEquals}
	 * labeled alternative in {@link SqlBaseParser#expressionProperty}.
	 * @param ctx the parse tree
	 */
		fn visit_expressionPropertyWithKeyNoEquals(&mut self, ctx: &ExpressionPropertyWithKeyNoEqualsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#constantList}.
	 * @param ctx the parse tree
	 */
		fn visit_constantList(&mut self, ctx: &ConstantListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#nestedConstantList}.
	 * @param ctx the parse tree
	 */
		fn visit_nestedConstantList(&mut self, ctx: &NestedConstantListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#createFileFormat}.
	 * @param ctx the parse tree
	 */
		fn visit_createFileFormat(&mut self, ctx: &CreateFileFormatContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code tableFileFormat}
	 * labeled alternative in {@link SqlBaseParser#fileFormat}.
	 * @param ctx the parse tree
	 */
		fn visit_tableFileFormat(&mut self, ctx: &TableFileFormatContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code genericFileFormat}
	 * labeled alternative in {@link SqlBaseParser#fileFormat}.
	 * @param ctx the parse tree
	 */
		fn visit_genericFileFormat(&mut self, ctx: &GenericFileFormatContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#storageHandler}.
	 * @param ctx the parse tree
	 */
		fn visit_storageHandler(&mut self, ctx: &StorageHandlerContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#resource}.
	 * @param ctx the parse tree
	 */
		fn visit_resource(&mut self, ctx: &ResourceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code singleInsertQuery}
	 * labeled alternative in {@link SqlBaseParser#dmlStatementNoWith}.
	 * @param ctx the parse tree
	 */
		fn visit_singleInsertQuery(&mut self, ctx: &SingleInsertQueryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code multiInsertQuery}
	 * labeled alternative in {@link SqlBaseParser#dmlStatementNoWith}.
	 * @param ctx the parse tree
	 */
		fn visit_multiInsertQuery(&mut self, ctx: &MultiInsertQueryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code deleteFromTable}
	 * labeled alternative in {@link SqlBaseParser#dmlStatementNoWith}.
	 * @param ctx the parse tree
	 */
		fn visit_deleteFromTable(&mut self, ctx: &DeleteFromTableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code updateTable}
	 * labeled alternative in {@link SqlBaseParser#dmlStatementNoWith}.
	 * @param ctx the parse tree
	 */
		fn visit_updateTable(&mut self, ctx: &UpdateTableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code mergeIntoTable}
	 * labeled alternative in {@link SqlBaseParser#dmlStatementNoWith}.
	 * @param ctx the parse tree
	 */
		fn visit_mergeIntoTable(&mut self, ctx: &MergeIntoTableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#identifierReference}.
	 * @param ctx the parse tree
	 */
		fn visit_identifierReference(&mut self, ctx: &IdentifierReferenceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#catalogIdentifierReference}.
	 * @param ctx the parse tree
	 */
		fn visit_catalogIdentifierReference(&mut self, ctx: &CatalogIdentifierReferenceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#queryOrganization}.
	 * @param ctx the parse tree
	 */
		fn visit_queryOrganization(&mut self, ctx: &QueryOrganizationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#multiInsertQueryBody}.
	 * @param ctx the parse tree
	 */
		fn visit_multiInsertQueryBody(&mut self, ctx: &MultiInsertQueryBodyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code operatorPipeStatement}
	 * labeled alternative in {@link SqlBaseParser#queryTerm}.
	 * @param ctx the parse tree
	 */
		fn visit_operatorPipeStatement(&mut self, ctx: &OperatorPipeStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code queryTermDefault}
	 * labeled alternative in {@link SqlBaseParser#queryTerm}.
	 * @param ctx the parse tree
	 */
		fn visit_queryTermDefault(&mut self, ctx: &QueryTermDefaultContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code setOperation}
	 * labeled alternative in {@link SqlBaseParser#queryTerm}.
	 * @param ctx the parse tree
	 */
		fn visit_setOperation(&mut self, ctx: &SetOperationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code queryPrimaryDefault}
	 * labeled alternative in {@link SqlBaseParser#queryPrimary}.
	 * @param ctx the parse tree
	 */
		fn visit_queryPrimaryDefault(&mut self, ctx: &QueryPrimaryDefaultContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code fromStmt}
	 * labeled alternative in {@link SqlBaseParser#queryPrimary}.
	 * @param ctx the parse tree
	 */
		fn visit_fromStmt(&mut self, ctx: &FromStmtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code table}
	 * labeled alternative in {@link SqlBaseParser#queryPrimary}.
	 * @param ctx the parse tree
	 */
		fn visit_table(&mut self, ctx: &TableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code inlineTableDefault1}
	 * labeled alternative in {@link SqlBaseParser#queryPrimary}.
	 * @param ctx the parse tree
	 */
		fn visit_inlineTableDefault1(&mut self, ctx: &InlineTableDefault1Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code subquery}
	 * labeled alternative in {@link SqlBaseParser#queryPrimary}.
	 * @param ctx the parse tree
	 */
		fn visit_subquery(&mut self, ctx: &SubqueryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#sortItem}.
	 * @param ctx the parse tree
	 */
		fn visit_sortItem(&mut self, ctx: &SortItemContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#fromStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_fromStatement(&mut self, ctx: &FromStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#fromStatementBody}.
	 * @param ctx the parse tree
	 */
		fn visit_fromStatementBody(&mut self, ctx: &FromStatementBodyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code transformQuerySpecification}
	 * labeled alternative in {@link SqlBaseParser#querySpecification}.
	 * @param ctx the parse tree
	 */
		fn visit_transformQuerySpecification(&mut self, ctx: &TransformQuerySpecificationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code regularQuerySpecification}
	 * labeled alternative in {@link SqlBaseParser#querySpecification}.
	 * @param ctx the parse tree
	 */
		fn visit_regularQuerySpecification(&mut self, ctx: &RegularQuerySpecificationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#transformClause}.
	 * @param ctx the parse tree
	 */
		fn visit_transformClause(&mut self, ctx: &TransformClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#selectClause}.
	 * @param ctx the parse tree
	 */
		fn visit_selectClause(&mut self, ctx: &SelectClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#setClause}.
	 * @param ctx the parse tree
	 */
		fn visit_setClause(&mut self, ctx: &SetClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#matchedClause}.
	 * @param ctx the parse tree
	 */
		fn visit_matchedClause(&mut self, ctx: &MatchedClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#notMatchedClause}.
	 * @param ctx the parse tree
	 */
		fn visit_notMatchedClause(&mut self, ctx: &NotMatchedClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#notMatchedBySourceClause}.
	 * @param ctx the parse tree
	 */
		fn visit_notMatchedBySourceClause(&mut self, ctx: &NotMatchedBySourceClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#matchedAction}.
	 * @param ctx the parse tree
	 */
		fn visit_matchedAction(&mut self, ctx: &MatchedActionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#notMatchedAction}.
	 * @param ctx the parse tree
	 */
		fn visit_notMatchedAction(&mut self, ctx: &NotMatchedActionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#notMatchedBySourceAction}.
	 * @param ctx the parse tree
	 */
		fn visit_notMatchedBySourceAction(&mut self, ctx: &NotMatchedBySourceActionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#exceptClause}.
	 * @param ctx the parse tree
	 */
		fn visit_exceptClause(&mut self, ctx: &ExceptClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#assignmentList}.
	 * @param ctx the parse tree
	 */
		fn visit_assignmentList(&mut self, ctx: &AssignmentListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#assignment}.
	 * @param ctx the parse tree
	 */
		fn visit_assignment(&mut self, ctx: &AssignmentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#whereClause}.
	 * @param ctx the parse tree
	 */
		fn visit_whereClause(&mut self, ctx: &WhereClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#havingClause}.
	 * @param ctx the parse tree
	 */
		fn visit_havingClause(&mut self, ctx: &HavingClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#hint}.
	 * @param ctx the parse tree
	 */
		fn visit_hint(&mut self, ctx: &HintContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#hintStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_hintStatement(&mut self, ctx: &HintStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#fromClause}.
	 * @param ctx the parse tree
	 */
		fn visit_fromClause(&mut self, ctx: &FromClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#temporalClause}.
	 * @param ctx the parse tree
	 */
		fn visit_temporalClause(&mut self, ctx: &TemporalClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#aggregationClause}.
	 * @param ctx the parse tree
	 */
		fn visit_aggregationClause(&mut self, ctx: &AggregationClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#groupByClause}.
	 * @param ctx the parse tree
	 */
		fn visit_groupByClause(&mut self, ctx: &GroupByClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#groupingAnalytics}.
	 * @param ctx the parse tree
	 */
		fn visit_groupingAnalytics(&mut self, ctx: &GroupingAnalyticsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#groupingElement}.
	 * @param ctx the parse tree
	 */
		fn visit_groupingElement(&mut self, ctx: &GroupingElementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#groupingSet}.
	 * @param ctx the parse tree
	 */
		fn visit_groupingSet(&mut self, ctx: &GroupingSetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#pivotClause}.
	 * @param ctx the parse tree
	 */
		fn visit_pivotClause(&mut self, ctx: &PivotClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#pivotColumn}.
	 * @param ctx the parse tree
	 */
		fn visit_pivotColumn(&mut self, ctx: &PivotColumnContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#pivotValue}.
	 * @param ctx the parse tree
	 */
		fn visit_pivotValue(&mut self, ctx: &PivotValueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#unpivotClause}.
	 * @param ctx the parse tree
	 */
		fn visit_unpivotClause(&mut self, ctx: &UnpivotClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#unpivotNullClause}.
	 * @param ctx the parse tree
	 */
		fn visit_unpivotNullClause(&mut self, ctx: &UnpivotNullClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#unpivotOperator}.
	 * @param ctx the parse tree
	 */
		fn visit_unpivotOperator(&mut self, ctx: &UnpivotOperatorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#unpivotSingleValueColumnClause}.
	 * @param ctx the parse tree
	 */
		fn visit_unpivotSingleValueColumnClause(&mut self, ctx: &UnpivotSingleValueColumnClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#unpivotMultiValueColumnClause}.
	 * @param ctx the parse tree
	 */
		fn visit_unpivotMultiValueColumnClause(&mut self, ctx: &UnpivotMultiValueColumnClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#unpivotColumnSet}.
	 * @param ctx the parse tree
	 */
		fn visit_unpivotColumnSet(&mut self, ctx: &UnpivotColumnSetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#unpivotValueColumn}.
	 * @param ctx the parse tree
	 */
		fn visit_unpivotValueColumn(&mut self, ctx: &UnpivotValueColumnContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#unpivotNameColumn}.
	 * @param ctx the parse tree
	 */
		fn visit_unpivotNameColumn(&mut self, ctx: &UnpivotNameColumnContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#unpivotColumnAndAlias}.
	 * @param ctx the parse tree
	 */
		fn visit_unpivotColumnAndAlias(&mut self, ctx: &UnpivotColumnAndAliasContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#unpivotColumn}.
	 * @param ctx the parse tree
	 */
		fn visit_unpivotColumn(&mut self, ctx: &UnpivotColumnContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#unpivotAlias}.
	 * @param ctx the parse tree
	 */
		fn visit_unpivotAlias(&mut self, ctx: &UnpivotAliasContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#lateralView}.
	 * @param ctx the parse tree
	 */
		fn visit_lateralView(&mut self, ctx: &LateralViewContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#watermarkClause}.
	 * @param ctx the parse tree
	 */
		fn visit_watermarkClause(&mut self, ctx: &WatermarkClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#setQuantifier}.
	 * @param ctx the parse tree
	 */
		fn visit_setQuantifier(&mut self, ctx: &SetQuantifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#relation}.
	 * @param ctx the parse tree
	 */
		fn visit_relation(&mut self, ctx: &RelationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#relationExtension}.
	 * @param ctx the parse tree
	 */
		fn visit_relationExtension(&mut self, ctx: &RelationExtensionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#joinRelation}.
	 * @param ctx the parse tree
	 */
		fn visit_joinRelation(&mut self, ctx: &JoinRelationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#joinType}.
	 * @param ctx the parse tree
	 */
		fn visit_joinType(&mut self, ctx: &JoinTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#joinCriteria}.
	 * @param ctx the parse tree
	 */
		fn visit_joinCriteria(&mut self, ctx: &JoinCriteriaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#sample}.
	 * @param ctx the parse tree
	 */
		fn visit_sample(&mut self, ctx: &SampleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code sampleByPercentile}
	 * labeled alternative in {@link SqlBaseParser#sampleMethod}.
	 * @param ctx the parse tree
	 */
		fn visit_sampleByPercentile(&mut self, ctx: &SampleByPercentileContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code sampleByRows}
	 * labeled alternative in {@link SqlBaseParser#sampleMethod}.
	 * @param ctx the parse tree
	 */
		fn visit_sampleByRows(&mut self, ctx: &SampleByRowsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code sampleByBucket}
	 * labeled alternative in {@link SqlBaseParser#sampleMethod}.
	 * @param ctx the parse tree
	 */
		fn visit_sampleByBucket(&mut self, ctx: &SampleByBucketContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code sampleByBytes}
	 * labeled alternative in {@link SqlBaseParser#sampleMethod}.
	 * @param ctx the parse tree
	 */
		fn visit_sampleByBytes(&mut self, ctx: &SampleByBytesContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#identifierList}.
	 * @param ctx the parse tree
	 */
		fn visit_identifierList(&mut self, ctx: &IdentifierListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#identifierSeq}.
	 * @param ctx the parse tree
	 */
		fn visit_identifierSeq(&mut self, ctx: &IdentifierSeqContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#orderedIdentifierList}.
	 * @param ctx the parse tree
	 */
		fn visit_orderedIdentifierList(&mut self, ctx: &OrderedIdentifierListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#orderedIdentifier}.
	 * @param ctx the parse tree
	 */
		fn visit_orderedIdentifier(&mut self, ctx: &OrderedIdentifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#identifierCommentList}.
	 * @param ctx the parse tree
	 */
		fn visit_identifierCommentList(&mut self, ctx: &IdentifierCommentListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#identifierComment}.
	 * @param ctx the parse tree
	 */
		fn visit_identifierComment(&mut self, ctx: &IdentifierCommentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code streamRelation}
	 * labeled alternative in {@link SqlBaseParser#relationPrimary}.
	 * @param ctx the parse tree
	 */
		fn visit_streamRelation(&mut self, ctx: &StreamRelationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code tableName}
	 * labeled alternative in {@link SqlBaseParser#relationPrimary}.
	 * @param ctx the parse tree
	 */
		fn visit_tableName(&mut self, ctx: &TableNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code aliasedQuery}
	 * labeled alternative in {@link SqlBaseParser#relationPrimary}.
	 * @param ctx the parse tree
	 */
		fn visit_aliasedQuery(&mut self, ctx: &AliasedQueryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code aliasedRelation}
	 * labeled alternative in {@link SqlBaseParser#relationPrimary}.
	 * @param ctx the parse tree
	 */
		fn visit_aliasedRelation(&mut self, ctx: &AliasedRelationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code inlineTableDefault2}
	 * labeled alternative in {@link SqlBaseParser#relationPrimary}.
	 * @param ctx the parse tree
	 */
		fn visit_inlineTableDefault2(&mut self, ctx: &InlineTableDefault2Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code tableValuedFunction}
	 * labeled alternative in {@link SqlBaseParser#relationPrimary}.
	 * @param ctx the parse tree
	 */
		fn visit_tableValuedFunction(&mut self, ctx: &TableValuedFunctionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#optionsClause}.
	 * @param ctx the parse tree
	 */
		fn visit_optionsClause(&mut self, ctx: &OptionsClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#inlineTable}.
	 * @param ctx the parse tree
	 */
		fn visit_inlineTable(&mut self, ctx: &InlineTableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#functionTableSubqueryArgument}.
	 * @param ctx the parse tree
	 */
		fn visit_functionTableSubqueryArgument(&mut self, ctx: &FunctionTableSubqueryArgumentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#tableArgumentPartitioning}.
	 * @param ctx the parse tree
	 */
		fn visit_tableArgumentPartitioning(&mut self, ctx: &TableArgumentPartitioningContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#functionTableNamedArgumentExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_functionTableNamedArgumentExpression(&mut self, ctx: &FunctionTableNamedArgumentExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#functionTableReferenceArgument}.
	 * @param ctx the parse tree
	 */
		fn visit_functionTableReferenceArgument(&mut self, ctx: &FunctionTableReferenceArgumentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#functionTableArgument}.
	 * @param ctx the parse tree
	 */
		fn visit_functionTableArgument(&mut self, ctx: &FunctionTableArgumentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#functionTable}.
	 * @param ctx the parse tree
	 */
		fn visit_functionTable(&mut self, ctx: &FunctionTableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#tableAlias}.
	 * @param ctx the parse tree
	 */
		fn visit_tableAlias(&mut self, ctx: &TableAliasContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code rowFormatSerde}
	 * labeled alternative in {@link SqlBaseParser#rowFormat}.
	 * @param ctx the parse tree
	 */
		fn visit_rowFormatSerde(&mut self, ctx: &RowFormatSerdeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code rowFormatDelimited}
	 * labeled alternative in {@link SqlBaseParser#rowFormat}.
	 * @param ctx the parse tree
	 */
		fn visit_rowFormatDelimited(&mut self, ctx: &RowFormatDelimitedContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#multipartIdentifierList}.
	 * @param ctx the parse tree
	 */
		fn visit_multipartIdentifierList(&mut self, ctx: &MultipartIdentifierListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#multipartIdentifier}.
	 * @param ctx the parse tree
	 */
		fn visit_multipartIdentifier(&mut self, ctx: &MultipartIdentifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#multipartIdentifierPropertyList}.
	 * @param ctx the parse tree
	 */
		fn visit_multipartIdentifierPropertyList(&mut self, ctx: &MultipartIdentifierPropertyListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#multipartIdentifierProperty}.
	 * @param ctx the parse tree
	 */
		fn visit_multipartIdentifierProperty(&mut self, ctx: &MultipartIdentifierPropertyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#tableIdentifier}.
	 * @param ctx the parse tree
	 */
		fn visit_tableIdentifier(&mut self, ctx: &TableIdentifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#functionIdentifier}.
	 * @param ctx the parse tree
	 */
		fn visit_functionIdentifier(&mut self, ctx: &FunctionIdentifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#namedExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_namedExpression(&mut self, ctx: &NamedExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#namedExpressionSeq}.
	 * @param ctx the parse tree
	 */
		fn visit_namedExpressionSeq(&mut self, ctx: &NamedExpressionSeqContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#partitionFieldList}.
	 * @param ctx the parse tree
	 */
		fn visit_partitionFieldList(&mut self, ctx: &PartitionFieldListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code partitionTransform}
	 * labeled alternative in {@link SqlBaseParser#partitionField}.
	 * @param ctx the parse tree
	 */
		fn visit_partitionTransform(&mut self, ctx: &PartitionTransformContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code partitionColumn}
	 * labeled alternative in {@link SqlBaseParser#partitionField}.
	 * @param ctx the parse tree
	 */
		fn visit_partitionColumn(&mut self, ctx: &PartitionColumnContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code identityTransform}
	 * labeled alternative in {@link SqlBaseParser#transform}.
	 * @param ctx the parse tree
	 */
		fn visit_identityTransform(&mut self, ctx: &IdentityTransformContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code applyTransform}
	 * labeled alternative in {@link SqlBaseParser#transform}.
	 * @param ctx the parse tree
	 */
		fn visit_applyTransform(&mut self, ctx: &ApplyTransformContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#transformArgument}.
	 * @param ctx the parse tree
	 */
		fn visit_transformArgument(&mut self, ctx: &TransformArgumentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#namedArgumentExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_namedArgumentExpression(&mut self, ctx: &NamedArgumentExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#functionArgument}.
	 * @param ctx the parse tree
	 */
		fn visit_functionArgument(&mut self, ctx: &FunctionArgumentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#expressionSeq}.
	 * @param ctx the parse tree
	 */
		fn visit_expressionSeq(&mut self, ctx: &ExpressionSeqContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code logicalNot}
	 * labeled alternative in {@link SqlBaseParser#booleanExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_logicalNot(&mut self, ctx: &LogicalNotContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code predicated}
	 * labeled alternative in {@link SqlBaseParser#booleanExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_predicated(&mut self, ctx: &PredicatedContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code exists}
	 * labeled alternative in {@link SqlBaseParser#booleanExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_exists(&mut self, ctx: &ExistsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code logicalBinary}
	 * labeled alternative in {@link SqlBaseParser#booleanExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_logicalBinary(&mut self, ctx: &LogicalBinaryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#predicate}.
	 * @param ctx the parse tree
	 */
		fn visit_predicate(&mut self, ctx: &PredicateContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#errorCapturingNot}.
	 * @param ctx the parse tree
	 */
		fn visit_errorCapturingNot(&mut self, ctx: &ErrorCapturingNotContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code valueExpressionDefault}
	 * labeled alternative in {@link SqlBaseParser#valueExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_valueExpressionDefault(&mut self, ctx: &ValueExpressionDefaultContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code comparison}
	 * labeled alternative in {@link SqlBaseParser#valueExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_comparison(&mut self, ctx: &ComparisonContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code shiftExpression}
	 * labeled alternative in {@link SqlBaseParser#valueExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_shiftExpression(&mut self, ctx: &ShiftExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code arithmeticBinary}
	 * labeled alternative in {@link SqlBaseParser#valueExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_arithmeticBinary(&mut self, ctx: &ArithmeticBinaryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code arithmeticUnary}
	 * labeled alternative in {@link SqlBaseParser#valueExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_arithmeticUnary(&mut self, ctx: &ArithmeticUnaryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#shiftOperator}.
	 * @param ctx the parse tree
	 */
		fn visit_shiftOperator(&mut self, ctx: &ShiftOperatorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#datetimeUnit}.
	 * @param ctx the parse tree
	 */
		fn visit_datetimeUnit(&mut self, ctx: &DatetimeUnitContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code struct}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_struct(&mut self, ctx: &StructContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code dereference}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_dereference(&mut self, ctx: &DereferenceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code castByColon}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_castByColon(&mut self, ctx: &CastByColonContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code timestampadd}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_timestampadd(&mut self, ctx: &TimestampaddContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code substring}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_substring(&mut self, ctx: &SubstringContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code cast}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_cast(&mut self, ctx: &CastContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code lambda}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_lambda(&mut self, ctx: &LambdaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code parenthesizedExpression}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_parenthesizedExpression(&mut self, ctx: &ParenthesizedExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code any_value}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_any_value(&mut self, ctx: &Any_valueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code trim}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_trim(&mut self, ctx: &TrimContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code semiStructuredExtract}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_semiStructuredExtract(&mut self, ctx: &SemiStructuredExtractContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code simpleCase}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_simpleCase(&mut self, ctx: &SimpleCaseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code currentLike}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_currentLike(&mut self, ctx: &CurrentLikeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code columnReference}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_columnReference(&mut self, ctx: &ColumnReferenceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code rowConstructor}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_rowConstructor(&mut self, ctx: &RowConstructorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code last}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_last(&mut self, ctx: &LastContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code star}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_star(&mut self, ctx: &StarContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code overlay}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_overlay(&mut self, ctx: &OverlayContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code subscript}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_subscript(&mut self, ctx: &SubscriptContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code timestampdiff}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_timestampdiff(&mut self, ctx: &TimestampdiffContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code subqueryExpression}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_subqueryExpression(&mut self, ctx: &SubqueryExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code collate}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_collate(&mut self, ctx: &CollateContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code constantDefault}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_constantDefault(&mut self, ctx: &ConstantDefaultContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code extract}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_extract(&mut self, ctx: &ExtractContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code functionCall}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_functionCall(&mut self, ctx: &FunctionCallContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code searchedCase}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_searchedCase(&mut self, ctx: &SearchedCaseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code position}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_position(&mut self, ctx: &PositionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code first}
	 * labeled alternative in {@link SqlBaseParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_first(&mut self, ctx: &FirstContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#semiStructuredExtractionPath}.
	 * @param ctx the parse tree
	 */
		fn visit_semiStructuredExtractionPath(&mut self, ctx: &SemiStructuredExtractionPathContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#jsonPathIdentifier}.
	 * @param ctx the parse tree
	 */
		fn visit_jsonPathIdentifier(&mut self, ctx: &JsonPathIdentifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#jsonPathBracketedIdentifier}.
	 * @param ctx the parse tree
	 */
		fn visit_jsonPathBracketedIdentifier(&mut self, ctx: &JsonPathBracketedIdentifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#jsonPathFirstPart}.
	 * @param ctx the parse tree
	 */
		fn visit_jsonPathFirstPart(&mut self, ctx: &JsonPathFirstPartContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#jsonPathParts}.
	 * @param ctx the parse tree
	 */
		fn visit_jsonPathParts(&mut self, ctx: &JsonPathPartsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#literalType}.
	 * @param ctx the parse tree
	 */
		fn visit_literalType(&mut self, ctx: &LiteralTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code nullLiteral}
	 * labeled alternative in {@link SqlBaseParser#constant}.
	 * @param ctx the parse tree
	 */
		fn visit_nullLiteral(&mut self, ctx: &NullLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code posParameterLiteral}
	 * labeled alternative in {@link SqlBaseParser#constant}.
	 * @param ctx the parse tree
	 */
		fn visit_posParameterLiteral(&mut self, ctx: &PosParameterLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code namedParameterLiteral}
	 * labeled alternative in {@link SqlBaseParser#constant}.
	 * @param ctx the parse tree
	 */
		fn visit_namedParameterLiteral(&mut self, ctx: &NamedParameterLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code intervalLiteral}
	 * labeled alternative in {@link SqlBaseParser#constant}.
	 * @param ctx the parse tree
	 */
		fn visit_intervalLiteral(&mut self, ctx: &IntervalLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code typeConstructor}
	 * labeled alternative in {@link SqlBaseParser#constant}.
	 * @param ctx the parse tree
	 */
		fn visit_typeConstructor(&mut self, ctx: &TypeConstructorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code numericLiteral}
	 * labeled alternative in {@link SqlBaseParser#constant}.
	 * @param ctx the parse tree
	 */
		fn visit_numericLiteral(&mut self, ctx: &NumericLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code booleanLiteral}
	 * labeled alternative in {@link SqlBaseParser#constant}.
	 * @param ctx the parse tree
	 */
		fn visit_booleanLiteral(&mut self, ctx: &BooleanLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code stringLiteral}
	 * labeled alternative in {@link SqlBaseParser#constant}.
	 * @param ctx the parse tree
	 */
		fn visit_stringLiteral(&mut self, ctx: &StringLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#namedParameterMarker}.
	 * @param ctx the parse tree
	 */
		fn visit_namedParameterMarker(&mut self, ctx: &NamedParameterMarkerContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#comparisonOperator}.
	 * @param ctx the parse tree
	 */
		fn visit_comparisonOperator(&mut self, ctx: &ComparisonOperatorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#arithmeticOperator}.
	 * @param ctx the parse tree
	 */
		fn visit_arithmeticOperator(&mut self, ctx: &ArithmeticOperatorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#predicateOperator}.
	 * @param ctx the parse tree
	 */
		fn visit_predicateOperator(&mut self, ctx: &PredicateOperatorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#booleanValue}.
	 * @param ctx the parse tree
	 */
		fn visit_booleanValue(&mut self, ctx: &BooleanValueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#interval}.
	 * @param ctx the parse tree
	 */
		fn visit_interval(&mut self, ctx: &IntervalContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#errorCapturingMultiUnitsInterval}.
	 * @param ctx the parse tree
	 */
		fn visit_errorCapturingMultiUnitsInterval(&mut self, ctx: &ErrorCapturingMultiUnitsIntervalContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#multiUnitsInterval}.
	 * @param ctx the parse tree
	 */
		fn visit_multiUnitsInterval(&mut self, ctx: &MultiUnitsIntervalContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#errorCapturingUnitToUnitInterval}.
	 * @param ctx the parse tree
	 */
		fn visit_errorCapturingUnitToUnitInterval(&mut self, ctx: &ErrorCapturingUnitToUnitIntervalContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#unitToUnitInterval}.
	 * @param ctx the parse tree
	 */
		fn visit_unitToUnitInterval(&mut self, ctx: &UnitToUnitIntervalContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#intervalValue}.
	 * @param ctx the parse tree
	 */
		fn visit_intervalValue(&mut self, ctx: &IntervalValueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#unitInMultiUnits}.
	 * @param ctx the parse tree
	 */
		fn visit_unitInMultiUnits(&mut self, ctx: &UnitInMultiUnitsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#unitInUnitToUnit}.
	 * @param ctx the parse tree
	 */
		fn visit_unitInUnitToUnit(&mut self, ctx: &UnitInUnitToUnitContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#colPosition}.
	 * @param ctx the parse tree
	 */
		fn visit_colPosition(&mut self, ctx: &ColPositionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#collationSpec}.
	 * @param ctx the parse tree
	 */
		fn visit_collationSpec(&mut self, ctx: &CollationSpecContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#collateClause}.
	 * @param ctx the parse tree
	 */
		fn visit_collateClause(&mut self, ctx: &CollateClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#nonTrivialPrimitiveType}.
	 * @param ctx the parse tree
	 */
		fn visit_nonTrivialPrimitiveType(&mut self, ctx: &NonTrivialPrimitiveTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#trivialPrimitiveType}.
	 * @param ctx the parse tree
	 */
		fn visit_trivialPrimitiveType(&mut self, ctx: &TrivialPrimitiveTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#primitiveType}.
	 * @param ctx the parse tree
	 */
		fn visit_primitiveType(&mut self, ctx: &PrimitiveTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code complexDataType}
	 * labeled alternative in {@link SqlBaseParser#dataType}.
	 * @param ctx the parse tree
	 */
		fn visit_complexDataType(&mut self, ctx: &ComplexDataTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code primitiveDataType}
	 * labeled alternative in {@link SqlBaseParser#dataType}.
	 * @param ctx the parse tree
	 */
		fn visit_primitiveDataType(&mut self, ctx: &PrimitiveDataTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#qualifiedColTypeWithPositionList}.
	 * @param ctx the parse tree
	 */
		fn visit_qualifiedColTypeWithPositionList(&mut self, ctx: &QualifiedColTypeWithPositionListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#qualifiedColTypeWithPosition}.
	 * @param ctx the parse tree
	 */
		fn visit_qualifiedColTypeWithPosition(&mut self, ctx: &QualifiedColTypeWithPositionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#colDefinitionDescriptorWithPosition}.
	 * @param ctx the parse tree
	 */
		fn visit_colDefinitionDescriptorWithPosition(&mut self, ctx: &ColDefinitionDescriptorWithPositionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#defaultExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_defaultExpression(&mut self, ctx: &DefaultExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#variableDefaultExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_variableDefaultExpression(&mut self, ctx: &VariableDefaultExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#colTypeList}.
	 * @param ctx the parse tree
	 */
		fn visit_colTypeList(&mut self, ctx: &ColTypeListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#colType}.
	 * @param ctx the parse tree
	 */
		fn visit_colType(&mut self, ctx: &ColTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#tableElementList}.
	 * @param ctx the parse tree
	 */
		fn visit_tableElementList(&mut self, ctx: &TableElementListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#tableElement}.
	 * @param ctx the parse tree
	 */
		fn visit_tableElement(&mut self, ctx: &TableElementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#colDefinitionList}.
	 * @param ctx the parse tree
	 */
		fn visit_colDefinitionList(&mut self, ctx: &ColDefinitionListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#colDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_colDefinition(&mut self, ctx: &ColDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#colDefinitionOption}.
	 * @param ctx the parse tree
	 */
		fn visit_colDefinitionOption(&mut self, ctx: &ColDefinitionOptionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code generatedColumn}
	 * labeled alternative in {@link SqlBaseParser#generationExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_generatedColumn(&mut self, ctx: &GeneratedColumnContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code identityColumn}
	 * labeled alternative in {@link SqlBaseParser#generationExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_identityColumn(&mut self, ctx: &IdentityColumnContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#identityColSpec}.
	 * @param ctx the parse tree
	 */
		fn visit_identityColSpec(&mut self, ctx: &IdentityColSpecContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#sequenceGeneratorOption}.
	 * @param ctx the parse tree
	 */
		fn visit_sequenceGeneratorOption(&mut self, ctx: &SequenceGeneratorOptionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#sequenceGeneratorStartOrStep}.
	 * @param ctx the parse tree
	 */
		fn visit_sequenceGeneratorStartOrStep(&mut self, ctx: &SequenceGeneratorStartOrStepContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#complexColTypeList}.
	 * @param ctx the parse tree
	 */
		fn visit_complexColTypeList(&mut self, ctx: &ComplexColTypeListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#complexColType}.
	 * @param ctx the parse tree
	 */
		fn visit_complexColType(&mut self, ctx: &ComplexColTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#codeLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_codeLiteral(&mut self, ctx: &CodeLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#routineCharacteristics}.
	 * @param ctx the parse tree
	 */
		fn visit_routineCharacteristics(&mut self, ctx: &RoutineCharacteristicsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#routineLanguage}.
	 * @param ctx the parse tree
	 */
		fn visit_routineLanguage(&mut self, ctx: &RoutineLanguageContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#specificName}.
	 * @param ctx the parse tree
	 */
		fn visit_specificName(&mut self, ctx: &SpecificNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#deterministic}.
	 * @param ctx the parse tree
	 */
		fn visit_deterministic(&mut self, ctx: &DeterministicContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#sqlDataAccess}.
	 * @param ctx the parse tree
	 */
		fn visit_sqlDataAccess(&mut self, ctx: &SqlDataAccessContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#nullCall}.
	 * @param ctx the parse tree
	 */
		fn visit_nullCall(&mut self, ctx: &NullCallContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#rightsClause}.
	 * @param ctx the parse tree
	 */
		fn visit_rightsClause(&mut self, ctx: &RightsClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#whenClause}.
	 * @param ctx the parse tree
	 */
		fn visit_whenClause(&mut self, ctx: &WhenClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#windowClause}.
	 * @param ctx the parse tree
	 */
		fn visit_windowClause(&mut self, ctx: &WindowClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#namedWindow}.
	 * @param ctx the parse tree
	 */
		fn visit_namedWindow(&mut self, ctx: &NamedWindowContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code windowRef}
	 * labeled alternative in {@link SqlBaseParser#windowSpec}.
	 * @param ctx the parse tree
	 */
		fn visit_windowRef(&mut self, ctx: &WindowRefContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code windowDef}
	 * labeled alternative in {@link SqlBaseParser#windowSpec}.
	 * @param ctx the parse tree
	 */
		fn visit_windowDef(&mut self, ctx: &WindowDefContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#windowFrame}.
	 * @param ctx the parse tree
	 */
		fn visit_windowFrame(&mut self, ctx: &WindowFrameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#frameBound}.
	 * @param ctx the parse tree
	 */
		fn visit_frameBound(&mut self, ctx: &FrameBoundContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#qualifiedNameList}.
	 * @param ctx the parse tree
	 */
		fn visit_qualifiedNameList(&mut self, ctx: &QualifiedNameListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#functionName}.
	 * @param ctx the parse tree
	 */
		fn visit_functionName(&mut self, ctx: &FunctionNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#qualifiedName}.
	 * @param ctx the parse tree
	 */
		fn visit_qualifiedName(&mut self, ctx: &QualifiedNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#errorCapturingIdentifier}.
	 * @param ctx the parse tree
	 */
		fn visit_errorCapturingIdentifier(&mut self, ctx: &ErrorCapturingIdentifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code errorIdent}
	 * labeled alternative in {@link SqlBaseParser#errorCapturingIdentifierExtra}.
	 * @param ctx the parse tree
	 */
		fn visit_errorIdent(&mut self, ctx: &ErrorIdentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code realIdent}
	 * labeled alternative in {@link SqlBaseParser#errorCapturingIdentifierExtra}.
	 * @param ctx the parse tree
	 */
		fn visit_realIdent(&mut self, ctx: &RealIdentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#identifier}.
	 * @param ctx the parse tree
	 */
		fn visit_identifier(&mut self, ctx: &IdentifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#simpleIdentifier}.
	 * @param ctx the parse tree
	 */
		fn visit_simpleIdentifier(&mut self, ctx: &SimpleIdentifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code unquotedIdentifier}
	 * labeled alternative in {@link SqlBaseParser#strictIdentifier}.
	 * @param ctx the parse tree
	 */
		fn visit_unquotedIdentifier(&mut self, ctx: &UnquotedIdentifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code quotedIdentifierAlternative}
	 * labeled alternative in {@link SqlBaseParser#strictIdentifier}.
	 * @param ctx the parse tree
	 */
		fn visit_quotedIdentifierAlternative(&mut self, ctx: &QuotedIdentifierAlternativeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code identifierLiteral}
	 * labeled alternative in {@link SqlBaseParser#strictIdentifier}.
	 * @param ctx the parse tree
	 */
		fn visit_identifierLiteral(&mut self, ctx: &IdentifierLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code simpleUnquotedIdentifier}
	 * labeled alternative in {@link SqlBaseParser#simpleStrictIdentifier}.
	 * @param ctx the parse tree
	 */
		fn visit_simpleUnquotedIdentifier(&mut self, ctx: &SimpleUnquotedIdentifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code simpleQuotedIdentifierAlternative}
	 * labeled alternative in {@link SqlBaseParser#simpleStrictIdentifier}.
	 * @param ctx the parse tree
	 */
		fn visit_simpleQuotedIdentifierAlternative(&mut self, ctx: &SimpleQuotedIdentifierAlternativeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#quotedIdentifier}.
	 * @param ctx the parse tree
	 */
		fn visit_quotedIdentifier(&mut self, ctx: &QuotedIdentifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#backQuotedIdentifier}.
	 * @param ctx the parse tree
	 */
		fn visit_backQuotedIdentifier(&mut self, ctx: &BackQuotedIdentifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code exponentLiteral}
	 * labeled alternative in {@link SqlBaseParser#number}.
	 * @param ctx the parse tree
	 */
		fn visit_exponentLiteral(&mut self, ctx: &ExponentLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code decimalLiteral}
	 * labeled alternative in {@link SqlBaseParser#number}.
	 * @param ctx the parse tree
	 */
		fn visit_decimalLiteral(&mut self, ctx: &DecimalLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code legacyDecimalLiteral}
	 * labeled alternative in {@link SqlBaseParser#number}.
	 * @param ctx the parse tree
	 */
		fn visit_legacyDecimalLiteral(&mut self, ctx: &LegacyDecimalLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code integerLiteral}
	 * labeled alternative in {@link SqlBaseParser#number}.
	 * @param ctx the parse tree
	 */
		fn visit_integerLiteral(&mut self, ctx: &IntegerLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code bigIntLiteral}
	 * labeled alternative in {@link SqlBaseParser#number}.
	 * @param ctx the parse tree
	 */
		fn visit_bigIntLiteral(&mut self, ctx: &BigIntLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code smallIntLiteral}
	 * labeled alternative in {@link SqlBaseParser#number}.
	 * @param ctx the parse tree
	 */
		fn visit_smallIntLiteral(&mut self, ctx: &SmallIntLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code tinyIntLiteral}
	 * labeled alternative in {@link SqlBaseParser#number}.
	 * @param ctx the parse tree
	 */
		fn visit_tinyIntLiteral(&mut self, ctx: &TinyIntLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code doubleLiteral}
	 * labeled alternative in {@link SqlBaseParser#number}.
	 * @param ctx the parse tree
	 */
		fn visit_doubleLiteral(&mut self, ctx: &DoubleLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code floatLiteral}
	 * labeled alternative in {@link SqlBaseParser#number}.
	 * @param ctx the parse tree
	 */
		fn visit_floatLiteral(&mut self, ctx: &FloatLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code bigDecimalLiteral}
	 * labeled alternative in {@link SqlBaseParser#number}.
	 * @param ctx the parse tree
	 */
		fn visit_bigDecimalLiteral(&mut self, ctx: &BigDecimalLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code integerVal}
	 * labeled alternative in {@link SqlBaseParser#integerValue}.
	 * @param ctx the parse tree
	 */
		fn visit_integerVal(&mut self, ctx: &IntegerValContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code parameterIntegerValue}
	 * labeled alternative in {@link SqlBaseParser#integerValue}.
	 * @param ctx the parse tree
	 */
		fn visit_parameterIntegerValue(&mut self, ctx: &ParameterIntegerValueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#columnConstraintDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_columnConstraintDefinition(&mut self, ctx: &ColumnConstraintDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#columnConstraint}.
	 * @param ctx the parse tree
	 */
		fn visit_columnConstraint(&mut self, ctx: &ColumnConstraintContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#tableConstraintDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_tableConstraintDefinition(&mut self, ctx: &TableConstraintDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#tableConstraint}.
	 * @param ctx the parse tree
	 */
		fn visit_tableConstraint(&mut self, ctx: &TableConstraintContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#checkConstraint}.
	 * @param ctx the parse tree
	 */
		fn visit_checkConstraint(&mut self, ctx: &CheckConstraintContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#uniqueSpec}.
	 * @param ctx the parse tree
	 */
		fn visit_uniqueSpec(&mut self, ctx: &UniqueSpecContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#uniqueConstraint}.
	 * @param ctx the parse tree
	 */
		fn visit_uniqueConstraint(&mut self, ctx: &UniqueConstraintContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#referenceSpec}.
	 * @param ctx the parse tree
	 */
		fn visit_referenceSpec(&mut self, ctx: &ReferenceSpecContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#foreignKeyConstraint}.
	 * @param ctx the parse tree
	 */
		fn visit_foreignKeyConstraint(&mut self, ctx: &ForeignKeyConstraintContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#constraintCharacteristic}.
	 * @param ctx the parse tree
	 */
		fn visit_constraintCharacteristic(&mut self, ctx: &ConstraintCharacteristicContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#enforcedCharacteristic}.
	 * @param ctx the parse tree
	 */
		fn visit_enforcedCharacteristic(&mut self, ctx: &EnforcedCharacteristicContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#relyCharacteristic}.
	 * @param ctx the parse tree
	 */
		fn visit_relyCharacteristic(&mut self, ctx: &RelyCharacteristicContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#alterColumnSpecList}.
	 * @param ctx the parse tree
	 */
		fn visit_alterColumnSpecList(&mut self, ctx: &AlterColumnSpecListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#alterColumnSpec}.
	 * @param ctx the parse tree
	 */
		fn visit_alterColumnSpec(&mut self, ctx: &AlterColumnSpecContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#alterColumnAction}.
	 * @param ctx the parse tree
	 */
		fn visit_alterColumnAction(&mut self, ctx: &AlterColumnActionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code singleStringLiteralValue}
	 * labeled alternative in {@link SqlBaseParser#singleStringLitWithoutMarker}.
	 * @param ctx the parse tree
	 */
		fn visit_singleStringLiteralValue(&mut self, ctx: &SingleStringLiteralValueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code singleDoubleQuotedStringLiteralValue}
	 * labeled alternative in {@link SqlBaseParser#singleStringLitWithoutMarker}.
	 * @param ctx the parse tree
	 */
		fn visit_singleDoubleQuotedStringLiteralValue(&mut self, ctx: &SingleDoubleQuotedStringLiteralValueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#singleStringLit}.
	 * @param ctx the parse tree
	 */
		fn visit_singleStringLit(&mut self, ctx: &SingleStringLitContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code namedParameterMarkerRule}
	 * labeled alternative in {@link SqlBaseParser#parameterMarker}.
	 * @param ctx the parse tree
	 */
		fn visit_namedParameterMarkerRule(&mut self, ctx: &NamedParameterMarkerRuleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code positionalParameterMarkerRule}
	 * labeled alternative in {@link SqlBaseParser#parameterMarker}.
	 * @param ctx the parse tree
	 */
		fn visit_positionalParameterMarkerRule(&mut self, ctx: &PositionalParameterMarkerRuleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#stringLit}.
	 * @param ctx the parse tree
	 */
		fn visit_stringLit(&mut self, ctx: &StringLitContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#comment}.
	 * @param ctx the parse tree
	 */
		fn visit_comment(&mut self, ctx: &CommentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#version}.
	 * @param ctx the parse tree
	 */
		fn visit_version(&mut self, ctx: &VersionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#operatorPipeRightSide}.
	 * @param ctx the parse tree
	 */
		fn visit_operatorPipeRightSide(&mut self, ctx: &OperatorPipeRightSideContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#operatorPipeSetAssignmentSeq}.
	 * @param ctx the parse tree
	 */
		fn visit_operatorPipeSetAssignmentSeq(&mut self, ctx: &OperatorPipeSetAssignmentSeqContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#ansiNonReserved}.
	 * @param ctx the parse tree
	 */
		fn visit_ansiNonReserved(&mut self, ctx: &AnsiNonReservedContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#strictNonReserved}.
	 * @param ctx the parse tree
	 */
		fn visit_strictNonReserved(&mut self, ctx: &StrictNonReservedContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SqlBaseParser#nonReserved}.
	 * @param ctx the parse tree
	 */
		fn visit_nonReserved(&mut self, ctx: &NonReservedContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> SqlBaseParserVisitor<'input> for T
where
	T: SqlBaseParserVisitorCompat<'input>
{
	fn visit_compoundOrSingleStatement(&mut self, ctx: &CompoundOrSingleStatementContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_compoundOrSingleStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_singleCompoundStatement(&mut self, ctx: &SingleCompoundStatementContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_singleCompoundStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_beginEndCompoundBlock(&mut self, ctx: &BeginEndCompoundBlockContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_beginEndCompoundBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_compoundBody(&mut self, ctx: &CompoundBodyContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_compoundBody(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_compoundStatement(&mut self, ctx: &CompoundStatementContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_compoundStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_setVariableInsideSqlScript(&mut self, ctx: &SetVariableInsideSqlScriptContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_setVariableInsideSqlScript(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sqlStateValue(&mut self, ctx: &SqlStateValueContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_sqlStateValue(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_declareConditionStatement(&mut self, ctx: &DeclareConditionStatementContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_declareConditionStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_conditionValue(&mut self, ctx: &ConditionValueContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_conditionValue(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_conditionValues(&mut self, ctx: &ConditionValuesContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_conditionValues(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_declareHandlerStatement(&mut self, ctx: &DeclareHandlerStatementContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_declareHandlerStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_whileStatement(&mut self, ctx: &WhileStatementContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_whileStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ifElseStatement(&mut self, ctx: &IfElseStatementContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_ifElseStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_repeatStatement(&mut self, ctx: &RepeatStatementContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_repeatStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_leaveStatement(&mut self, ctx: &LeaveStatementContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_leaveStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_iterateStatement(&mut self, ctx: &IterateStatementContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_iterateStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_searchedCaseStatement(&mut self, ctx: &SearchedCaseStatementContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_searchedCaseStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_simpleCaseStatement(&mut self, ctx: &SimpleCaseStatementContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_simpleCaseStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_loopStatement(&mut self, ctx: &LoopStatementContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_loopStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_forStatement(&mut self, ctx: &ForStatementContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_forStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_singleStatement(&mut self, ctx: &SingleStatementContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_singleStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_beginLabel(&mut self, ctx: &BeginLabelContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_beginLabel(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_endLabel(&mut self, ctx: &EndLabelContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_endLabel(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_singleExpression(&mut self, ctx: &SingleExpressionContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_singleExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_singleTableIdentifier(&mut self, ctx: &SingleTableIdentifierContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_singleTableIdentifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_singleMultipartIdentifier(&mut self, ctx: &SingleMultipartIdentifierContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_singleMultipartIdentifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_singleFunctionIdentifier(&mut self, ctx: &SingleFunctionIdentifierContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_singleFunctionIdentifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_singleDataType(&mut self, ctx: &SingleDataTypeContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_singleDataType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_singleTableSchema(&mut self, ctx: &SingleTableSchemaContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_singleTableSchema(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_singleRoutineParamList(&mut self, ctx: &SingleRoutineParamListContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_singleRoutineParamList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_statementDefault(&mut self, ctx: &StatementDefaultContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_statementDefault(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_visitExecuteImmediate(&mut self, ctx: &VisitExecuteImmediateContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_visitExecuteImmediate(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dmlStatement(&mut self, ctx: &DmlStatementContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_dmlStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_use(&mut self, ctx: &UseContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_use(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_useNamespace(&mut self, ctx: &UseNamespaceContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_useNamespace(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_setCatalog(&mut self, ctx: &SetCatalogContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_setCatalog(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_createNamespace(&mut self, ctx: &CreateNamespaceContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_createNamespace(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_setNamespaceProperties(&mut self, ctx: &SetNamespacePropertiesContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_setNamespaceProperties(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unsetNamespaceProperties(&mut self, ctx: &UnsetNamespacePropertiesContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_unsetNamespaceProperties(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_setNamespaceCollation(&mut self, ctx: &SetNamespaceCollationContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_setNamespaceCollation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_setNamespaceLocation(&mut self, ctx: &SetNamespaceLocationContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_setNamespaceLocation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dropNamespace(&mut self, ctx: &DropNamespaceContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_dropNamespace(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_showNamespaces(&mut self, ctx: &ShowNamespacesContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_showNamespaces(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_createTable(&mut self, ctx: &CreateTableContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_createTable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_createTableLike(&mut self, ctx: &CreateTableLikeContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_createTableLike(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_replaceTable(&mut self, ctx: &ReplaceTableContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_replaceTable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_analyze(&mut self, ctx: &AnalyzeContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_analyze(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_analyzeTables(&mut self, ctx: &AnalyzeTablesContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_analyzeTables(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_addTableColumns(&mut self, ctx: &AddTableColumnsContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_addTableColumns(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_renameTableColumn(&mut self, ctx: &RenameTableColumnContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_renameTableColumn(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dropTableColumns(&mut self, ctx: &DropTableColumnsContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_dropTableColumns(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_renameTable(&mut self, ctx: &RenameTableContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_renameTable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_setTableProperties(&mut self, ctx: &SetTablePropertiesContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_setTableProperties(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unsetTableProperties(&mut self, ctx: &UnsetTablePropertiesContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_unsetTableProperties(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_alterTableAlterColumn(&mut self, ctx: &AlterTableAlterColumnContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_alterTableAlterColumn(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_hiveChangeColumn(&mut self, ctx: &HiveChangeColumnContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_hiveChangeColumn(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_hiveReplaceColumns(&mut self, ctx: &HiveReplaceColumnsContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_hiveReplaceColumns(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_setTableSerDe(&mut self, ctx: &SetTableSerDeContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_setTableSerDe(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_addTablePartition(&mut self, ctx: &AddTablePartitionContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_addTablePartition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_renameTablePartition(&mut self, ctx: &RenameTablePartitionContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_renameTablePartition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dropTablePartitions(&mut self, ctx: &DropTablePartitionsContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_dropTablePartitions(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_setTableLocation(&mut self, ctx: &SetTableLocationContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_setTableLocation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_recoverPartitions(&mut self, ctx: &RecoverPartitionsContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_recoverPartitions(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_alterClusterBy(&mut self, ctx: &AlterClusterByContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_alterClusterBy(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_alterTableCollation(&mut self, ctx: &AlterTableCollationContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_alterTableCollation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_addTableConstraint(&mut self, ctx: &AddTableConstraintContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_addTableConstraint(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dropTableConstraint(&mut self, ctx: &DropTableConstraintContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_dropTableConstraint(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dropTable(&mut self, ctx: &DropTableContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_dropTable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dropView(&mut self, ctx: &DropViewContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_dropView(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_createView(&mut self, ctx: &CreateViewContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_createView(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_createMetricView(&mut self, ctx: &CreateMetricViewContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_createMetricView(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_createTempViewUsing(&mut self, ctx: &CreateTempViewUsingContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_createTempViewUsing(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_alterViewQuery(&mut self, ctx: &AlterViewQueryContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_alterViewQuery(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_alterViewSchemaBinding(&mut self, ctx: &AlterViewSchemaBindingContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_alterViewSchemaBinding(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_createFunction(&mut self, ctx: &CreateFunctionContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_createFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_createUserDefinedFunction(&mut self, ctx: &CreateUserDefinedFunctionContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_createUserDefinedFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dropFunction(&mut self, ctx: &DropFunctionContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_dropFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_createVariable(&mut self, ctx: &CreateVariableContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_createVariable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dropVariable(&mut self, ctx: &DropVariableContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_dropVariable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_explain(&mut self, ctx: &ExplainContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_explain(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_showTables(&mut self, ctx: &ShowTablesContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_showTables(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_showTableExtended(&mut self, ctx: &ShowTableExtendedContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_showTableExtended(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_showTblProperties(&mut self, ctx: &ShowTblPropertiesContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_showTblProperties(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_showColumns(&mut self, ctx: &ShowColumnsContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_showColumns(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_showViews(&mut self, ctx: &ShowViewsContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_showViews(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_showPartitions(&mut self, ctx: &ShowPartitionsContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_showPartitions(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_showFunctions(&mut self, ctx: &ShowFunctionsContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_showFunctions(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_showProcedures(&mut self, ctx: &ShowProceduresContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_showProcedures(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_showCreateTable(&mut self, ctx: &ShowCreateTableContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_showCreateTable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_showCurrentNamespace(&mut self, ctx: &ShowCurrentNamespaceContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_showCurrentNamespace(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_showCatalogs(&mut self, ctx: &ShowCatalogsContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_showCatalogs(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_describeFunction(&mut self, ctx: &DescribeFunctionContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_describeFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_describeProcedure(&mut self, ctx: &DescribeProcedureContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_describeProcedure(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_describeNamespace(&mut self, ctx: &DescribeNamespaceContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_describeNamespace(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_describeRelation(&mut self, ctx: &DescribeRelationContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_describeRelation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_describeQuery(&mut self, ctx: &DescribeQueryContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_describeQuery(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_commentNamespace(&mut self, ctx: &CommentNamespaceContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_commentNamespace(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_commentTable(&mut self, ctx: &CommentTableContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_commentTable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_refreshTable(&mut self, ctx: &RefreshTableContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_refreshTable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_refreshFunction(&mut self, ctx: &RefreshFunctionContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_refreshFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_refreshResource(&mut self, ctx: &RefreshResourceContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_refreshResource(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_cacheTable(&mut self, ctx: &CacheTableContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_cacheTable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_uncacheTable(&mut self, ctx: &UncacheTableContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_uncacheTable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_clearCache(&mut self, ctx: &ClearCacheContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_clearCache(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_loadData(&mut self, ctx: &LoadDataContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_loadData(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_truncateTable(&mut self, ctx: &TruncateTableContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_truncateTable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_repairTable(&mut self, ctx: &RepairTableContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_repairTable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_manageResource(&mut self, ctx: &ManageResourceContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_manageResource(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_createIndex(&mut self, ctx: &CreateIndexContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_createIndex(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dropIndex(&mut self, ctx: &DropIndexContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_dropIndex(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_call(&mut self, ctx: &CallContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_call(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_failNativeCommand(&mut self, ctx: &FailNativeCommandContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_failNativeCommand(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_createPipelineDataset(&mut self, ctx: &CreatePipelineDatasetContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_createPipelineDataset(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_createPipelineInsertIntoFlow(&mut self, ctx: &CreatePipelineInsertIntoFlowContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_createPipelineInsertIntoFlow(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_materializedView(&mut self, ctx: &MaterializedViewContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_materializedView(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_streamingTable(&mut self, ctx: &StreamingTableContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_streamingTable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_createPipelineDatasetHeader(&mut self, ctx: &CreatePipelineDatasetHeaderContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_createPipelineDatasetHeader(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_streamTableName(&mut self, ctx: &StreamTableNameContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_streamTableName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_failSetRole(&mut self, ctx: &FailSetRoleContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_failSetRole(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_setTimeZone(&mut self, ctx: &SetTimeZoneContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_setTimeZone(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_setVariable(&mut self, ctx: &SetVariableContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_setVariable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_setQuotedConfiguration(&mut self, ctx: &SetQuotedConfigurationContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_setQuotedConfiguration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_setConfiguration(&mut self, ctx: &SetConfigurationContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_setConfiguration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_resetQuotedConfiguration(&mut self, ctx: &ResetQuotedConfigurationContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_resetQuotedConfiguration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_resetConfiguration(&mut self, ctx: &ResetConfigurationContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_resetConfiguration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_executeImmediate(&mut self, ctx: &ExecuteImmediateContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_executeImmediate(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_executeImmediateUsing(&mut self, ctx: &ExecuteImmediateUsingContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_executeImmediateUsing(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_timezone(&mut self, ctx: &TimezoneContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_timezone(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_configKey(&mut self, ctx: &ConfigKeyContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_configKey(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_configValue(&mut self, ctx: &ConfigValueContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_configValue(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unsupportedHiveNativeCommands(&mut self, ctx: &UnsupportedHiveNativeCommandsContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_unsupportedHiveNativeCommands(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_createTableHeader(&mut self, ctx: &CreateTableHeaderContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_createTableHeader(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_replaceTableHeader(&mut self, ctx: &ReplaceTableHeaderContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_replaceTableHeader(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_clusterBySpec(&mut self, ctx: &ClusterBySpecContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_clusterBySpec(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_bucketSpec(&mut self, ctx: &BucketSpecContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_bucketSpec(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_skewSpec(&mut self, ctx: &SkewSpecContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_skewSpec(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_locationSpec(&mut self, ctx: &LocationSpecContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_locationSpec(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_schemaBinding(&mut self, ctx: &SchemaBindingContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_schemaBinding(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_commentSpec(&mut self, ctx: &CommentSpecContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_commentSpec(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_singleQuery(&mut self, ctx: &SingleQueryContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_singleQuery(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_query(&mut self, ctx: &QueryContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_query(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_insertOverwriteTable(&mut self, ctx: &InsertOverwriteTableContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_insertOverwriteTable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_insertIntoTable(&mut self, ctx: &InsertIntoTableContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_insertIntoTable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_insertIntoReplaceWhere(&mut self, ctx: &InsertIntoReplaceWhereContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_insertIntoReplaceWhere(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_insertOverwriteHiveDir(&mut self, ctx: &InsertOverwriteHiveDirContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_insertOverwriteHiveDir(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_insertOverwriteDir(&mut self, ctx: &InsertOverwriteDirContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_insertOverwriteDir(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_partitionSpecLocation(&mut self, ctx: &PartitionSpecLocationContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_partitionSpecLocation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_partitionSpec(&mut self, ctx: &PartitionSpecContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_partitionSpec(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_partitionVal(&mut self, ctx: &PartitionValContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_partitionVal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_createPipelineFlowHeader(&mut self, ctx: &CreatePipelineFlowHeaderContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_createPipelineFlowHeader(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_namespace(&mut self, ctx: &NamespaceContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_namespace(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_namespaces(&mut self, ctx: &NamespacesContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_namespaces(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_variable(&mut self, ctx: &VariableContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_variable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_describeFuncName(&mut self, ctx: &DescribeFuncNameContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_describeFuncName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_describeColName(&mut self, ctx: &DescribeColNameContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_describeColName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ctes(&mut self, ctx: &CtesContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_ctes(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_namedQuery(&mut self, ctx: &NamedQueryContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_namedQuery(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tableProvider(&mut self, ctx: &TableProviderContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_tableProvider(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_createTableClauses(&mut self, ctx: &CreateTableClausesContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_createTableClauses(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_propertyList(&mut self, ctx: &PropertyListContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_propertyList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_propertyWithKeyAndEquals(&mut self, ctx: &PropertyWithKeyAndEqualsContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_propertyWithKeyAndEquals(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_propertyWithKeyNoEquals(&mut self, ctx: &PropertyWithKeyNoEqualsContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_propertyWithKeyNoEquals(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_propertyKey(&mut self, ctx: &PropertyKeyContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_propertyKey(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_propertyKeyOrStringLit(&mut self, ctx: &PropertyKeyOrStringLitContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_propertyKeyOrStringLit(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_propertyKeyOrStringLitNoCoalesce(&mut self, ctx: &PropertyKeyOrStringLitNoCoalesceContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_propertyKeyOrStringLitNoCoalesce(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_propertyValue(&mut self, ctx: &PropertyValueContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_propertyValue(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expressionPropertyList(&mut self, ctx: &ExpressionPropertyListContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_expressionPropertyList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expressionPropertyWithKeyAndEquals(&mut self, ctx: &ExpressionPropertyWithKeyAndEqualsContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_expressionPropertyWithKeyAndEquals(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expressionPropertyWithKeyNoEquals(&mut self, ctx: &ExpressionPropertyWithKeyNoEqualsContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_expressionPropertyWithKeyNoEquals(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_constantList(&mut self, ctx: &ConstantListContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_constantList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nestedConstantList(&mut self, ctx: &NestedConstantListContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_nestedConstantList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_createFileFormat(&mut self, ctx: &CreateFileFormatContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_createFileFormat(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tableFileFormat(&mut self, ctx: &TableFileFormatContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_tableFileFormat(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_genericFileFormat(&mut self, ctx: &GenericFileFormatContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_genericFileFormat(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_storageHandler(&mut self, ctx: &StorageHandlerContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_storageHandler(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_resource(&mut self, ctx: &ResourceContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_resource(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_singleInsertQuery(&mut self, ctx: &SingleInsertQueryContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_singleInsertQuery(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multiInsertQuery(&mut self, ctx: &MultiInsertQueryContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_multiInsertQuery(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_deleteFromTable(&mut self, ctx: &DeleteFromTableContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_deleteFromTable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_updateTable(&mut self, ctx: &UpdateTableContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_updateTable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mergeIntoTable(&mut self, ctx: &MergeIntoTableContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_mergeIntoTable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_identifierReference(&mut self, ctx: &IdentifierReferenceContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_identifierReference(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_catalogIdentifierReference(&mut self, ctx: &CatalogIdentifierReferenceContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_catalogIdentifierReference(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_queryOrganization(&mut self, ctx: &QueryOrganizationContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_queryOrganization(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multiInsertQueryBody(&mut self, ctx: &MultiInsertQueryBodyContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_multiInsertQueryBody(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_operatorPipeStatement(&mut self, ctx: &OperatorPipeStatementContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_operatorPipeStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_queryTermDefault(&mut self, ctx: &QueryTermDefaultContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_queryTermDefault(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_setOperation(&mut self, ctx: &SetOperationContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_setOperation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_queryPrimaryDefault(&mut self, ctx: &QueryPrimaryDefaultContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_queryPrimaryDefault(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fromStmt(&mut self, ctx: &FromStmtContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_fromStmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_table(&mut self, ctx: &TableContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_table(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inlineTableDefault1(&mut self, ctx: &InlineTableDefault1Context<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_inlineTableDefault1(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_subquery(&mut self, ctx: &SubqueryContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_subquery(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sortItem(&mut self, ctx: &SortItemContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_sortItem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fromStatement(&mut self, ctx: &FromStatementContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_fromStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fromStatementBody(&mut self, ctx: &FromStatementBodyContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_fromStatementBody(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_transformQuerySpecification(&mut self, ctx: &TransformQuerySpecificationContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_transformQuerySpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_regularQuerySpecification(&mut self, ctx: &RegularQuerySpecificationContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_regularQuerySpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_transformClause(&mut self, ctx: &TransformClauseContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_transformClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_selectClause(&mut self, ctx: &SelectClauseContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_selectClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_setClause(&mut self, ctx: &SetClauseContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_setClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_matchedClause(&mut self, ctx: &MatchedClauseContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_matchedClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_notMatchedClause(&mut self, ctx: &NotMatchedClauseContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_notMatchedClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_notMatchedBySourceClause(&mut self, ctx: &NotMatchedBySourceClauseContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_notMatchedBySourceClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_matchedAction(&mut self, ctx: &MatchedActionContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_matchedAction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_notMatchedAction(&mut self, ctx: &NotMatchedActionContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_notMatchedAction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_notMatchedBySourceAction(&mut self, ctx: &NotMatchedBySourceActionContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_notMatchedBySourceAction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_exceptClause(&mut self, ctx: &ExceptClauseContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_exceptClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_assignmentList(&mut self, ctx: &AssignmentListContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_assignmentList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_assignment(&mut self, ctx: &AssignmentContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_assignment(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_whereClause(&mut self, ctx: &WhereClauseContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_whereClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_havingClause(&mut self, ctx: &HavingClauseContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_havingClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_hint(&mut self, ctx: &HintContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_hint(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_hintStatement(&mut self, ctx: &HintStatementContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_hintStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fromClause(&mut self, ctx: &FromClauseContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_fromClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_temporalClause(&mut self, ctx: &TemporalClauseContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_temporalClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_aggregationClause(&mut self, ctx: &AggregationClauseContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_aggregationClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_groupByClause(&mut self, ctx: &GroupByClauseContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_groupByClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_groupingAnalytics(&mut self, ctx: &GroupingAnalyticsContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_groupingAnalytics(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_groupingElement(&mut self, ctx: &GroupingElementContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_groupingElement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_groupingSet(&mut self, ctx: &GroupingSetContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_groupingSet(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pivotClause(&mut self, ctx: &PivotClauseContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_pivotClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pivotColumn(&mut self, ctx: &PivotColumnContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_pivotColumn(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pivotValue(&mut self, ctx: &PivotValueContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_pivotValue(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unpivotClause(&mut self, ctx: &UnpivotClauseContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_unpivotClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unpivotNullClause(&mut self, ctx: &UnpivotNullClauseContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_unpivotNullClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unpivotOperator(&mut self, ctx: &UnpivotOperatorContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_unpivotOperator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unpivotSingleValueColumnClause(&mut self, ctx: &UnpivotSingleValueColumnClauseContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_unpivotSingleValueColumnClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unpivotMultiValueColumnClause(&mut self, ctx: &UnpivotMultiValueColumnClauseContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_unpivotMultiValueColumnClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unpivotColumnSet(&mut self, ctx: &UnpivotColumnSetContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_unpivotColumnSet(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unpivotValueColumn(&mut self, ctx: &UnpivotValueColumnContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_unpivotValueColumn(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unpivotNameColumn(&mut self, ctx: &UnpivotNameColumnContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_unpivotNameColumn(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unpivotColumnAndAlias(&mut self, ctx: &UnpivotColumnAndAliasContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_unpivotColumnAndAlias(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unpivotColumn(&mut self, ctx: &UnpivotColumnContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_unpivotColumn(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unpivotAlias(&mut self, ctx: &UnpivotAliasContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_unpivotAlias(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lateralView(&mut self, ctx: &LateralViewContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_lateralView(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_watermarkClause(&mut self, ctx: &WatermarkClauseContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_watermarkClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_setQuantifier(&mut self, ctx: &SetQuantifierContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_setQuantifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relation(&mut self, ctx: &RelationContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_relation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relationExtension(&mut self, ctx: &RelationExtensionContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_relationExtension(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_joinRelation(&mut self, ctx: &JoinRelationContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_joinRelation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_joinType(&mut self, ctx: &JoinTypeContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_joinType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_joinCriteria(&mut self, ctx: &JoinCriteriaContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_joinCriteria(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sample(&mut self, ctx: &SampleContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_sample(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sampleByPercentile(&mut self, ctx: &SampleByPercentileContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_sampleByPercentile(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sampleByRows(&mut self, ctx: &SampleByRowsContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_sampleByRows(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sampleByBucket(&mut self, ctx: &SampleByBucketContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_sampleByBucket(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sampleByBytes(&mut self, ctx: &SampleByBytesContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_sampleByBytes(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_identifierList(&mut self, ctx: &IdentifierListContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_identifierList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_identifierSeq(&mut self, ctx: &IdentifierSeqContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_identifierSeq(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_orderedIdentifierList(&mut self, ctx: &OrderedIdentifierListContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_orderedIdentifierList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_orderedIdentifier(&mut self, ctx: &OrderedIdentifierContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_orderedIdentifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_identifierCommentList(&mut self, ctx: &IdentifierCommentListContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_identifierCommentList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_identifierComment(&mut self, ctx: &IdentifierCommentContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_identifierComment(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_streamRelation(&mut self, ctx: &StreamRelationContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_streamRelation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tableName(&mut self, ctx: &TableNameContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_tableName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_aliasedQuery(&mut self, ctx: &AliasedQueryContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_aliasedQuery(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_aliasedRelation(&mut self, ctx: &AliasedRelationContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_aliasedRelation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inlineTableDefault2(&mut self, ctx: &InlineTableDefault2Context<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_inlineTableDefault2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tableValuedFunction(&mut self, ctx: &TableValuedFunctionContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_tableValuedFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_optionsClause(&mut self, ctx: &OptionsClauseContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_optionsClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inlineTable(&mut self, ctx: &InlineTableContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_inlineTable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionTableSubqueryArgument(&mut self, ctx: &FunctionTableSubqueryArgumentContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_functionTableSubqueryArgument(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tableArgumentPartitioning(&mut self, ctx: &TableArgumentPartitioningContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_tableArgumentPartitioning(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionTableNamedArgumentExpression(&mut self, ctx: &FunctionTableNamedArgumentExpressionContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_functionTableNamedArgumentExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionTableReferenceArgument(&mut self, ctx: &FunctionTableReferenceArgumentContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_functionTableReferenceArgument(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionTableArgument(&mut self, ctx: &FunctionTableArgumentContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_functionTableArgument(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionTable(&mut self, ctx: &FunctionTableContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_functionTable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tableAlias(&mut self, ctx: &TableAliasContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_tableAlias(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_rowFormatSerde(&mut self, ctx: &RowFormatSerdeContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_rowFormatSerde(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_rowFormatDelimited(&mut self, ctx: &RowFormatDelimitedContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_rowFormatDelimited(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multipartIdentifierList(&mut self, ctx: &MultipartIdentifierListContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_multipartIdentifierList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multipartIdentifier(&mut self, ctx: &MultipartIdentifierContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_multipartIdentifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multipartIdentifierPropertyList(&mut self, ctx: &MultipartIdentifierPropertyListContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_multipartIdentifierPropertyList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multipartIdentifierProperty(&mut self, ctx: &MultipartIdentifierPropertyContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_multipartIdentifierProperty(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tableIdentifier(&mut self, ctx: &TableIdentifierContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_tableIdentifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionIdentifier(&mut self, ctx: &FunctionIdentifierContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_functionIdentifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_namedExpression(&mut self, ctx: &NamedExpressionContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_namedExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_namedExpressionSeq(&mut self, ctx: &NamedExpressionSeqContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_namedExpressionSeq(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_partitionFieldList(&mut self, ctx: &PartitionFieldListContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_partitionFieldList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_partitionTransform(&mut self, ctx: &PartitionTransformContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_partitionTransform(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_partitionColumn(&mut self, ctx: &PartitionColumnContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_partitionColumn(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_identityTransform(&mut self, ctx: &IdentityTransformContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_identityTransform(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_applyTransform(&mut self, ctx: &ApplyTransformContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_applyTransform(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_transformArgument(&mut self, ctx: &TransformArgumentContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_transformArgument(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_expression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_namedArgumentExpression(&mut self, ctx: &NamedArgumentExpressionContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_namedArgumentExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionArgument(&mut self, ctx: &FunctionArgumentContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_functionArgument(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expressionSeq(&mut self, ctx: &ExpressionSeqContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_expressionSeq(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_logicalNot(&mut self, ctx: &LogicalNotContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_logicalNot(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_predicated(&mut self, ctx: &PredicatedContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_predicated(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_exists(&mut self, ctx: &ExistsContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_exists(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_logicalBinary(&mut self, ctx: &LogicalBinaryContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_logicalBinary(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_predicate(&mut self, ctx: &PredicateContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_predicate(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_errorCapturingNot(&mut self, ctx: &ErrorCapturingNotContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_errorCapturingNot(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_valueExpressionDefault(&mut self, ctx: &ValueExpressionDefaultContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_valueExpressionDefault(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_comparison(&mut self, ctx: &ComparisonContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_comparison(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_shiftExpression(&mut self, ctx: &ShiftExpressionContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_shiftExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_arithmeticBinary(&mut self, ctx: &ArithmeticBinaryContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_arithmeticBinary(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_arithmeticUnary(&mut self, ctx: &ArithmeticUnaryContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_arithmeticUnary(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_shiftOperator(&mut self, ctx: &ShiftOperatorContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_shiftOperator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_datetimeUnit(&mut self, ctx: &DatetimeUnitContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_datetimeUnit(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_struct(&mut self, ctx: &StructContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_struct(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dereference(&mut self, ctx: &DereferenceContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_dereference(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_castByColon(&mut self, ctx: &CastByColonContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_castByColon(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_timestampadd(&mut self, ctx: &TimestampaddContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_timestampadd(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_substring(&mut self, ctx: &SubstringContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_substring(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_cast(&mut self, ctx: &CastContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_cast(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lambda(&mut self, ctx: &LambdaContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_lambda(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parenthesizedExpression(&mut self, ctx: &ParenthesizedExpressionContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_parenthesizedExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_any_value(&mut self, ctx: &Any_valueContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_any_value(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_trim(&mut self, ctx: &TrimContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_trim(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_semiStructuredExtract(&mut self, ctx: &SemiStructuredExtractContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_semiStructuredExtract(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_simpleCase(&mut self, ctx: &SimpleCaseContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_simpleCase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_currentLike(&mut self, ctx: &CurrentLikeContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_currentLike(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_columnReference(&mut self, ctx: &ColumnReferenceContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_columnReference(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_rowConstructor(&mut self, ctx: &RowConstructorContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_rowConstructor(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_last(&mut self, ctx: &LastContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_last(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_star(&mut self, ctx: &StarContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_star(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_overlay(&mut self, ctx: &OverlayContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_overlay(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_subscript(&mut self, ctx: &SubscriptContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_subscript(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_timestampdiff(&mut self, ctx: &TimestampdiffContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_timestampdiff(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_subqueryExpression(&mut self, ctx: &SubqueryExpressionContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_subqueryExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_collate(&mut self, ctx: &CollateContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_collate(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_constantDefault(&mut self, ctx: &ConstantDefaultContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_constantDefault(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_extract(&mut self, ctx: &ExtractContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_extract(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionCall(&mut self, ctx: &FunctionCallContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_functionCall(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_searchedCase(&mut self, ctx: &SearchedCaseContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_searchedCase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_position(&mut self, ctx: &PositionContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_position(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_first(&mut self, ctx: &FirstContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_first(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_semiStructuredExtractionPath(&mut self, ctx: &SemiStructuredExtractionPathContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_semiStructuredExtractionPath(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_jsonPathIdentifier(&mut self, ctx: &JsonPathIdentifierContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_jsonPathIdentifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_jsonPathBracketedIdentifier(&mut self, ctx: &JsonPathBracketedIdentifierContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_jsonPathBracketedIdentifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_jsonPathFirstPart(&mut self, ctx: &JsonPathFirstPartContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_jsonPathFirstPart(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_jsonPathParts(&mut self, ctx: &JsonPathPartsContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_jsonPathParts(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_literalType(&mut self, ctx: &LiteralTypeContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_literalType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nullLiteral(&mut self, ctx: &NullLiteralContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_nullLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_posParameterLiteral(&mut self, ctx: &PosParameterLiteralContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_posParameterLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_namedParameterLiteral(&mut self, ctx: &NamedParameterLiteralContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_namedParameterLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_intervalLiteral(&mut self, ctx: &IntervalLiteralContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_intervalLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_typeConstructor(&mut self, ctx: &TypeConstructorContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_typeConstructor(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_numericLiteral(&mut self, ctx: &NumericLiteralContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_numericLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_booleanLiteral(&mut self, ctx: &BooleanLiteralContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_booleanLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stringLiteral(&mut self, ctx: &StringLiteralContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_stringLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_namedParameterMarker(&mut self, ctx: &NamedParameterMarkerContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_namedParameterMarker(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_comparisonOperator(&mut self, ctx: &ComparisonOperatorContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_comparisonOperator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_arithmeticOperator(&mut self, ctx: &ArithmeticOperatorContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_arithmeticOperator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_predicateOperator(&mut self, ctx: &PredicateOperatorContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_predicateOperator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_booleanValue(&mut self, ctx: &BooleanValueContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_booleanValue(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_interval(&mut self, ctx: &IntervalContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_interval(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_errorCapturingMultiUnitsInterval(&mut self, ctx: &ErrorCapturingMultiUnitsIntervalContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_errorCapturingMultiUnitsInterval(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multiUnitsInterval(&mut self, ctx: &MultiUnitsIntervalContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_multiUnitsInterval(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_errorCapturingUnitToUnitInterval(&mut self, ctx: &ErrorCapturingUnitToUnitIntervalContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_errorCapturingUnitToUnitInterval(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unitToUnitInterval(&mut self, ctx: &UnitToUnitIntervalContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_unitToUnitInterval(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_intervalValue(&mut self, ctx: &IntervalValueContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_intervalValue(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unitInMultiUnits(&mut self, ctx: &UnitInMultiUnitsContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_unitInMultiUnits(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unitInUnitToUnit(&mut self, ctx: &UnitInUnitToUnitContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_unitInUnitToUnit(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_colPosition(&mut self, ctx: &ColPositionContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_colPosition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_collationSpec(&mut self, ctx: &CollationSpecContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_collationSpec(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_collateClause(&mut self, ctx: &CollateClauseContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_collateClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nonTrivialPrimitiveType(&mut self, ctx: &NonTrivialPrimitiveTypeContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_nonTrivialPrimitiveType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_trivialPrimitiveType(&mut self, ctx: &TrivialPrimitiveTypeContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_trivialPrimitiveType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_primitiveType(&mut self, ctx: &PrimitiveTypeContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_primitiveType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_complexDataType(&mut self, ctx: &ComplexDataTypeContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_complexDataType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_primitiveDataType(&mut self, ctx: &PrimitiveDataTypeContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_primitiveDataType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_qualifiedColTypeWithPositionList(&mut self, ctx: &QualifiedColTypeWithPositionListContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_qualifiedColTypeWithPositionList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_qualifiedColTypeWithPosition(&mut self, ctx: &QualifiedColTypeWithPositionContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_qualifiedColTypeWithPosition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_colDefinitionDescriptorWithPosition(&mut self, ctx: &ColDefinitionDescriptorWithPositionContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_colDefinitionDescriptorWithPosition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_defaultExpression(&mut self, ctx: &DefaultExpressionContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_defaultExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_variableDefaultExpression(&mut self, ctx: &VariableDefaultExpressionContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_variableDefaultExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_colTypeList(&mut self, ctx: &ColTypeListContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_colTypeList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_colType(&mut self, ctx: &ColTypeContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_colType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tableElementList(&mut self, ctx: &TableElementListContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_tableElementList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tableElement(&mut self, ctx: &TableElementContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_tableElement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_colDefinitionList(&mut self, ctx: &ColDefinitionListContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_colDefinitionList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_colDefinition(&mut self, ctx: &ColDefinitionContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_colDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_colDefinitionOption(&mut self, ctx: &ColDefinitionOptionContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_colDefinitionOption(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_generatedColumn(&mut self, ctx: &GeneratedColumnContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_generatedColumn(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_identityColumn(&mut self, ctx: &IdentityColumnContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_identityColumn(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_identityColSpec(&mut self, ctx: &IdentityColSpecContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_identityColSpec(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sequenceGeneratorOption(&mut self, ctx: &SequenceGeneratorOptionContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_sequenceGeneratorOption(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sequenceGeneratorStartOrStep(&mut self, ctx: &SequenceGeneratorStartOrStepContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_sequenceGeneratorStartOrStep(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_complexColTypeList(&mut self, ctx: &ComplexColTypeListContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_complexColTypeList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_complexColType(&mut self, ctx: &ComplexColTypeContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_complexColType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_codeLiteral(&mut self, ctx: &CodeLiteralContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_codeLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_routineCharacteristics(&mut self, ctx: &RoutineCharacteristicsContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_routineCharacteristics(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_routineLanguage(&mut self, ctx: &RoutineLanguageContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_routineLanguage(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_specificName(&mut self, ctx: &SpecificNameContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_specificName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_deterministic(&mut self, ctx: &DeterministicContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_deterministic(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sqlDataAccess(&mut self, ctx: &SqlDataAccessContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_sqlDataAccess(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nullCall(&mut self, ctx: &NullCallContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_nullCall(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_rightsClause(&mut self, ctx: &RightsClauseContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_rightsClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_whenClause(&mut self, ctx: &WhenClauseContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_whenClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_windowClause(&mut self, ctx: &WindowClauseContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_windowClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_namedWindow(&mut self, ctx: &NamedWindowContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_namedWindow(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_windowRef(&mut self, ctx: &WindowRefContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_windowRef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_windowDef(&mut self, ctx: &WindowDefContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_windowDef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_windowFrame(&mut self, ctx: &WindowFrameContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_windowFrame(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_frameBound(&mut self, ctx: &FrameBoundContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_frameBound(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_qualifiedNameList(&mut self, ctx: &QualifiedNameListContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_qualifiedNameList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionName(&mut self, ctx: &FunctionNameContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_functionName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_qualifiedName(&mut self, ctx: &QualifiedNameContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_qualifiedName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_errorCapturingIdentifier(&mut self, ctx: &ErrorCapturingIdentifierContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_errorCapturingIdentifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_errorIdent(&mut self, ctx: &ErrorIdentContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_errorIdent(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_realIdent(&mut self, ctx: &RealIdentContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_realIdent(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_identifier(&mut self, ctx: &IdentifierContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_identifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_simpleIdentifier(&mut self, ctx: &SimpleIdentifierContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_simpleIdentifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unquotedIdentifier(&mut self, ctx: &UnquotedIdentifierContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_unquotedIdentifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_quotedIdentifierAlternative(&mut self, ctx: &QuotedIdentifierAlternativeContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_quotedIdentifierAlternative(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_identifierLiteral(&mut self, ctx: &IdentifierLiteralContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_identifierLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_simpleUnquotedIdentifier(&mut self, ctx: &SimpleUnquotedIdentifierContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_simpleUnquotedIdentifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_simpleQuotedIdentifierAlternative(&mut self, ctx: &SimpleQuotedIdentifierAlternativeContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_simpleQuotedIdentifierAlternative(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_quotedIdentifier(&mut self, ctx: &QuotedIdentifierContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_quotedIdentifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_backQuotedIdentifier(&mut self, ctx: &BackQuotedIdentifierContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_backQuotedIdentifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_exponentLiteral(&mut self, ctx: &ExponentLiteralContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_exponentLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_decimalLiteral(&mut self, ctx: &DecimalLiteralContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_decimalLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_legacyDecimalLiteral(&mut self, ctx: &LegacyDecimalLiteralContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_legacyDecimalLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_integerLiteral(&mut self, ctx: &IntegerLiteralContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_integerLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_bigIntLiteral(&mut self, ctx: &BigIntLiteralContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_bigIntLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_smallIntLiteral(&mut self, ctx: &SmallIntLiteralContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_smallIntLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tinyIntLiteral(&mut self, ctx: &TinyIntLiteralContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_tinyIntLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_doubleLiteral(&mut self, ctx: &DoubleLiteralContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_doubleLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_floatLiteral(&mut self, ctx: &FloatLiteralContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_floatLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_bigDecimalLiteral(&mut self, ctx: &BigDecimalLiteralContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_bigDecimalLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_integerVal(&mut self, ctx: &IntegerValContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_integerVal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parameterIntegerValue(&mut self, ctx: &ParameterIntegerValueContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_parameterIntegerValue(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_columnConstraintDefinition(&mut self, ctx: &ColumnConstraintDefinitionContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_columnConstraintDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_columnConstraint(&mut self, ctx: &ColumnConstraintContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_columnConstraint(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tableConstraintDefinition(&mut self, ctx: &TableConstraintDefinitionContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_tableConstraintDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tableConstraint(&mut self, ctx: &TableConstraintContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_tableConstraint(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_checkConstraint(&mut self, ctx: &CheckConstraintContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_checkConstraint(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_uniqueSpec(&mut self, ctx: &UniqueSpecContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_uniqueSpec(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_uniqueConstraint(&mut self, ctx: &UniqueConstraintContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_uniqueConstraint(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_referenceSpec(&mut self, ctx: &ReferenceSpecContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_referenceSpec(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_foreignKeyConstraint(&mut self, ctx: &ForeignKeyConstraintContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_foreignKeyConstraint(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_constraintCharacteristic(&mut self, ctx: &ConstraintCharacteristicContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_constraintCharacteristic(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enforcedCharacteristic(&mut self, ctx: &EnforcedCharacteristicContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_enforcedCharacteristic(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relyCharacteristic(&mut self, ctx: &RelyCharacteristicContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_relyCharacteristic(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_alterColumnSpecList(&mut self, ctx: &AlterColumnSpecListContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_alterColumnSpecList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_alterColumnSpec(&mut self, ctx: &AlterColumnSpecContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_alterColumnSpec(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_alterColumnAction(&mut self, ctx: &AlterColumnActionContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_alterColumnAction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_singleStringLiteralValue(&mut self, ctx: &SingleStringLiteralValueContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_singleStringLiteralValue(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_singleDoubleQuotedStringLiteralValue(&mut self, ctx: &SingleDoubleQuotedStringLiteralValueContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_singleDoubleQuotedStringLiteralValue(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_singleStringLit(&mut self, ctx: &SingleStringLitContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_singleStringLit(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_namedParameterMarkerRule(&mut self, ctx: &NamedParameterMarkerRuleContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_namedParameterMarkerRule(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_positionalParameterMarkerRule(&mut self, ctx: &PositionalParameterMarkerRuleContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_positionalParameterMarkerRule(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stringLit(&mut self, ctx: &StringLitContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_stringLit(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_comment(&mut self, ctx: &CommentContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_comment(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_version(&mut self, ctx: &VersionContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_version(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_operatorPipeRightSide(&mut self, ctx: &OperatorPipeRightSideContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_operatorPipeRightSide(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_operatorPipeSetAssignmentSeq(&mut self, ctx: &OperatorPipeSetAssignmentSeqContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_operatorPipeSetAssignmentSeq(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ansiNonReserved(&mut self, ctx: &AnsiNonReservedContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_ansiNonReserved(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_strictNonReserved(&mut self, ctx: &StrictNonReservedContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_strictNonReserved(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nonReserved(&mut self, ctx: &NonReservedContext<'input>){
		let result = <Self as SqlBaseParserVisitorCompat>::visit_nonReserved(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}