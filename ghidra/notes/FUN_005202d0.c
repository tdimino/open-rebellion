
int __thiscall FUN_005202d0(void *this,void *param_1)

{
  int iVar1;
  undefined4 local_24;
  undefined4 local_20;
  undefined4 local_1c [4];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006434c8;
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
  if (iVar1 == 0) {
    local_24 = 1;
    local_20 = 1;
    local_4._0_1_ = 2;
    FUN_00520580(param_1,&local_24);
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00619730();
  }
  local_4 = 0xffffffff;
  FUN_00541ea0();
  ExceptionList = pvStack_c;
  return iVar1;
}

