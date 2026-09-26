
bool __thiscall FUN_0053d100(void *this,int *param_1)

{
  void *pvVar1;
  int iVar2;
  int *piVar3;
  undefined4 local_24;
  undefined4 local_20;
  undefined4 local_1c [4];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00646618;
  pvStack_c = ExceptionList;
  iVar2 = *(int *)((int)this + 0x1c);
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
  if (iVar2 != 0) {
    if (param_1[1] != -1) {
      piVar3 = param_1;
      pvVar1 = (void *)(**(code **)(*(int *)this + 0x28))();
      iVar2 = FUN_00553aa0((int)local_1c,pvVar1,piVar3);
    }
    if (iVar2 != 0) goto LAB_0053d1cb;
  }
  local_24 = 1;
  local_20 = 1;
  local_4._0_1_ = 2;
  FUN_00520580(param_1,&local_24);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_00619730();
LAB_0053d1cb:
  iVar2 = param_1[1];
  local_4 = 0xffffffff;
  FUN_00541ea0();
  ExceptionList = pvStack_c;
  return iVar2 == -1;
}

