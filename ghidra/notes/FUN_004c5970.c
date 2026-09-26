
void __thiscall FUN_004c5970(void *this,void *param_1)

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
  puStack_8 = &LAB_0063a948;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004ece30(&local_18);
  local_4 = 0;
  FUN_005f4d90(param_1,(int *)((int)this + 8));
  (**(code **)(*(int *)this + 4))();
  FUN_005f4d90(param_1,&iStack_14);
  for (; iStack_14 != 0; iStack_14 = iStack_14 + -1) {
    FUN_004c5d50(this,param_1);
  }
  FUN_005f4d90(param_1,&iStack_10);
  if (iStack_10 == 0) {
    *(undefined4 *)((int)this + 0x10) = 0;
  }
  else {
    FUN_004ece90(&local_18,param_1);
    piVar1 = FUN_004f3220(*(int *)((int)this + 8),&local_18);
    *(int **)((int)this + 0x10) = piVar1;
  }
  FUN_005f4d90(param_1,&iStack_10);
  if (iStack_10 == 0) {
    *(undefined4 *)((int)this + 0xc) = 0;
  }
  else {
    FUN_004ece90(&local_18,param_1);
    FUN_004f26d0(&stack0xffffffd8,&local_18);
    pvVar2 = FUN_004c5dd0(this);
    *(void **)((int)this + 0xc) = pvVar2;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return;
}

