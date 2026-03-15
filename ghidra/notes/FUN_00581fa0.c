
bool __thiscall FUN_00581fa0(void *this,void *param_1)

{
  int iVar1;
  undefined4 local_24;
  undefined4 local_20;
  undefined4 local_1c [4];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0064ee08;
  pvStack_c = ExceptionList;
  iVar1 = *(int *)((int)this + 0x1c);
  local_24 = 1;
  local_20 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_1,&local_24);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_00541e70(local_1c);
  local_4 = 1;
  FUN_0051fcb0(this,local_1c);
  FUN_0051ff30(this,param_1);
  if (iVar1 != 0) {
    if (*(int *)((int)param_1 + 4) == -1) {
      iVar1 = FUN_00553960(local_1c,*(int *)((int)this + 0x44),param_1);
    }
    if (iVar1 != 0) goto LAB_00582067;
  }
  local_24 = 1;
  local_20 = 1;
  local_4._0_1_ = 2;
  FUN_00520580(param_1,&local_24);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_00619730();
LAB_00582067:
  iVar1 = *(int *)((int)param_1 + 4);
  local_4 = 0xffffffff;
  FUN_00541ea0();
  ExceptionList = pvStack_c;
  return iVar1 == -1;
}

