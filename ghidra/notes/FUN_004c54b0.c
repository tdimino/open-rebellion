
void __thiscall FUN_004c54b0(void *this,void *param_1,void *param_2,undefined4 param_3)

{
  int *piVar1;
  void *pvVar2;
  uint local_18;
  int iStack_14;
  int iStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063a8b8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004ece30(&local_18);
  local_4 = 0;
  FUN_005f4d90(param_1,(int)this + 8);
  (**(code **)(*(int *)this + 4))();
  FUN_005f4d90(param_1,&iStack_14);
  if (iStack_14 != 0) {
    do {
      FUN_004c5880(this,param_1,param_2,param_3);
      iStack_14 = iStack_14 + -1;
    } while (iStack_14 != 0);
    iStack_14 = 0;
  }
  FUN_005f4d90(param_1,&iStack_10);
  if (iStack_10 == 0) {
    *(undefined4 *)((int)this + 0x10) = 0;
  }
  else {
    FUN_004ece90(&local_18,param_1);
    piVar1 = FUN_004f3970(*(int *)((int)this + 8),&local_18);
    *(int **)((int)this + 0x10) = piVar1;
  }
  FUN_005f4d90(param_1,&iStack_10);
  if (iStack_10 == 0) {
    *(undefined4 *)((int)this + 0xc) = 0;
  }
  else {
    FUN_004ece90(&local_18,param_1);
    FUN_004f26d0(&stack0xffffffd4,&local_18);
    pvVar2 = FUN_0049dca0(this);
    *(void **)((int)this + 0xc) = pvVar2;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return;
}

