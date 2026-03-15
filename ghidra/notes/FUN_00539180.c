
int __thiscall FUN_00539180(void *this,void *param_1)

{
  int iVar1;
  int iVar2;
  uint uVar3;
  undefined4 local_24;
  undefined4 local_20;
  undefined4 local_1c [4];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00646058;
  pvStack_c = ExceptionList;
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
  iVar1 = FUN_005202d0(this,param_1);
  uVar3 = 0;
  if (iVar1 != 0) {
    if (*(int *)((int)param_1 + 4) == -1) {
                    /* WARNING: Load size is inaccurate */
      iVar2 = (**(code **)(*this + 0x14))();
      uVar3 = (uint)(iVar2 == 0);
    }
    if (iVar1 != 0) {
      if (*(int *)((int)param_1 + 4) == -1) {
        iVar2 = FUN_0041c210(*(int *)((int)this + 0x40));
        while ((iVar2 != 0 && (*(int *)((int)param_1 + 4) == -1))) {
          *(uint *)(iVar2 + 0x50) = uVar3;
          iVar2 = *(int *)(iVar2 + 0x10);
        }
      }
      if (iVar1 != 0) goto LAB_00539266;
    }
  }
  local_24 = 1;
  local_20 = 1;
  local_4._0_1_ = 2;
  FUN_00520580(param_1,&local_24);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_00619730();
LAB_00539266:
  local_4 = 0xffffffff;
  FUN_00541ea0();
  ExceptionList = pvStack_c;
  return iVar1;
}

