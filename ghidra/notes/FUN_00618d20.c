
void FUN_00618d20(undefined4 param_1,undefined4 param_2,int param_3,undefined *param_4)

{
  void *local_14;
  code *pcStack_10;
  undefined *puStack_c;
  undefined4 local_8;
  
  puStack_c = &DAT_0066e638;
  pcStack_10 = FUN_0061a414;
  local_14 = ExceptionList;
  local_8 = 0;
  ExceptionList = &local_14;
  while( true ) {
    param_3 = param_3 + -1;
    if (param_3 < 0) break;
    (*(code *)param_4)();
  }
  local_8 = 0xffffffff;
  FUN_00618d99();
  ExceptionList = local_14;
  return;
}

