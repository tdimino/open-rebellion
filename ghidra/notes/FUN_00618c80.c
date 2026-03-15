
void FUN_00618c80(undefined4 param_1,undefined4 param_2,int param_3,undefined *param_4)

{
  int iVar1;
  void *local_14;
  code *pcStack_10;
  undefined *puStack_c;
  undefined4 local_8;
  
  puStack_c = &DAT_0066e628;
  pcStack_10 = FUN_0061a414;
  local_14 = ExceptionList;
  local_8 = 0;
  ExceptionList = &local_14;
  for (iVar1 = 0; iVar1 < param_3; iVar1 = iVar1 + 1) {
    (*(code *)param_4)();
  }
  local_8 = 0xffffffff;
  FUN_00618cf8();
  ExceptionList = local_14;
  return;
}

