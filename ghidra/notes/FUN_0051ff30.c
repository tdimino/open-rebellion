
bool __thiscall FUN_0051ff30(void *this,void *param_1)

{
  int iVar1;
  undefined4 local_24;
  undefined4 local_20;
  undefined4 local_1c [3];
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00643448;
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
  (**(code **)(*(int *)this + 0x18))(param_1);
  if (iVar1 != 0) {
    if (*(int *)((int)param_1 + 4) == -1) {
      iVar1 = (**(code **)(*(int *)this + 0x50))(param_1);
    }
    if (iVar1 != 0) {
      if (*(int *)((int)param_1 + 4) == -1) {
        iVar1 = FUN_00553960(&local_20,*(int *)((int)this + 0x40),param_1);
      }
      if (iVar1 != 0) goto LAB_0052000a;
    }
  }
  local_24 = 1;
  puStack_8._0_1_ = 2;
  FUN_00520580(param_1,(undefined4 *)&stack0xffffffd8);
  puStack_8 = (undefined1 *)CONCAT31(puStack_8._1_3_,1);
  FUN_00619730();
LAB_0052000a:
  iVar1 = *(int *)((int)param_1 + 4);
  puStack_8 = (undefined1 *)0xffffffff;
  FUN_00541ea0();
  ExceptionList = pvStack_10;
  return iVar1 == -1;
}

