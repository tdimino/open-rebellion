
undefined4 __thiscall FUN_00520040(void *this,void *param_1)

{
  int iVar1;
  int iVar2;
  undefined4 uVar3;
  undefined4 local_1c;
  undefined4 local_18;
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00643470;
  pvStack_c = ExceptionList;
  iVar1 = *(int *)((int)this + 0x1c);
  local_1c = 1;
  local_18 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_1,&local_1c);
  local_4 = 0xffffffff;
  FUN_00619730();
  (**(code **)(*(int *)this + 0x1c))(param_1);
  if (iVar1 != 0) {
    if (*(int *)((int)param_1 + 4) == -1) {
      iVar2 = *(int *)this;
      uVar3 = FUN_004fd2f0();
      (**(code **)(iVar2 + 0x44))(uVar3);
    }
    if ((iVar1 != 0) && (*(int *)((int)param_1 + 4) == -1)) {
      FUN_00541e70((undefined4 *)&stack0xffffffe0);
      puStack_8 = (undefined1 *)0x1;
      FUN_0051fcb0(this,(undefined4 *)&stack0xffffffe0);
      FUN_005539f0((int)&stack0xffffffe0,*(void **)((int)this + 0x40));
      puStack_8 = (undefined1 *)0xffffffff;
      FUN_00541ea0();
    }
  }
  ExceptionList = pvStack_10;
  return 1;
}

